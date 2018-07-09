#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

extern crate aperture;
#[macro_use]
extern crate lazy_static;
extern crate wasm_bindgen;

use std::sync::Mutex;
use wasm_bindgen::prelude::*;

lazy_static! {
    static ref CAMERA: Mutex<aperture::Camera> = Mutex::new(aperture::Camera::new());
}

#[wasm_bindgen]
pub fn update(elapsed_millis: f32, window_width: f32, window_height: f32) {
    let cam = &mut *CAMERA.lock().unwrap();
    cam.update(elapsed_millis, window_width, window_height);
}

#[wasm_bindgen]
pub fn get_clipspace_transform() -> Vec<f32> {
    let cam = &*CAMERA.lock().unwrap();
    let raw_transform: [[f32; 4]; 4] = cam.get_clipspace_transform().into();
    let mut result = Vec::new();
    for column in 0..4 {
        for row in 0..4 {
            result.push(raw_transform[column][row]);
        }
    }
    result
}

#[wasm_bindgen]
pub fn handle_scroll(pixel_delta: f32) {
    let cam = &mut *CAMERA.lock().unwrap();
    cam.handle_scroll(pixel_delta);
}
