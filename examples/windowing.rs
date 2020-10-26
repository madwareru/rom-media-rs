use rom_media_rs::windowing::*;
use std::time::Duration;
use rom_media_rs::image_rendering::brezenham::plot_brezenham;

pub struct Window {
    game_over: bool,
    mouse_x: i32,
    mouse_y: i32,
}
impl PixelWindowHandler for Window {
    const FRAME_INTERVAL: Duration = Duration::from_micros(16667);
    fn update(&mut self) -> PixelWindowControlFlow {
        if self.game_over {
            PixelWindowControlFlow::Exit
        } else {
            PixelWindowControlFlow::Continue
        }
    }
    fn render(&mut self, buffer: &mut [u32], w: u16, h: u16) {
        let mut offset = 0;
        for j in 0..h {
            for i in 0..w {
                let b = (j+1) as f32 / h as f32;
                let g = (i+1) as f32 / w as f32;
                let b = (b * 255.0) as u32;
                let g = (g * 255.0) as u32;
                buffer[offset] = 0xFF000000 | g * 0x100 | b;
                offset += 1;
            }
        }
        plot_brezenham(320, 240, self.mouse_x, self.mouse_y, |x, y| {
            let idx = y as usize * w as usize + x as usize;
            buffer[idx] |= 0x00FF0000;
        })
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
        self.mouse_y = (y as i32).max(0).min(639);
    }

    fn on_mouse_button_pressed(&mut self, button_id: u8) {
        println!("Mouse button pressed: {}", button_id);
    }

    fn on_mouse_button_released(&mut self, button_id: u8) {
        println!("Mouse button released: {}", button_id);
    }
}
impl Window {
    fn new() -> Self {
        Window{ game_over: false, mouse_x: 0, mouse_y: 0 }
    }
}
fn main() {
    start_pixel_window(
        Window::new(),
        WindowParameters{
            title: "Example windowing",
            window_width: 640,
            window_height: 480,
            fullscreen: false,
            scale_up: 1
        }
    );
}