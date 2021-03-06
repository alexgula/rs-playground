extern crate sdl2;

use sdl2::pixels::Color;
use std::thread;

fn main() {
    // Initialize SDL2
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    // Create the window
    let window = video_subsystem.window("ArcadeRS Shooter", 800, 600)
        .position_centered().opengl()
        .build().unwrap();

    let mut renderer = window.renderer()
        .accelerated()
        .build().unwrap();

    renderer.set_draw_color(Color::RGB(50, 50, 50));
    renderer.clear();
    renderer.present();

    thread::sleep_ms(3000);
}
