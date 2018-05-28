use camera::*;
use glutin;

/// This function implements the suggested handling of mouse and keyboard input. Feel free to
/// intercept RecievedCharacter events if you do not want to use those shortcuts
pub fn camera_event_handler<'a>(cam: &'a mut Camera, event: glutin::Event) {
    match event {
        glutin::Event::WindowEvent { event, .. } => {
            match event {
                glutin::WindowEvent::MouseWheel {
                    delta: glutin::MouseScrollDelta::PixelDelta(_, y), ..
                } => {
                    cam.handle_scroll(y);
                }
                glutin::WindowEvent::CursorMoved { position: (x, y), .. } => {
                    cam.handle_mouse_move(x as f32, y as f32);
                }
                glutin::WindowEvent::MouseInput { state, button, .. } => {
                    match (state, button) {
                        (glutin::ElementState::Pressed, glutin::MouseButton::Left) => {
                            cam.handle_mouse_input(MouseButton::Left, ButtonState::Pressed);
                        }
                        (glutin::ElementState::Pressed, glutin::MouseButton::Right) => {
                            cam.handle_mouse_input(MouseButton::Right, ButtonState::Pressed);
                        }
                        (glutin::ElementState::Released, glutin::MouseButton::Left) => {
                            cam.handle_mouse_input(MouseButton::Left, ButtonState::Released);
                        }
                        (glutin::ElementState::Released, glutin::MouseButton::Right) => {
                            cam.handle_mouse_input(MouseButton::Right, ButtonState::Released);
                        }
                        _ => (),
                    }
                }
                glutin::WindowEvent::ReceivedCharacter(c) => {
                    match c {
                        's' => {
                            cam.set_current_as_default();
                        }
                        'd' => {
                            cam.transition_to_default();
                        }
                        _ => (),
                    }
                }
                _ => (),
            }
        }
        _ => (),
    }
}
