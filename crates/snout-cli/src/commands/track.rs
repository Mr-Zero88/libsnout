use std::{thread::sleep, time::Duration};

use snout::{
    calibration::EyeShape, capture::discovery::query_cameras, config::Config, track::{eye::EyeTracker, face::FaceTracker, initialize_runtime, output::Output},
};

pub struct TrackCommand {
    config: Config,
    eye_debug: bool,
}

impl TrackCommand {
    pub fn new(config: Config, eye_debug: bool) -> Self {
        Self { config, eye_debug }
    }

    pub fn run(&self) {
        initialize_runtime(self.config.libonnxruntime.as_ref());

        let cameras = query_cameras();

        let mut face_tracker = FaceTracker::with_config(&cameras, &self.config).unwrap();
        let mut eye_tracker = EyeTracker::with_config(&cameras, &self.config).unwrap();

        let mut output = Output::with_config(&self.config).unwrap();

        println!("Tracking...");

        loop {
            let face_report = face_tracker.track().unwrap();
            let eye_report = eye_tracker.track().unwrap();

            if let Some(face_report) = face_report {
                output.send_face(face_report.weights);
            }

            if let Some(eye_report) = eye_report {
                if self.eye_debug {
                    print!("\rL({:+.2},{:+.2}) R({:+.2},{:+.2}) lids:{:2.0}/{:2.0}            ",
                        eye_report.weights.get(EyeShape::LeftEyePitch).unwrap_or(0.),
                        eye_report.weights.get(EyeShape::LeftEyeYaw).unwrap_or(0.),
                        eye_report.weights.get(EyeShape::RightEyePitch).unwrap_or(0.),
                        eye_report.weights.get(EyeShape::RightEyeYaw).unwrap_or(0.),
                        eye_report.weights.get(EyeShape::LeftEyeLid).unwrap_or(0.) * 100.,
                        eye_report.weights.get(EyeShape::RightEyeLid).unwrap_or(0.) * 100.
                    );
                }

                output.send_eyes(eye_report.weights);
            }

            output.flush().unwrap();

            sleep(Duration::from_millis(10));
        }
    }
}
