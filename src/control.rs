use std::io;
use std::net::{ToSocketAddrs, UdpSocket};
use std::str::FromStr;

use rosc::{OscMessage, OscPacket, OscType};

use crate::calibration::FaceShape;

/// The largest OSC datagram we'll accept.
const MAX_PACKET_SIZE: usize = 1024;

/// A decoded control command.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ControlEvent {
    /// Set the input calibration range (`lower`, `upper`) for a face shape.
    FaceBounds(FaceShape, f32, f32),
    /// Begin a neutral-hold face calibration pass.
    FaceCalibrate,
    /// Begin an upper-bound calibration pass for a single shape, capturing the
    /// peak over the given number of frames.
    FaceCalibrateUpper(FaceShape, u32),
}

/// Receives control commands over OSC/UDP.
pub struct OscControl {
    socket: UdpSocket,
    buf: [u8; MAX_PACKET_SIZE],
}

impl OscControl {
    /// Binds a UDP socket to `addr` and listens for control messages.
    pub fn bind(addr: impl ToSocketAddrs) -> io::Result<Self> {
        Ok(Self {
            socket: UdpSocket::bind(addr)?,
            buf: [0; MAX_PACKET_SIZE],
        })
    }

    /// Blocks until a recognized control message arrives and returns it.
    ///
    /// Datagrams that don't decode as OSC, use an unknown address, or carry
    /// malformed arguments are skipped silently; the loop keeps reading. Only a
    /// genuine socket error is surfaced, so the caller can stop cleanly instead
    /// of spinning.
    pub fn receive(&mut self) -> io::Result<ControlEvent> {
        loop {
            let (len, _) = self.socket.recv_from(&mut self.buf)?;

            let Ok((_, packet)) = rosc::decoder::decode_udp(&self.buf[..len]) else {
                continue;
            };

            if let Some(event) = decode_packet(&packet) {
                return Ok(event);
            }
        }
    }
}

fn decode_packet(packet: &OscPacket) -> Option<ControlEvent> {
    match packet {
        OscPacket::Message(message) => decode_message(message),
        OscPacket::Bundle(bundle) => bundle.content.iter().find_map(decode_packet),
    }
}

fn decode_message(message: &OscMessage) -> Option<ControlEvent> {
    match message.addr.as_str() {
        "/snout/face/bounds" => {
            let [
                OscType::String(shape),
                OscType::Float(lower),
                OscType::Float(upper),
            ] = message.args.as_slice()
            else {
                return None;
            };

            let shape = FaceShape::from_str(shape).ok()?;
            Some(ControlEvent::FaceBounds(shape, *lower, *upper))
        }
        "/snout/face/calibrate" => Some(ControlEvent::FaceCalibrate),
        "/snout/face/calibrate/upper" => {
            let [OscType::String(shape), OscType::Int(frames)] = message.args.as_slice() else {
                return None;
            };

            let shape = FaceShape::from_str(shape).ok()?;
            Some(ControlEvent::FaceCalibrateUpper(
                shape,
                (*frames).max(1) as u32,
            ))
        }
        _ => None,
    }
}
