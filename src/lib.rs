#![allow(unused_variables)]
#![allow(dead_code)]

// The assert crate is a dev dependency
// However I need it for unit testing, so I think I need it here?
extern crate assert;
extern crate cgmath;
#[macro_use]
extern crate getset;
#[cfg(feature = "eventhandler")]
extern crate glutin; // TODO: This could prolly be winit?

mod camera;
#[cfg(feature = "eventhandler")]
mod eventhandler;
mod perspective;

#[cfg(feature = "eventhandler")]
pub use eventhandler::camera_event_handler;
pub use perspective::perspective_transform;
pub use perspective::fov_perspective_transform;
pub use camera::Camera;
pub use camera::MouseButton;
pub use camera::ButtonState;
