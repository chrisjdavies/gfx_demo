# gfx_demo

Once upon a time, I used to write graphics demos for fun.

There was no real purpose other than to experiment with a new cool algorithm --
like rendering flames or plasma.

Then I went to university, got a job and apparently became boring.  But it's not
all my fault: the friction to creating these sort of things made it much more of
a hassle than my QuickBASIC days.

Rust seems to have done pretty well at making development quite fun, especially
with modern IDE support (I quite love Visual Studio Code these days).

All this to say: this is a small wrapper around getting an SDL window launched
and backed with a `u32` vector for playing with graphics things.  There's
nothing fancy here and I don't expect anyone other than myself to be using this.

## Example

`gfx_demo` is the main interface here:

```rust
extern crate gfx_demo;

const WINDOW_TITLE: &'static str = "Fire";
const WINDOW_WIDTH: usize = 640;
const WINDOW_HEIGHT: usize = 480;
const CANVAS_WIDTH: usize = 100;
const CANVAS_HEIGHT: usize = 100;
const TICK_MS: u64 = 10;

fn main() {
    let mut i = 0;
    let mut inc = true;

    gfx_demo::gfx_demo(
        WINDOW_TITLE,
        WINDOW_WIDTH, WINDOW_HEIGHT,
        CANVAS_WIDTH, CANVAS_HEIGHT,
        TICK_MS,
        |pixels: &mut Vec<u32>| {
            if inc {
                i += 1;
            } else {
                i -= 1;
            }

            inc = match i {
                0 => true,
                255 => false,
                _ => inc
            };

            for y in 0..CANVAS_HEIGHT {
                for x in 0..CANVAS_WIDTH {
                    pixels[((y * CANVAS_WIDTH) + x) as usize] = 0xff000000u32 | (i << 16);
                }
            }
        }
    ).unwrap();
}
```

All you/I get is the `Vec<u32>` backing the window surface and that's all I need
to play and have fun in 2023.
