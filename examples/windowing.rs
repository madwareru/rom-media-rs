use rom_media_rs::windowing::*;
use std::time::Duration;

pub struct Window {
    game_over: bool,
}
impl PixelWindowHandler for Window {
    const TITLE: &'static str = "Example windowing";
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
                let r = (j+1) as f32 / h as f32;
                let g = (i+1) as f32 / w as f32;
                let r = (r * 255.0) as u32;
                let g = (g * 255.0) as u32;
                buffer[offset] = 0xFF000000 | g * 0x100 | r;
                offset += 1;
            }
        }
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
        println!("Mouse pos: ({}, {})", x, y);
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
        Window{ game_over: false, }
    }
}
fn main() {
    start_opengl_window::<Window>(
        Window::new(),
        WindowParameters{
            window_width: 640,
            window_height: 480,
            fullscreen: false,
            scale_up: 1
        }
    );
}