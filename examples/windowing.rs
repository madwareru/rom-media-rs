use rom_media_rs::windowing::*;
use std::time::{Duration, Instant};
use rom_media_rs::image_rendering::brezenham::plot_brezenham;
use std::cmp::Ordering;

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
    mouse_x: i32,
    mouse_y: i32,
    bench_results: [f32; 1024],
    cur_bench: usize
}
impl PixelWindowHandler for Window {
    const FRAME_INTERVAL: Duration = Duration::from_micros(33000);
    fn update(&mut self) -> PixelWindowControlFlow {
        if self.game_over {
            PixelWindowControlFlow::Exit
        } else {
            PixelWindowControlFlow::Continue
        }
    }
    fn render(&mut self, buffer: &mut [u32], w: u16, _h: u16) {
        let (mut bottom, mut top) = ([0i32; 32], [0i32; 32]);
        let inst = Instant::now();
        for jj in 0..10 {
            let yy = 70 * jj + 5;
            for ii in 0..31 {
                let xx = 16 + ii * 32;
                for k in 0..32 {
                    bottom[k] = yy;
                    top[k] = yy + 70;
                }
                plot_brezenham(0, yy + 30i32, 31, yy, |x, y| {
                    top[x as usize] = y.min(top[x as usize]);
                });
                plot_brezenham(0, yy + 70, 31, yy + 70, |x, y| {
                    bottom[x as usize] = y.max(bottom[x as usize]);
                });
                for ix in xx..xx+32 {
                    let i = ix - xx;
                    let top_y = top[i];
                    let bottom_y = bottom[i];
                    plot_brezenham(top_y+1, 0, bottom_y, 31, |row, id| {
                        let idx = (row as usize) * w as usize + ix;
                        buffer[idx] = TEST_PAL[id as usize];
                    })
                }
            }
        }
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
        self.mouse_x = (x as i32).max(0).min(639);
        self.mouse_y = (y as i32).max(0).min(479);
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
            mouse_x: 0,
            mouse_y: 0,
            bench_results: [0.0; 1024],
            cur_bench: 0
        }
    }
}
fn main() {
    start_pixel_window(
        Window::new(),
        WindowParameters{
            title: "Example windowing",
            window_width: 1024,
            window_height: 768,
            fullscreen: false,
            scale_up: 1
        }
    );
}