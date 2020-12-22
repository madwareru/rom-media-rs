use rom_media_rs::windowing::*;
use std::time::{Duration, Instant};
use rom_loaders_rs::images::sprite::BmpSprite;
use rom_media_rs::image_rendering::bresenham::{ plot_bresenham_4d};
use std::cmp::Ordering;
use rom_media_rs::image_rendering::triangles::{draw_brezenham_triangles};
use bumpalo::Bump;

const TEST_PAL: &[u32] = &[
    0xFF000000,
    0xFF050505,
    0xFF101010,
    0xFF151515,
    0xFF202020,
    0xFF252525,
    0xFF303030,
    0xFF353535,
    0xFF404040,
    0xFF454545,
    0xFF505050,
    0xFF555555,
    0xFF606060,
    0xFF656565,
    0xFF707070,
    0xFF757575,
    0xFF808080,
    0xFF858585,
    0xFF909090,
    0xFF959595,
    0xFFA0A0A0,
    0xFFA5A5A5,
    0xFFB0B0B0,
    0xFFB5B5B5,
    0xFFC0C0C0,
    0xFFC5C5C5,
    0xFFD0D0D0,
    0xFFD5D5D5,
    0xFFE0E0E0,
    0xFFE5E5E5,
    0xFFF0F0F0,
    0xFFF5F5F5
];

pub struct Window {
    game_over: bool,
    delta_instant: Instant,
    mouse_x: i32,
    mouse_y: i32,
    bench_results: [f32; 1024],
    cur_bench: usize,
    bump: Bump
}
impl PixelWindowHandler for Window {
    const FRAME_INTERVAL: Duration = Duration::from_micros(33000);
    fn update(&mut self) -> PixelWindowControlFlow {
        if self.game_over {
            PixelWindowControlFlow::Exit
        } else {
            let _dt = self.delta_instant.elapsed().as_millis() as f32 / 1_000.0;
            self.delta_instant = Instant::now();

            PixelWindowControlFlow::Continue
        }
    }
    fn render(&mut self, buffer: &mut [u32], w: u16, h: u16) {
        for entry in buffer.iter_mut() {
            *entry = 0xFF777777;
        }

        let inst = Instant::now();

        self.bump.reset();

        draw_brezenham_triangles(
            &self.bump,
            &[
                (
                    [(1, 1), (318, 1), (1, 178)],
                    [(0, 0), (255, 0), (0, 255)]
                ),
                (
                    [(318, 178), (318, 1), (1, 178)],
                    [(255, 255), (255, 0), (0, 255)]
                )
            ],
            w as usize, w as usize * h as usize,
            |idx, (u, v)| {
                let checker_board = (((u / 16) & 0x01) ^ ((v / 16) & 0x01)) as u32;
                let r = u as u32 * checker_board;
                let g = v as u32 * checker_board;
                buffer[idx] = 0xFF_00_00_00 | r * 0x1_00_00 | g * 0x1_00;
            }
        );

        plot_bresenham_4d(
            w as i32 / 2 , h as i32 / 2, 0, 0,
            self.mouse_x, self.mouse_y,
            (self.mouse_x - w as i32 / 2).abs().min(255),
            (self.mouse_y - h as i32 / 2).abs().min(255),
            |x, y, r, g| {
                let idx = (y as usize) * w as usize + x as usize;
                buffer[idx] = 0xFF000000 | (r as u32 * 0x10000) | (g as u32 * 0x100);
            }
        );

        self.bench_results[self.cur_bench] = inst.elapsed().as_micros() as f32 / 1000.0;
        self.cur_bench = (self.cur_bench + 1) % 1024;
    }

    fn on_key_pressed(&mut self, key: Key) {
        if key == Key::Escape {
            self.game_over = true;
        }
    }

    fn on_key_released(&mut self, key: Key) {
        println!("Key released: {:?}", key);
    }

    fn on_mouse_moved(&mut self, x: f64, y: f64) {
        self.mouse_x = ((x / 4.0) as i32).max(0).min(319);
        self.mouse_y = ((y / 4.0) as i32).max(0).min(179);
    }

    fn on_mouse_button_pressed(&mut self, button_id: u8) {
        println!("Mouse button pressed: {}", button_id);
    }

    fn on_mouse_button_released(&mut self, button_id: u8) {
        println!("Mouse button released: {}", button_id);
    }

    fn on_window_closed(&mut self) {
        let mut results = self.bench_results
            .iter()
            .filter(|&e| *e > 0.0001)
            .map(|e| *e)
            .collect::<Vec<_>>();
        results.sort_by(|lhs, rhs| if lhs < rhs {
            Ordering::Less
        } else if lhs > rhs {
            Ordering::Greater
        } else {
            Ordering::Equal
        });
        let max_time = results[results.len()-1];
        let min_time = results[0];
        let median = if results.len() % 2 != 0 {
            results[results.len()/2]
        } else {
            (results[results.len()/2] + results[results.len()/2-1]) / 2.0
        };
        let average: f32 = results
            .iter()
            .fold(
                0.0,
                |acc, &cur| acc + cur
            ) / results.len() as f32;
        println!("max_time: {}", max_time);
        println!("min_time: {}", min_time);
        println!("median: {}", median);
        println!("average: {}", average);
    }
}
impl Window {
    fn new() -> Self {
        Window{
            game_over: false,
            delta_instant: Instant::now(),
            mouse_x: 0,
            mouse_y: 0,
            bench_results: [0.0; 1024],
            cur_bench: 0,
            bump: Bump::new()
        }
    }
}
fn main() {
    start_pixel_window(
        Window::new(),
        WindowParameters{
            title: "Example windowing",
            window_width: 320,
            window_height: 180,
            fullscreen: false,
            scale_up: 4
        }
    );
}