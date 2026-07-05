use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::calibration::Bounds;
use crate::weights::{Shape, Weights};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum FaceShape {
    CheekPuffLeft,
    CheekPuffRight,
    CheekSuckLeft,
    CheekSuckRight,
    JawOpen,
    JawForward,
    JawLeft,
    JawRight,
    NoseSneerLeft,
    NoseSneerRight,
    MouthFunnel,
    MouthPucker,
    MouthLeft,
    MouthRight,
    MouthRollUpper,
    MouthRollLower,
    MouthShrugUpper,
    MouthShrugLower,
    MouthClose,
    MouthSmileLeft,
    MouthSmileRight,
    MouthFrownLeft,
    MouthFrownRight,
    MouthDimpleLeft,
    MouthDimpleRight,
    MouthUpperUpLeft,
    MouthUpperUpRight,
    MouthLowerDownLeft,
    MouthLowerDownRight,
    MouthPressLeft,
    MouthPressRight,
    MouthStretchLeft,
    MouthStretchRight,
    TongueOut,
    TongueUp,
    TongueDown,
    TongueLeft,
    TongueRight,
    TongueRoll,
    TongueBendDown,
    TongueCurlUp,
    TongueSquish,
    TongueFlat,
    TongueTwistLeft,
    TongueTwistRight,
}

impl FaceShape {
    pub const fn count() -> usize {
        const {
            assert!(Self::TongueTwistRight as usize + 1 == 45);
        }

        Self::TongueTwistRight as usize + 1
    }

    pub(crate) fn to_babble(self) -> &'static str {
        match self {
            FaceShape::CheekPuffLeft => "/cheekPuffLeft",
            FaceShape::CheekPuffRight => "/cheekPuffRight",
            FaceShape::CheekSuckLeft => "/cheekSuckLeft",
            FaceShape::CheekSuckRight => "/cheekSuckRight",
            FaceShape::JawOpen => "/jawOpen",
            FaceShape::JawForward => "/jawForward",
            FaceShape::JawLeft => "/jawLeft",
            FaceShape::JawRight => "/jawRight",
            FaceShape::NoseSneerLeft => "/noseSneerLeft",
            FaceShape::NoseSneerRight => "/noseSneerRight",
            FaceShape::MouthFunnel => "/mouthFunnel",
            FaceShape::MouthPucker => "/mouthPucker",
            FaceShape::MouthLeft => "/mouthLeft",
            FaceShape::MouthRight => "/mouthRight",
            FaceShape::MouthRollUpper => "/mouthRollUpper",
            FaceShape::MouthRollLower => "/mouthRollLower",
            FaceShape::MouthShrugUpper => "/mouthShrugUpper",
            FaceShape::MouthShrugLower => "/mouthShrugLower",
            FaceShape::MouthClose => "/mouthClose",
            FaceShape::MouthSmileLeft => "/mouthSmileLeft",
            FaceShape::MouthSmileRight => "/mouthSmileRight",
            FaceShape::MouthFrownLeft => "/mouthFrownLeft",
            FaceShape::MouthFrownRight => "/mouthFrownRight",
            FaceShape::MouthDimpleLeft => "/mouthDimpleLeft",
            FaceShape::MouthDimpleRight => "/mouthDimpleRight",
            FaceShape::MouthUpperUpLeft => "/mouthUpperUpLeft",
            FaceShape::MouthUpperUpRight => "/mouthUpperUpRight",
            FaceShape::MouthLowerDownLeft => "/mouthLowerDownLeft",
            FaceShape::MouthLowerDownRight => "/mouthLowerDownRight",
            FaceShape::MouthPressLeft => "/mouthPressLeft",
            FaceShape::MouthPressRight => "/mouthPressRight",
            FaceShape::MouthStretchLeft => "/mouthStretchLeft",
            FaceShape::MouthStretchRight => "/mouthStretchRight",
            FaceShape::TongueOut => "/tongueOut",
            FaceShape::TongueUp => "/tongueUp",
            FaceShape::TongueDown => "/tongueDown",
            FaceShape::TongueLeft => "/tongueLeft",
            FaceShape::TongueRight => "/tongueRight",
            FaceShape::TongueRoll => "/tongueRoll",
            FaceShape::TongueBendDown => "/tongueBendDown",
            FaceShape::TongueCurlUp => "/tongueCurlUp",
            FaceShape::TongueSquish => "/tongueSquish",
            FaceShape::TongueFlat => "/tongueFlat",
            FaceShape::TongueTwistLeft => "/tongueTwistLeft",
            FaceShape::TongueTwistRight => "/tongueTwistRight",
        }
    }
}

impl From<FaceShape> for usize {
    fn from(value: FaceShape) -> Self {
        value as usize
    }
}

impl From<usize> for FaceShape {
    fn from(value: usize) -> Self {
        assert!(value < Self::count());

        unsafe { std::mem::transmute(value as u8) }
    }
}

/// Error returned when a string does not name a [`FaceShape`].
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ParseFaceShapeError;

impl std::fmt::Display for ParseFaceShapeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("unknown face shape")
    }
}

impl std::error::Error for ParseFaceShapeError {}

impl FromStr for FaceShape {
    type Err = ParseFaceShapeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "cheekPuffLeft" => Self::CheekPuffLeft,
            "cheekPuffRight" => Self::CheekPuffRight,
            "cheekSuckLeft" => Self::CheekSuckLeft,
            "cheekSuckRight" => Self::CheekSuckRight,
            "jawOpen" => Self::JawOpen,
            "jawForward" => Self::JawForward,
            "jawLeft" => Self::JawLeft,
            "jawRight" => Self::JawRight,
            "noseSneerLeft" => Self::NoseSneerLeft,
            "noseSneerRight" => Self::NoseSneerRight,
            "mouthFunnel" => Self::MouthFunnel,
            "mouthPucker" => Self::MouthPucker,
            "mouthLeft" => Self::MouthLeft,
            "mouthRight" => Self::MouthRight,
            "mouthRollUpper" => Self::MouthRollUpper,
            "mouthRollLower" => Self::MouthRollLower,
            "mouthShrugUpper" => Self::MouthShrugUpper,
            "mouthShrugLower" => Self::MouthShrugLower,
            "mouthClose" => Self::MouthClose,
            "mouthSmileLeft" => Self::MouthSmileLeft,
            "mouthSmileRight" => Self::MouthSmileRight,
            "mouthFrownLeft" => Self::MouthFrownLeft,
            "mouthFrownRight" => Self::MouthFrownRight,
            "mouthDimpleLeft" => Self::MouthDimpleLeft,
            "mouthDimpleRight" => Self::MouthDimpleRight,
            "mouthUpperUpLeft" => Self::MouthUpperUpLeft,
            "mouthUpperUpRight" => Self::MouthUpperUpRight,
            "mouthLowerDownLeft" => Self::MouthLowerDownLeft,
            "mouthLowerDownRight" => Self::MouthLowerDownRight,
            "mouthPressLeft" => Self::MouthPressLeft,
            "mouthPressRight" => Self::MouthPressRight,
            "mouthStretchLeft" => Self::MouthStretchLeft,
            "mouthStretchRight" => Self::MouthStretchRight,
            "tongueOut" => Self::TongueOut,
            "tongueUp" => Self::TongueUp,
            "tongueDown" => Self::TongueDown,
            "tongueLeft" => Self::TongueLeft,
            "tongueRight" => Self::TongueRight,
            "tongueRoll" => Self::TongueRoll,
            "tongueBendDown" => Self::TongueBendDown,
            "tongueCurlUp" => Self::TongueCurlUp,
            "tongueSquish" => Self::TongueSquish,
            "tongueFlat" => Self::TongueFlat,
            "tongueTwistLeft" => Self::TongueTwistLeft,
            "tongueTwistRight" => Self::TongueTwistRight,
            _ => return Err(ParseFaceShapeError),
        })
    }
}

impl Shape for FaceShape {
    fn count() -> usize {
        const {
            assert!(Self::TongueTwistRight as usize + 1 == 45);
        }

        Self::TongueTwistRight as usize + 1
    }
}

pub struct ManualFaceCalibrator {
    bounds: FaceBounds,
    weights: Weights<FaceShape>,
    neutral_hold: NeutralHold,
    peak_capture: PeakCapture,
}

struct FaceBounds {
    bounds: Vec<Bounds>,
}

impl FaceBounds {
    fn new() -> Self {
        Self {
            bounds: vec![Bounds::new_01(); FaceShape::count()],
        }
    }

    fn get(&self, shape: FaceShape) -> Bounds {
        self.bounds[shape as usize]
    }

    fn set(&mut self, shape: FaceShape, bounds: Bounds) {
        tracing::debug!(shape = ?shape, ?bounds, "set_bounds");
        self.bounds[shape as usize] = bounds;
    }

    fn set_lower(&mut self, shape: FaceShape, lower: f32) {
        tracing::debug!(shape = ?shape, lower, "set_lower");
        self.bounds[shape as usize].lower = lower;
    }

    fn set_upper(&mut self, shape: FaceShape, upper: f32) {
        tracing::debug!(shape = ?shape, upper, "set_upper");
        self.bounds[shape as usize].upper = upper;
    }

    fn remap_into(&self, raw: &Weights<FaceShape>, out: &mut Weights<FaceShape>) {
        out.clear();

        for (shape, value) in raw.iter() {
            out.set(shape, self.bounds[shape as usize].remap(value));
        }
    }
}

/// At ~30 fps this is roughly three seconds of neutral hold.
const CALIBRATION_SAMPLES: usize = 100;

struct NeutralHold {
    remaining: usize,
    sums: Vec<f32>,
    counts: Vec<u32>,
}

impl NeutralHold {
    fn new() -> Self {
        Self {
            remaining: 0,
            sums: vec![0.0; FaceShape::count()],
            counts: vec![0; FaceShape::count()],
        }
    }

    fn start(&mut self, frames: usize) {
        tracing::debug!(frames, "start_calibration");

        self.remaining = frames;
        self.sums.fill(0.0);
        self.counts.fill(0);
    }

    fn feed(&mut self, raw: &Weights<FaceShape>, bounds: &mut FaceBounds) {
        if self.remaining == 0 {
            return;
        }

        for (shape, value) in raw.iter() {
            let index = shape as usize;
            self.sums[index] += value;
            self.counts[index] += 1;
        }

        self.remaining -= 1;
        if self.remaining != 0 {
            return;
        }

        for shape in FaceShape::iter() {
            let index = shape as usize;
            if self.counts[index] > 0 {
                let mean = self.sums[index] / self.counts[index] as f32;
                bounds.set_lower(shape, mean);
            }
        }
    }
}

struct PeakCapture {
    shape: FaceShape,
    remaining: usize,
    max: f32,
}

impl PeakCapture {
    fn new() -> Self {
        Self {
            shape: FaceShape::CheekPuffLeft,
            remaining: 0,
            max: f32::NEG_INFINITY,
        }
    }

    fn start(&mut self, shape: FaceShape, frames: usize) {
        tracing::debug!(?shape, frames, "start_upper_calibration");

        self.shape = shape;
        self.remaining = frames;
        self.max = f32::NEG_INFINITY;
    }

    fn feed(&mut self, raw: &Weights<FaceShape>, bounds: &mut FaceBounds) {
        if self.remaining == 0 {
            return;
        }

        if let Some(value) = raw.get(self.shape) {
            self.max = self.max.max(value);
        }

        self.remaining -= 1;
        if self.remaining != 0 {
            return;
        }

        bounds.set_upper(self.shape, self.max);
    }
}

impl ManualFaceCalibrator {
    pub fn new() -> Self {
        Self {
            bounds: FaceBounds::new(),
            weights: Weights::new(),
            neutral_hold: NeutralHold::new(),
            peak_capture: PeakCapture::new(),
        }
    }

    pub fn bounds(&self, shape: FaceShape) -> Bounds {
        self.bounds.get(shape)
    }

    pub fn set_bounds(&mut self, shape: FaceShape, bounds: Bounds) {
        self.bounds.set(shape, bounds);
    }

    pub fn set_upper(&mut self, shape: FaceShape, upper: f32) {
        self.bounds.set_upper(shape, upper);
    }

    pub fn set_lower(&mut self, shape: FaceShape, lower: f32) {
        self.bounds.set_lower(shape, lower);
    }

    pub fn start_calibration(&mut self) {
        self.neutral_hold.start(CALIBRATION_SAMPLES);
    }

    pub fn start_upper_calibration(&mut self, shape: FaceShape, frames: usize) {
        self.peak_capture.start(shape, frames);
    }

    pub fn calibrate(&mut self, raw: &Weights<FaceShape>) -> &Weights<FaceShape> {
        self.bounds.remap_into(raw, &mut self.weights);

        self.neutral_hold.feed(raw, &mut self.bounds);
        self.peak_capture.feed(raw, &mut self.bounds);

        &self.weights
    }
}
