use std::io;
use std::path::Path;
use std::time::Duration;

use crate::capture::{
    CameraError, StereoCamera,
    discovery::{CameraInfo, CameraSource},
    processing::FramePreprocessor,
};
use crate::config::{Config, OverlayMode};
use crate::sample::collector::{FrameCollector, Phase};
use crate::sample::net::{Event, Mode, Overlay, Routine};

#[derive(Debug, thiserror::Error)]
pub enum SamplerError {
    #[error("no sample config in config file")]
    NoConfig,
    #[error("overlay error: {0}")]
    Overlay(#[from] io::Error),
    #[error("camera error: {0}")]
    Camera(String),
}

pub struct LongSampler {
    overlay_path: String,
    overlay_mode: Mode,
    left_preprocessor: FramePreprocessor,
    right_preprocessor: FramePreprocessor,
    camera: Option<StereoCamera>,
    left_source: Option<CameraSource>,
    right_source: Option<CameraSource>,
}

impl LongSampler {
    pub fn with_config(cameras: &[CameraInfo], config: &Config) -> Result<Self, SamplerError> {
        let sample_config = config.sample.as_ref().ok_or(SamplerError::NoConfig)?;

        let overlay_mode = match sample_config.overlay.mode {
            OverlayMode::OpenVr => Mode::OpenVr,
            OverlayMode::OpenXr => Mode::OpenXr,
            OverlayMode::Debug => Mode::Debug,
        };

        let left_source = cameras
            .iter()
            .find(|s| s.display_name() == config.eye.left.camera)
            .map(|c| c.source.clone());

        let right_source = cameras
            .iter()
            .find(|s| s.display_name() == config.eye.right.camera)
            .map(|c| c.source.clone());

        let mut left_preprocessor = FramePreprocessor::new();
        left_preprocessor.set_crop(config.eye.left.crop);
        if let Some(transform) = &config.eye.left.transform {
            left_preprocessor.set_config(*transform);
        }

        let mut right_preprocessor = FramePreprocessor::new();
        right_preprocessor.set_crop(config.eye.right.crop);
        if let Some(transform) = &config.eye.right.transform {
            right_preprocessor.set_config(*transform);
        }

        Ok(Self {
            overlay_path: sample_config.overlay.path.to_string_lossy().into_owned(),
            overlay_mode,
            left_preprocessor,
            right_preprocessor,
            camera: None,
            left_source,
            right_source,
        })
    }

    pub fn run(&mut self, output: impl AsRef<Path>) -> Result<(), SamplerError> {
        self.ensure_camera()?;

        let mut overlay = Overlay::start(&self.overlay_path, self.overlay_mode)?;
        let mut collector = FrameCollector::new();

        // Gaze tutorial
        overlay.begin(Routine::GazeTutorial)?;
        self.wait_for_finish(&mut overlay)?;

        // Gaze capture
        overlay.begin(Routine::Gaze(Duration::from_secs(60)))?;
        self.collect(&mut overlay, &mut collector, Phase::Gaze)?;

        // Blink tutorial
        overlay.begin(Routine::BlinkTutorial)?;
        self.wait_for_finish(&mut overlay)?;

        // Blink capture
        overlay.begin(Routine::Blink(Duration::from_secs(10)))?;
        self.collect(&mut overlay, &mut collector, Phase::Blink)?;

        // Write all frames
        collector.write(output)?;

        overlay.close()?;

        Ok(())
    }

    fn collect(
        &mut self,
        overlay: &mut Overlay,
        collector: &mut FrameCollector,
        phase: Phase,
    ) -> Result<(), SamplerError> {
        loop {
            match overlay.try_recv()? {
                Some(Event::Position(pos)) => {
                    collector.set_position(pos);
                }
                Some(Event::Finished) => return Ok(()),
                None => {}
            }

            if let Some((left, right)) = self.grab_frame()? {
                collector.add(phase, &left, &right);
            }
        }
    }

    fn wait_for_finish(&mut self, overlay: &mut Overlay) -> Result<(), SamplerError> {
        loop {
            match overlay.try_recv()? {
                Some(Event::Finished) => return Ok(()),
                _ => std::thread::sleep(Duration::from_millis(10)),
            }
        }
    }

    fn grab_frame(
        &mut self,
    ) -> Result<Option<(crate::capture::Frame, crate::capture::Frame)>, SamplerError> {
        let camera = self.camera.as_mut().unwrap();

        let (left_raw, right_raw) = match camera.get_frames() {
            Ok(frames) => frames,
            Err(CameraError::InvalidFrame(_)) => return Ok(None),
            Err(e) => return Err(SamplerError::Camera(e.to_string())),
        };

        let left = self
            .left_preprocessor
            .process(left_raw)
            .map_err(|e| SamplerError::Camera(e.to_string()))?
            .clone();
        let right = self
            .right_preprocessor
            .process(right_raw)
            .map_err(|e| SamplerError::Camera(e.to_string()))?
            .clone();

        Ok(Some((left, right)))
    }

    fn ensure_camera(&mut self) -> Result<(), SamplerError> {
        if self.camera.is_none() {
            let (Some(left), Some(right)) = (&self.left_source, &self.right_source) else {
                return Err(SamplerError::Camera("no camera source configured".into()));
            };

            let camera = if left == right {
                StereoCamera::open_sbs(left)
            } else {
                StereoCamera::open(left, right)
            }
            .map_err(|e| SamplerError::Camera(e.to_string()))?;

            self.camera = Some(camera);
        }

        Ok(())
    }
}
