#![windows_subsystem = "windows"]

use image;
use minifb::{Key, Window, WindowOptions};
use rodio::{Decoder, OutputStream, Sink};
use std::io::Cursor;

// Embed the image and sound data into the portable .EXE at build time
// change this to the name of your audio files if they're not mp3
const IMAGE_DATA: &[u8] = include_bytes!("image.png"); // [Default: "image.png"] [any file of types: png, gif, tif, jpg/jpeg, bmp]
const AUDIO_DATA: &[u8] = include_bytes!("sound.mp3"); // [Default: "sound.mp3"] [any file of types: mp3, flac, ogg, wav]

/// Window title
/// [Default: "Image Displayer Pro"] [Any string]
const WINDOW_TITLE: &str = "Image Displayer Pro";


/// Window scaling factor
/// [Default: X1] [X1, X2, X4, X8, X16, X32, FitScreen]
const SCALE: minifb::Scale = minifb::Scale::X1;


/// Window close keybind
/// [Default: Escape] [Any valid Key enum]
const EXIT_KEY: Key = Key::Escape;


/// Enable Window close keybind
/// [Default: true] [true, false]
const EXIT_KEY_ENABLED: bool = true;


/// Hide window decorations
/// [Default: true] [true, false]
const HIDE_CLOSE: bool = true;


/// Keep window always on top
/// [Default: true] [true, false]
const ALWAYS_ON_TOP: bool = true;


/// Show executable icon in title bar
/// [Default: false] [true, false]
const ICON_ENABLED: bool = false;


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
            title: ICON_ENABLED,
            resize: true,
            scale: SCALE,
            scale_mode: minifb::ScaleMode::AspectRatioStretch,
            topmost: ALWAYS_ON_TOP,
            transparency: false,
            none: false,
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.set_target_fps(60);

    // Displays the window prepared before and handles updates
    while window.is_open() && !(window.is_key_down(EXIT_KEY) && EXIT_KEY_ENABLED) {
        window.update_with_buffer(&buffer, width, height).unwrap();
    }

    sink.stop();
}
