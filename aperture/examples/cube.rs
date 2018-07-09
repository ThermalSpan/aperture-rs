extern crate cgmath;
#[macro_use]
extern crate glium;
extern crate aperture;

use cgmath::prelude::*;
use glium::glutin;
use glium::Surface;
use std::thread::sleep;
use std::time::{Duration, SystemTime};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
}
implement_vertex!(Vertex, position);

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("Cube Example")
        .with_dimensions(1024, 1024);
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    // We statically include the shader sources, and build the shader program
    let vertex_shader_src = "
#version 400
in vec3 position;
uniform mat4 rotation_matrix;
uniform vec4 u_color;
out vec4 f_color;
void main() {
    f_color = u_color;
    gl_Position = rotation_matrix * vec4(position, 1.0);
}";

    let fragment_shader_src = "
#version 400
in vec4 f_color;
out vec4 color;
void main() {
    color = f_color;
}";

    let shader_program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

    // Build up the outline of a cube
    //
    // 1.) Make a vertex buffer with all the corners
    let mut vertices = Vec::new();
    for x in vec![-1.0, 1.0] {
        for y in vec![-1.0, 1.0] {
            for z in vec![-1.0, 1.0] {
                vertices.push(Vertex {
                    position: [x, y, z],
                })
            }
        }
    }
    let vertex_buffer = glium::VertexBuffer::new(&display, &vertices).unwrap();

    // 2.) Make an index buffer with all the appropriate endpoints
    let indices = glium::IndexBuffer::new(
        &display,
        glium::index::PrimitiveType::LinesList,
        #[cfg_attr(rustfmt, rustfmt_skip)]
        &[0, 1, 1, 3, 3, 2, 2, 0,
          4, 5, 5, 7, 7, 6, 6, 4,
          0, 4, 1, 5, 3, 7, 2, 6u16],
    ).unwrap();

    // Drawing parameters
    let params = glium::DrawParameters {
        line_width: Some(5.0),
        blend: glium::Blend::alpha_blending(),
        ..Default::default()
    };

    let mut closed = false;
    let mut cam = aperture::Camera::new();
    let fps: f32 = 60.0;
    let frame_duration_cap = Duration::from_millis(((1.0 / fps) * 1000.0) as u64);
    let mut current_time = SystemTime::now();
    while !closed {
        let mut target = display.draw();
        // listing the events produced by application and waiting to be received
        events_loop.poll_events(|ev| match ev {
            glutin::Event::WindowEvent {
                event: glutin::WindowEvent::Closed,
                ..
            } => {
                closed = true;
            }
            event => {
                aperture::camera_event_handler(&mut cam, event);
            }
        });

        let new_time = SystemTime::now();
        let frame_time = current_time.elapsed().unwrap();
        let elapsed_millis =
            (1000 * frame_time.as_secs() + frame_time.subsec_millis() as u64) as f32;
        current_time = new_time;

        let (window_width, window_height) = {
            let (window_width_i, window_height_i) = target.get_dimensions();
            (window_width_i as f32, window_height_i as f32)
        };

        cam.update(elapsed_millis, window_width, window_height);

        let world_transform = cam.get_clipspace_transform();

        // A weird yellow background
        target.clear_color(0.7654, 0.567, 0.1245, 1.0);

        // Lets make an interesting nested cube thing
        let segments: u16 = 50;
        for i in (1..segments) {
            let frac = i as f32 / segments as f32;
            let scale = cgmath::Matrix4::from_scale(frac);
            let object_transform: [[f32; 4]; 4] = (world_transform * scale).into();

            let uniforms = uniform!{
                rotation_matrix: object_transform,
                u_color: [frac, 0.134 * frac, 1.0 - frac, 0.5]
            };

            // Clear the screen, draw, and swap the buffers
            target
                .draw(
                    &vertex_buffer,
                    &indices,
                    &shader_program,
                    &uniforms,
                    &params,
                )
                .unwrap();
        }

        // Here we limit the framerate to avoid consuming uneeded CPU time
        let elapsed = current_time.elapsed().unwrap();
        if elapsed < frame_duration_cap {
            let sleep_time = frame_duration_cap - elapsed;
            sleep(sleep_time);
        }

        target.finish().unwrap();
    }
}
