use rom_media_rs::windowing::*;
use std::time::{Duration, Instant};
use rom_media_rs::image_rendering::bresenham::{ plot_bresenham_4d};
use rom_media_rs::graphics::PixelSurface;
use std::cmp::Ordering;
use rom_media_rs::image_rendering::triangles::{draw_brezenham_triangles};
use bumpalo::Bump;

pub struct Window {
    game_over: bool,
    delta_instant: Instant,
    mouse_x: i32,
    mouse_y: i32,
    bench_results: [f32; 1024],
    cur_bench: usize,
    bump: Bump,
    surface: PixelSurface
}
impl PixelWindowHandler for Window {
    const FRAME_INTERVAL: Duration = Duration::from_micros(33000);

    fn create(window_params: &WindowParameters) -> Self {
        let surface = PixelSurface::create(
            window_params.window_width,
            window_params.window_height
        );

        Self{
            game_over: false,
            delta_instant: Instant::now(),
            mouse_x: 0,
            mouse_y: 0,
            bench_results: [0.0; 1024],
            cur_bench: 0,
            bump: Bump::new(),
            surface
        }
    }

    fn update(&mut self) -> PixelWindowControlFlow {
        if self.game_over {
            PixelWindowControlFlow::Exit
        } else {
            let _dt = self.delta_instant.elapsed().as_millis() as f32 / 1_000.0;
            self.delta_instant = Instant::now();

            PixelWindowControlFlow::Continue
        }
    }
    fn prerender(&mut self) {
        let buffer = &mut self.surface.bytes;

        let w = self.surface.width as usize;
        let h = self.surface.height as usize;

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
            w, w * h,
            |idx, (u, v)| {
                let checker_board = (((u / 16) & 0x01) ^ ((v / 16) & 0x01)) as u32;
                let r = u as u32 * checker_board;
                let g = v as u32 * checker_board;
                buffer[idx] = 0xFF_00_00_00 | r * 0x1_00_00 | g * 0x1_00;
            }
        );

        plot_bresenham_4d(
            w as i32 / 2, h as i32 / 2, 0, 0,
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

        self.surface.actualize_buffer();
    }

    fn render(&mut self) {
        self.surface.draw(0.0, 0.0, 1.0, 1.0);
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
        self.mouse_x = (x as i32).max(0).min(319);
        self.mouse_y = (y as i32).max(0).min(179);
    }

    fn on_mouse_button_pressed(&mut self, button_id: u16) {
        println!("Mouse button pressed: {}", button_id);
    }

    fn on_mouse_button_released(&mut self, button_id: u16) {
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
fn main() {
    start_pixel_window::<Window>(
        WindowParameters{
            title: "Example windowing",
            window_width: 320,
            window_height: 180,
            fullscreen: true,
            scale_up: 4,
            cursor_visible: false
        }
    );
}