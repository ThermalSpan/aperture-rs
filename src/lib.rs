#![allow(unused_variables)]
#![allow(dead_code)]

extern crate cgmath;
#[macro_use]
extern crate getset;

mod camera;
mod perspective;

pub use perspective::perspective_transform;
pub use perspective::fov_perspective_transform;
pub use camera::Camera;
pub use camera::MouseButton;
pub use camera::ButtonState;

