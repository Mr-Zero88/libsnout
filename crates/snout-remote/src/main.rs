use std::net::UdpSocket;

use clap::{Parser, Subcommand};
use rosc::{OscMessage, OscPacket, OscType};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Address of the snout control listener.
    #[arg(short, long, default_value = "127.0.0.1:9500")]
    target: String,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Set the input calibration range for a face shape.
    FaceBounds {
        /// Face shape name, e.g. `jawOpen`, `mouthSmileLeft`, `tongueOut`.
        shape: String,
        /// Lower bound of the input range.
        lower: f32,
        /// Upper bound of the input range.
        upper: f32,
    },
    /// Begin a neutral-hold face calibration pass.
    FaceCalibrate,
    /// Capture the peak of a face shape over a window and set its upper bound.
    ///
    /// Hold (or sweep to) the maximum of the expression for the duration of the
    /// capture; the highest value seen becomes the shape's upper bound.
    FaceCalibrateUpper {
        /// Face shape name, e.g. `jawLeft`, `tongueOut`.
        shape: String,
        /// Number of frames to capture the peak over.
        #[arg(short, long, default_value_t = 100)]
        frames: u32,
    },
}

impl Command {
    fn into_message(self) -> OscMessage {
        match self {
            Command::FaceBounds {
                shape,
                lower,
                upper,
            } => OscMessage {
                addr: "/snout/face/bounds".to_string(),
                args: vec![
                    OscType::String(shape),
                    OscType::Float(lower),
                    OscType::Float(upper),
                ],
            },
            Command::FaceCalibrate => OscMessage {
                addr: "/snout/face/calibrate".to_string(),
                args: vec![],
            },
            Command::FaceCalibrateUpper { shape, frames } => OscMessage {
                addr: "/snout/face/calibrate/upper".to_string(),
                args: vec![OscType::String(shape), OscType::Int(frames as i32)],
            },
        }
    }
}

fn main() {
    let args = Args::parse();

    if let Err(error) = send(&args.target, args.command.into_message()) {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}

fn send(target: &str, message: OscMessage) -> Result<(), Box<dyn std::error::Error>> {
    let packet = OscPacket::Message(message);
    let buf = rosc::encoder::encode(&packet)
        .map_err(|e| format!("failed to encode OSC packet: {e:?}"))?;

    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.send_to(&buf, target)?;

    Ok(())
}
