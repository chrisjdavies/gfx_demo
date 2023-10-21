extern crate sdl2;

use std::time::Duration;

use sdl2::event::Event;
use sdl2::pixels::PixelFormatEnum;

pub fn gfx_demo<F>(
    title: &str,
    window_width: u32,
    window_height: u32,
    canvas_width: u32,
    canvas_height: u32,
    tick_ms: u64,
    mut tick_fn: F,
) where
    F: FnMut(&mut Vec<u32>),
{
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window(title, window_width, window_height)
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::ARGB8888, canvas_width, canvas_height)
        .unwrap();
    let mut pixels = vec![0xff000000u32; (canvas_width * canvas_height) as usize];
    let u8pixels = unsafe { std::mem::transmute(pixels.as_mut_slice()) };
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut finished = false;

    while !finished {
        tick_fn(&mut pixels);

        texture
            .update(None, u8pixels, (canvas_width * 4) as usize)
            .unwrap();
        canvas.copy(&texture, None, None).unwrap();
        canvas.present();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => finished = true,
                _ => (),
            }
        }

        std::thread::sleep(Duration::from_millis(tick_ms));
    }
}