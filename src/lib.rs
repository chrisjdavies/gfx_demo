extern crate sdl2;

use std::time::Duration;

use sdl2::event::Event;
use sdl2::pixels::PixelFormatEnum;

/// Runs an SDL graphics demo.
///
/// Implements all of the setup code for starting SDL, creating the window and
/// setting up a pixel buffer.  The pixel buffer is in ARGB8888 format and is
/// passed to `tick_fn` every frame.
///
/// # Arguments
///
/// * `title` - The title of the window.
/// * `window_width` - How wide the display window should be.
/// * `window_height` - How tall the display window should be.
/// * `canvas_width` - How wide the canvas should be.
/// * `canvas_height` - How tall the canvas should be.
/// * `tick_ms` - How many milliseconds to wait between refresh calls.
/// * `tick_fn` - The callback function for implementing the graphics.
///
pub fn gfx_demo<F>(
    title: &str,
    window_width: u32,
    window_height: u32,
    canvas_width: u32,
    canvas_height: u32,
    tick_ms: u64,
    mut tick_fn: F,
) -> Result<(), String>
where
    F: FnMut(&mut Vec<u32>),
{
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window(title, window_width, window_height)
        .build()
        .map_err(|_| "Failed to create SDL window")?;

    let mut canvas = window
        .into_canvas()
        .build()
        .map_err(|_| "Failed to create SDL canvas")?;
    let texture_creator = canvas.texture_creator();

    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::ARGB8888, canvas_width, canvas_height)
        .map_err(|_| "Failed to create SDL texture")?;

    let mut pixels = vec![0xff000000u32; (canvas_width * canvas_height) as usize];
    let u8pixels = unsafe { std::mem::transmute(pixels.as_mut_slice()) };

    let mut event_pump = sdl_context.event_pump()?;

    let mut finished = false;
    while !finished {
        tick_fn(&mut pixels);

        texture
            .update(None, u8pixels, (canvas_width * 4) as usize)
            .map_err(|s| format!("Failed to update texture: {}", s))?;
        canvas.copy(&texture, None, None)?;
        canvas.present();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => finished = true,
                _ => (),
            }
        }

        std::thread::sleep(Duration::from_millis(tick_ms));
    }

    Ok(())
}
