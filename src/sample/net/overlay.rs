use std::io;
use std::process::{Child, Command};
use std::time::Duration;

use super::internal::connection::OverlayConnection;
use super::internal::packet::{
    OverlayMessage, Packet,
    RunFixedLengthRoutinePacket, RunVariableLengthRoutinePacket,
};

pub enum Routine {
    GazeTutorial, // 30s
    ShortGazeTutorial, // 5s
    Gaze(Duration),
    BlinkTutorial, // 10s
    ShortBlinkTutorial, // 4s
    Blink(Duration),
}

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub routine_pitch: f32,
    pub routine_yaw: f32,
    pub routine_distance: f32,
    pub routine_convergence: f32,
    pub fov_adjust_distance: f32,
    pub left_eye_pitch: f32,
    pub left_eye_yaw: f32,
    pub right_eye_pitch: f32,
    pub right_eye_yaw: f32,
}

pub enum Event {
    Finished,
    Position(Position),
}

#[derive(Clone, Copy)]
pub enum Mode {
    OpenVr,
    OpenXr,
    Debug,
}

pub struct Overlay {
    conn: OverlayConnection,
    child: Child,
}

impl Overlay {
    pub fn start(path: &str, mode: Mode) -> io::Result<Self> {
        let child = Command::new(path)
            .args(match mode {
                Mode::OpenVr => &["--use-openvr"] as &[&str],
                Mode::OpenXr => &["--xr-mode", "on"],
                Mode::Debug => &["--use-debug"],
            })
            .spawn()?;

        let conn = OverlayConnection::listen()?;
        std::thread::sleep(Duration::from_millis(1));

        Ok(Self { conn, child })
    }

    pub fn begin(&mut self, routine: Routine) -> io::Result<()> {
        let (name, duration) = match routine {
            Routine::GazeTutorial => ("gazetutorial", Duration::from_secs(30)),
            Routine::ShortGazeTutorial => ("gazetutorialshort", Duration::from_secs(5)),
            Routine::Gaze(d) => ("gaze", d),
            Routine::BlinkTutorial => ("blinktutorial", Duration::from_secs(10)),
            Routine::ShortBlinkTutorial => ("blinktutorial", Duration::from_secs(4)),
            Routine::Blink(d) => ("blink", d),
        };

        let packet = Packet::new(
            "RunVariableLenghtRoutinePacket",
            &RunVariableLengthRoutinePacket::new(name, duration.as_secs_f64()),
        );
        self.conn.send(&packet)
    }

    pub fn try_recv(&mut self) -> io::Result<Option<Event>> {
        match self.conn.try_recv()? {
            Some(OverlayMessage::PositionalData(data)) => {
                Ok(Some(Event::Position(Position {
                    routine_pitch: data.routine_pitch,
                    routine_yaw: data.routine_yaw,
                    routine_distance: data.routine_distance,
                    routine_convergence: data.routine_convergence,
                    fov_adjust_distance: data.fov_adjust_distance,
                    left_eye_pitch: data.left_eye_pitch,
                    left_eye_yaw: data.left_eye_yaw,
                    right_eye_pitch: data.right_eye_pitch,
                    right_eye_yaw: data.right_eye_yaw,
                })))
            }
            Some(OverlayMessage::RoutineFinished(_)) => Ok(Some(Event::Finished)),
            Some(OverlayMessage::Unknown(_)) => Ok(None),
            None => Ok(None),
        }
    }

    pub fn close(&mut self) -> io::Result<()> {
        let packet = Packet::new(
            "RunFixedLenghtRoutinePacket",
            &RunFixedLengthRoutinePacket {
                routine_name: "close".to_string(),
            },
        );
        self.conn.send(&packet)
    }
}

impl Drop for Overlay {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}
