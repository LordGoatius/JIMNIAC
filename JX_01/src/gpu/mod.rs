use sdl3::{Sdl, pixels::Color, render::Canvas, video::Window};
use ternary::{trits::Trit, word::Word};

use crate::memory::{Address, Memory};

/// Represents the inner GPU data structure
pub struct Gpu {
    // Internal GPU fields
    vector_buffer: Address,
    vector_buffer_size: Word,
    event_loop_callback: Address,
    gpu_state: Word, // temp
    // External SDL
    canvas: Canvas<Window>,
    sdl: Sdl,
}

impl Gpu {
    // Called by `egel`, and then stored in the CPU struct
    pub fn from_addr(mut addr: Address, memory: &mut Memory) -> Gpu {
        let sdl = sdl3::init().unwrap();
        let video_subsystem = sdl.video().unwrap();

        let window = video_subsystem
            .window("Ternary VM GPU Output", 729, 729)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas();

        let vector_buffer = *memory.get_physical_word(addr);
        addr = addr + Word::PONE;
        let vector_buffer_size = *memory.get_physical_word(addr);
        addr = addr + Word::PONE;
        let event_loop_callback = *memory.get_physical_word(addr);
        addr = addr + Word::PONE;
        let gpu_state = *memory.get_physical_word(addr);

        Gpu {
            vector_buffer,
            vector_buffer_size,
            event_loop_callback,
            gpu_state,
            canvas,
            sdl,
        }
    }

    pub(crate) fn reset_canvas(&mut self) {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();
    }

    /// # Unsafe
    /// The size of `coord_one` and `coord_two` must be 12.
    /// I may let the next 3 trits in the draw word to change color, but for now this works.
    /// I may need them to specify circle arc instead of line though, and I like green.
    unsafe fn draw_line(&mut self, coord_one: &[Trit], coord_two: &[Trit], color: Color) {
        self.canvas.set_draw_color(color);

        // x and y will in [-364, 364]. We need to add 364 to get a proper coordinate
        let x_1 = (coord_one.iter().enumerate().take(6).fold(0isize, |acc, (i, trit)| {
            acc + (3isize.pow(i as u32) * match trit {
                Trit::NOne => -1,
                Trit::Zero => 0,
                Trit::POne => 1,
            })
        }) + 364) as f32;

        let y_1 = (coord_one.iter().skip(6).take(6).enumerate().fold(0isize, |acc, (i, trit)| {
            acc + (3isize.pow(i as u32) * match trit {
                Trit::NOne => -1,
                Trit::Zero => 0,
                Trit::POne => 1,
            })
        }) + 364) as f32;

        let x_2 = (coord_two.iter().enumerate().take(6).fold(0isize, |acc, (i, trit)| {
            acc + (3isize.pow(i as u32) * match trit {
                Trit::NOne => -1,
                Trit::Zero => 0,
                Trit::POne => 1,
            })
        }) + 364) as f32;

        let y_2 = (coord_two.iter().skip(6).take(6).enumerate().fold(0isize, |acc, (i, trit)| {
            acc + (3isize.pow(i as u32) * match trit {
                Trit::NOne => -1,
                Trit::Zero => 0,
                Trit::POne => 1,
            })
        }) + 364) as f32;

        println!("{:?}", ((x_1, 729.0 - y_1), (x_2, 729.0 - y_2)));

        // Flip the x axis when drawing so that coordinates work as if it's centered on (0,0) in
        // [-364, 364] x [-364, 364]
        self.canvas.draw_line((x_1, 729.0 - y_1), (x_2, 729.0 - y_2)).unwrap();
    }

    fn draw(&mut self, word: Word) {
        let coord: [Trit; 27] = word.into();
        // of size 3
        let color: &[Trit] = &coord[(27 - 3)..];
        let trit_to_color = |t: Trit| {
            match t {
                Trit::NOne => 0,
                Trit::Zero => const { u8::MAX / 2 },
                Trit::POne => u8::MAX,
            }
        };
        let r = trit_to_color(color[0]);
        let g = trit_to_color(color[1]);
        let b = trit_to_color(color[2]);

        let color = Color::RGB(r, g, b);
        unsafe {
            self.draw_line(&coord[0..12], &coord[12..24], color);
        }
    }
}

#[cfg(test)]
pub mod tests {
    use std::time::Duration;

    use sdl3::{event::Event, keyboard::Keycode};
    use ternary::{trits::Trit, word::Word};

    use crate::gpu::Gpu;

    #[test]
    fn test_gpu() {
        let sdl = sdl3::init().unwrap();
        let video_subsystem = sdl.video().unwrap();

        let window = video_subsystem
            .window("Ternary VM GPU Output", 729, 729)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas();

        let mut gpu = Gpu {
            vector_buffer: Word::ZERO,
            vector_buffer_size: Word::ZERO,
            event_loop_callback: Word::ZERO,
            gpu_state: Word::ZERO,
            canvas,
            sdl,
        };

        gpu.reset_canvas();

        let coord1: Word = {
            use Trit::*;
            [
             POne, POne, POne, POne, POne, POne, POne, POne, POne, POne, POne, POne,
             NOne, NOne, NOne, NOne, NOne, NOne, NOne, NOne, NOne, NOne, NOne, NOne,
             POne, Zero, Zero
            ].into()
        };

        let coord2: Word = {
            use Trit::*;
            [
             POne, POne, POne, POne, POne, POne,
             NOne, NOne, NOne, NOne, NOne, NOne,
             NOne, NOne, NOne, NOne, NOne, NOne,
             POne, POne, POne, POne, POne, POne,
             Zero, POne, Zero
            ].into()
        };

        let coord3: Word = {
            use Trit::*;
            [
             Zero, Zero, Zero, Zero, Zero, Zero,
             POne, POne, POne, POne, POne, POne,
             Zero, Zero, Zero, Zero, Zero, Zero,
             NOne, NOne, NOne, NOne, NOne, NOne,
             Zero, Zero, POne
            ].into()
        };

        gpu.draw(coord1);
        gpu.draw(coord2);
        gpu.draw(coord3);

        gpu.canvas.present();

        let mut count = 0;

        'running: loop {
            for event in gpu.sdl.event_pump().unwrap().poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::AppTerminating { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape | Keycode::Q),
                        ..
                    } => break 'running,
                    _ => {
                        gpu.canvas.present();
                        std::thread::sleep(Duration::from_millis(50));
                        if count > 10 { break 'running; } else { count += 1; }
                    }
                }
            }
        }
    }
}
