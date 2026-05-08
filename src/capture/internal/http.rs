use std::io::Read;
use std::sync::mpsc::{Receiver, SyncSender, TrySendError, sync_channel};
use std::thread;

use image::GrayImage;

use crate::capture::internal::Camera;
use crate::capture::{CameraError, discovery::HttpSource};

const SOI: [u8; 2] = [0xFF, 0xD8];
const EOI: [u8; 2] = [0xFF, 0xD9];
const READ_CHUNK: usize = 4096;
const MAX_FRAME: usize = 1024 * 1024;

pub struct HttpCamera {
    rx: Receiver<GrayImage>,
}

impl Camera for HttpCamera {
    fn read_frame(&mut self) -> Result<GrayImage, CameraError> {
        self.rx
            .recv()
            .map_err(|_| CameraError::Internal("MJPEG worker thread terminated".into()))
    }
}

impl HttpCamera {
    pub fn open(source: &HttpSource) -> Result<Self, CameraError> {
        let (tx, rx) = sync_channel::<GrayImage>(1);
        let url = source.url.clone();

        thread::Builder::new()
            .name("http-mjpeg-capture".into())
            .spawn(move || run(&url, &tx))
            .map_err(|e| CameraError::Internal(format!("failed to spawn worker: {e}")))?;

        Ok(Self { rx })
    }
}

fn run(url: &str, tx: &SyncSender<GrayImage>) {
    let mut body = match ureq::get(url).call() {
        Ok(r) => r.into_body(),
        Err(e) => {
            tracing::warn!(target: "snout::capture::http", "HTTP request failed: {e}");
            return;
        }
    };

    for frame in JpegFrames::new(body.as_reader()) {
        let frame = match frame {
            Ok(f) => f,
            Err(e) => {
                tracing::warn!(target: "snout::capture::http", "stream read error: {e}");
                return;
            }
        };

        let Ok(img) = image::load_from_memory(&frame) else {
            continue; // skip malformed frames
        };

        if let Err(TrySendError::Disconnected(_)) = tx.try_send(img.into_luma8()) {
            return; // consumer gone
        }
        // TrySendError::Full: drop this frame, keep draining the socket
        // so the server's send buffer doesn't back up.
    }
}

/// Iterator that pulls JPEG frames out of any byte stream by scanning
/// for SOI/EOI markers. Yields the raw JPEG bytes including markers.
struct JpegFrames<R: Read> {
    reader: R,
    buf: Vec<u8>,
    chunk: [u8; READ_CHUNK],
    eof: bool,
}

impl<R: Read> JpegFrames<R> {
    fn new(reader: R) -> Self {
        Self {
            reader,
            buf: Vec::with_capacity(MAX_FRAME),
            chunk: [0; READ_CHUNK],
            eof: false,
        }
    }

    /// Try to slice off the next complete JPEG from the front of `buf`,
    /// discarding any preceding garbage.
    fn extract_frame(&mut self) -> Option<Vec<u8>> {
        let start = find(&self.buf, &SOI)?;
        let end = find(&self.buf[start + 2..], &EOI)? + start + 2;
        let frame = self.buf[start..end + 2].to_vec();
        self.buf.drain(..end + 2);
        Some(frame)
    }

    /// Read one chunk into `buf`. Returns `Ok(true)` on EOF.
    fn fill(&mut self) -> std::io::Result<bool> {
        loop {
            return match self.reader.read(&mut self.chunk) {
                Ok(0) => Ok(true),
                Ok(n) => {
                    self.buf.extend_from_slice(&self.chunk[..n]);
                    if self.buf.len() > MAX_FRAME {
                        // Garbage between frames is unbounded; reset.
                        self.buf.clear();
                    }
                    Ok(false)
                }
                Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
                Err(e) => Err(e),
            };
        }
    }
}

impl<R: Read> Iterator for JpegFrames<R> {
    type Item = std::io::Result<Vec<u8>>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(frame) = self.extract_frame() {
                return Some(Ok(frame));
            }
            if self.eof {
                return None;
            }
            match self.fill() {
                Ok(true) => self.eof = true,
                Ok(false) => {}
                Err(e) => return Some(Err(e)),
            }
        }
    }
}

fn find(haystack: &[u8], needle: &[u8; 2]) -> Option<usize> {
    haystack.windows(2).position(|w| w == needle)
}
