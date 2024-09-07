# Image Displayer Pro

This is a simple, harmless prank app written in 100% safe Rust. It displays an image in a window and plays a funny sound when the app is launched.

## Features
- Displays a window matching the image size.
- Plays a sound file provided at build time.
- Portable Windows `.exe` with no external dependencies needed after build.
- Great for light-hearted pranks!
- Almost instant subsequent builds for your next prank idea.

## You'll need
   1. Great taste.
   2. An idea.
   3. An image.
   4. A sound.
   5. Rust build tools installed

## Setup
Use this majestic project in 3 easy steps

1. **Edit the files**:
   - Edit `./src/main.rs` to change window title, window decorations, and key options.
   - Replace `./src/image.png` with your image.
   - Replace `./src/sound.mp3` with your sound.

2. **Build**:
   Run the following command to build the project:
   ```sh
   cargo build --release
   ```

3. **Enjoy**:
   You'll find the finished `.exe` in `./target/release/ImageDisplayerPro.exe`. Send this to all your friends!

## Notes
- All dependencies are managed via `Cargo.toml` and are installed automatically during the build.
- This project uses only safe Rust with no unsafe blocks.