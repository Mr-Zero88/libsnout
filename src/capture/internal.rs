pub mod http;
pub mod v4l;

use image::GrayImage;
pub use v4l::V4lCamera;

use crate::capture::CameraError;

pub trait Camera {
    fn read_frame(&mut self) -> Result<GrayImage, CameraError>;
}
