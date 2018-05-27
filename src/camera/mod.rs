use cgmath::prelude::*;
use cgmath::{Basis3, Matrix3, Matrix4, Rad, Vector2, Vector3, Quaternion};
use std::f32;
use std::time::Duration;
use perspective::fov_perspective_transform;

enum CamState {
    Pan,
    Tumble,
    Idle,
}

pub enum MouseButton {
    Left,
    Right,
}

pub enum ButtonState {
    Pressed,
    Released,
}

/// The camera struct maintains all the state of the camera. In order to maintain correct the
/// correct aspect ratio and timing for orbital mechanics, it needs to be updated every frame.
#[derive(Getters, Setters)]
pub struct Camera {
    state: CamState,
    window_width: f32,
    window_height: f32,
    aspect_ratio: f32,

    prev_mouse_coords: Vector2<f32>,

    // These are for maintaining state with the arcball calculations
    original_rotation: Quaternion<f32>,
    original_sphere_point: Vector3<f32>,

    /// How far the camera is from the target in world coordinates
    #[get = "pub"]
    #[set = "pub"]
    distance: f32,

    /// The distance from the camera to the far plane of the viewing frustrum
    #[get = "pub"]
    #[set = "pub"]
    far: f32,

    /// The field of view to use when making the perspective transform
    #[get = "pub"]
    #[set = "pub"]
    field_of_view: f32,

    /// How the camera is oriented relative to the target in world coordinates
    #[get = "pub"]
    #[set = "pub"]
    rotation: Quaternion<f32>,

    /// The factor applied to the number of pixels from each scroll event,
    /// I have default set of (1 / 200)
    #[get = "pub"]
    #[set = "pub"]
    scroll_modifier: f32,

    /// The target is where the camera points in world coordinates
    #[get = "pub"]
    #[set = "pub"]
    target: Vector3<f32>,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            state: CamState::Idle,
            target: Vector3::new(0.0, 0.0, 0.0),
            distance: 50.0,
            prev_mouse_coords: Vector2::zero(),
            original_rotation: Quaternion::one(),
            original_sphere_point: Vector3::zero(),
            rotation: Quaternion::from(Basis3::from_angle_y(Rad(-0.5 * f32::consts::PI))),
            window_width: 1.0,
            window_height: 1.0,
            aspect_ratio: 1.0,
            field_of_view: f32::consts::PI / 2.0,
            far: 100.0,
            scroll_modifier: 1.0 / 200.0,
        }
    }

    /// The update function should be called once per frame in order to maintain the aspect ratio
    /// and the timing for orbital mechanics. Ideally it should be called at the begining of your
    /// "simulation loop", right after you have calculated your frame time.
    pub fn update(&mut self, elapsed_time: Duration, window_width: f32, window_height: f32) {
        self.window_width = window_width;
        self.window_height = window_height;
        self.aspect_ratio = window_width / window_height;
    }

    /// Get the position of the camera in world coordinates
    pub fn get_position(&self) -> Vector3<f32> {
        self.target +
            self.rotation.rotate_vector(
                Vector3::unit_z() * self.distance,
            )
    }

    /// Get the world coordinates to clipspace coordinates transform
    /// If you are unsure, this is probably the transform you want from the camera.
    pub fn get_clipspace_transform(&self) -> Matrix4<f32> {
        // We need to move to transform the world so that the origin is the cam's pos
        let inverse_pos = -self.get_position();
        let pos_transform = Matrix4::from_translation(inverse_pos);

        let rotation_basis = Basis3::from(self.rotation);

        let rotation_transform = Matrix3::from(rotation_basis.invert());

        let perspective_transform =
            fov_perspective_transform(self.field_of_view, self.aspect_ratio, self.far);

        // We need to an inverted order of operations becuase the matrix is inverted(?)
        perspective_transform * Matrix4::from(rotation_transform) * pos_transform
    }

    fn mouse_to_sphere_point(&self, mouse_coords: Vector2<f32>) -> Vector3<f32> {
        // Figure out radius of arc-ball control circle in pixels
        // If aspect ratio > 1.0 then height is smaller,
        // So normalize mouse point by removing center then diving by that radius
        let pixel_radius = (if self.aspect_ratio >= 1.0 {
                                self.window_height
                            } else {
                                self.window_width
                            }) / 2.0;
        let screen_center = Vector2::new(self.window_width, self.window_height) * 0.5;
        let mouse_point = (mouse_coords - screen_center) / pixel_radius;

        // Now we find point on sphere by clamping to unit circle
        // and finding z component
        let mouse_radius = mouse_point.magnitude2();
        let sphere_point = if mouse_radius > 1.0 {
            (mouse_point / mouse_radius).extend(0.0)
        } else {
            mouse_point.extend((1.0 - mouse_radius).sqrt())
        };

        // If we were contraining axis, that would go here

        sphere_point
    }

    /// Handle mouse movement as pixel coordinates
    pub fn handle_mouse_move(&mut self, mouse_x: f32, mouse_y: f32) {
        self.prev_mouse_coords = Vector2::new(mouse_x, mouse_y);

        match self.state {
            CamState::Tumble => {
                let sphere_point = self.mouse_to_sphere_point(self.prev_mouse_coords);
                let rotation_axis = self.original_sphere_point.cross(sphere_point);
                let scalar = self.original_sphere_point.dot(sphere_point);
                let move_rotation = Quaternion::from_sv(scalar, rotation_axis);
                self.rotation = move_rotation * self.original_rotation;
            }
            _ => (),
        }
    }

    /// Handle mouse clicks,
    pub fn handle_mouse_input(&mut self, button: MouseButton, state: ButtonState) {
        match (button, state) {
            (MouseButton::Left, ButtonState::Pressed) => {
                self.state = CamState::Tumble;
                self.original_sphere_point = self.mouse_to_sphere_point(self.prev_mouse_coords);
                self.original_rotation = self.rotation.clone();
                println!("Mouse pressed");
            }
            (_, ButtonState::Released) => {
                self.state = CamState::Idle;
            }
            _ => (),
        }
    }

    // Handle scroll events as pixel deltas
    pub fn handle_scroll(&mut self, pixel_delta: f32) {
        let normalized_delta = pixel_delta * self.scroll_modifier;

        let scale = 1.0 + normalized_delta;

        self.distance *= scale;
    }

    /// Move the camera's target
    pub fn translate(&mut self, delta: Vector3<f32>) {
        self.target += delta;
    }
}
