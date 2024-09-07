#![windows_subsystem = "windows"]

use image;
use minifb::{Key, Window, WindowOptions};
use rodio::{Decoder, OutputStream, Sink};
use std::io::Cursor;

// Embed the image and sound data into the portable .EXE at build time
const IMAGE_DATA: &[u8] = include_bytes!("image.png");
const AUDIO_DATA: &[u8] = include_bytes!("sound.mp3");

// Config 'n' shit.
const WINDOW_TITLE: &str = "Image Displayer Pro";   // ["Image Displayer Pro"] Title of the window.
const EXIT_KEY: Key = Key::Escape;                  // [Key::Escape] Key to close the window
const HIDE_CLOSE: bool = true;                      // [true] Wether or not to hide window decorations

fn main() {
    // Load the image
    let img = image::load_from_memory(IMAGE_DATA).unwrap();
    let (width, height) = (img.width() as usize, img.height() as usize);

    // Convert image to ARGB buffer (IDK i stole this code)
    let buffer: Vec<u32> = img
        .to_rgba8()
        .pixels()
        .map(|p| {
            let [r, g, b, a] = p.0;
            ((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
        })
        .collect();

    // audio shit
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    let audio_cursor = Cursor::new(AUDIO_DATA);
    let audio_source = Decoder::new(audio_cursor).unwrap();
    sink.append(audio_source);
    sink.play();

    // Create da window
    let mut window = Window::new(
        WINDOW_TITLE,
        width,
        height,
        WindowOptions {
            borderless: HIDE_CLOSE,
            title: false,
            resize: false,
            scale: minifb::Scale::X1,
            scale_mode: minifb::ScaleMode::AspectRatioStretch,
            topmost: true,
            transparency: false,
            none: false,
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.set_target_fps(60);

    // Displays the window prepared before and handles updates
    while window.is_open() && !window.is_key_down(EXIT_KEY) {
        window.update_with_buffer(&buffer, width, height).unwrap();
    }

    sink.stop();
}
