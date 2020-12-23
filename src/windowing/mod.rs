use glutin::PossiblyCurrent;
use std::ffi::CStr;
use glutin::window::{WindowBuilder, Fullscreen};
use glutin::event_loop::{EventLoop, ControlFlow};
use glutin::event::{Event, WindowEvent, KeyboardInput, VirtualKeyCode, ElementState, MouseButton};
use glutin::ContextBuilder;
use glutin::dpi::{Size, LogicalSize};
use std::time::{Instant, Duration};
use super::gl;
use crate::pixel_surface::{PixelSurface};
use crate::opengl_wrappings::{init_opengl_context, clear_background, OpenGlSurfaceImpl};

#[derive(Debug, Hash, PartialEq, Clone, Copy)]
pub enum Key {
    Digit(u32),
    Numpad(u32),
    Functional(u32),
    Letter(char),
    Escape,
    Scroll,
    Pause,
    Insert,
    Home,
    Delete,
    End,
    PageDown,
    PageUp,
    Left,
    Up,
    Right,
    Down,
    Backspace,
    Return,
    Space,
    Compose,
    Caret,
    Numlock,
    NumpadAdd,
    NumpadDivide,
    NumpadDecimal,
    NumpadComma,
    NumpadEnter,
    NumpadEquals,
    NumpadMultiply,
    NumpadSubtract,
    AbntC1,
    AbntC2,
    Apostrophe,
    Apps,
    Asterisk,
    At,
    Ax,
    Backslash,
    Calculator,
    Capital,
    Colon,
    Comma,
    Convert,
    Equals,
    Grave,
    Kana,
    Kanji,
    LAlt,
    LBracket,
    LControl,
    LShift,
    LWin,
    Mail,
    MediaSelect,
    MediaStop,
    Minus,
    Mute,
    MyComputer,
    NavigateForward,
    NavigateBackward,
    NextTrack,
    NoConvert,
    OEM102,
    Period,
    PlayPause,
    Plus,
    Power,
    PrevTrack,
    RAlt,
    RBracket,
    RControl,
    RShift,
    RWin,
    Semicolon,
    Slash,
    Sleep,
    Stop,
    Sysrq,
    Tab,
    Underline,
    Unlabeled,
    VolumeDown,
    VolumeUp,
    Wake,
    WebBack,
    WebFavorites,
    WebForward,
    WebHome,
    WebRefresh,
    WebSearch,
    WebStop,
    Yen,
    Copy,
    Paste,
    Cut,
    Unknown
}
impl Key {
    fn map_from_keycode(key_code: VirtualKeyCode) -> Self {
        if key_code >= VirtualKeyCode::Key1 && key_code <= VirtualKeyCode::Key9 {
            Key::Digit(key_code as u32 - VirtualKeyCode::Key1 as u32 + 1)
        } else if key_code == VirtualKeyCode::Key0 {
            Key::Digit(0)
        } else if key_code >= VirtualKeyCode::F1 && key_code <= VirtualKeyCode::F24 {
            Key::Digit(key_code as u32 - VirtualKeyCode::F1 as u32 + 1)
        } else if key_code >= VirtualKeyCode::A && key_code <= VirtualKeyCode::Z {
            let order = (key_code as u32 - VirtualKeyCode::A as u32) as u8;
            Key::Letter(char::from(b'A' + order))
        } else if key_code >= VirtualKeyCode::Numpad0 && key_code <= VirtualKeyCode::Numpad9 {
            Key::Numpad(key_code as u32 - VirtualKeyCode::Numpad0 as u32)
        } else {
            match key_code {
                VirtualKeyCode::Escape => Key::Escape,
                VirtualKeyCode::Scroll => Key::Scroll,
                VirtualKeyCode::Pause => Key::Pause,
                VirtualKeyCode::Insert => Key::Insert,
                VirtualKeyCode::Home => Key::Home,
                VirtualKeyCode::Delete => Key::Delete,
                VirtualKeyCode::End => Key::End,
                VirtualKeyCode::PageDown => Key::PageDown,
                VirtualKeyCode::PageUp => Key::PageUp,
                VirtualKeyCode::Left => Key::Left,
                VirtualKeyCode::Up => Key::Up,
                VirtualKeyCode::Right => Key::Right,
                VirtualKeyCode::Down => Key::Down,
                VirtualKeyCode::Back => Key::Backspace,
                VirtualKeyCode::Return => Key::Return,
                VirtualKeyCode::Space => Key::Space,
                VirtualKeyCode::Compose => Key::Compose,
                VirtualKeyCode::Caret => Key::Caret,
                VirtualKeyCode::Numlock => Key::Numlock,
                VirtualKeyCode::NumpadAdd => Key::NumpadAdd,
                VirtualKeyCode::NumpadDivide => Key::NumpadDivide,
                VirtualKeyCode::NumpadDecimal => Key::NumpadDecimal,
                VirtualKeyCode::NumpadComma => Key::NumpadComma,
                VirtualKeyCode::NumpadEnter => Key::NumpadEnter,
                VirtualKeyCode::NumpadEquals => Key::NumpadEquals,
                VirtualKeyCode::NumpadMultiply => Key::NumpadMultiply,
                VirtualKeyCode::NumpadSubtract => Key::NumpadSubtract,
                VirtualKeyCode::AbntC1 => Key::AbntC1,
                VirtualKeyCode::AbntC2 => Key::AbntC2,
                VirtualKeyCode::Apostrophe => Key::Apostrophe,
                VirtualKeyCode::Apps => Key::Apps,
                VirtualKeyCode::Asterisk => Key::Asterisk,
                VirtualKeyCode::At => Key::At,
                VirtualKeyCode::Ax => Key::Ax,
                VirtualKeyCode::Backslash => Key::Backslash,
                VirtualKeyCode::Calculator => Key::Calculator,
                VirtualKeyCode::Capital => Key::Capital,
                VirtualKeyCode::Colon => Key::Colon,
                VirtualKeyCode::Comma => Key::Comma,
                VirtualKeyCode::Convert => Key::Convert,
                VirtualKeyCode::Equals => Key::Equals,
                VirtualKeyCode::Grave => Key::Grave,
                VirtualKeyCode::Kana => Key::Kana,
                VirtualKeyCode::Kanji => Key::Kanji,
                VirtualKeyCode::LAlt => Key::LAlt,
                VirtualKeyCode::LBracket => Key::LBracket,
                VirtualKeyCode::LControl => Key::LControl,
                VirtualKeyCode::LShift => Key::LShift,
                VirtualKeyCode::LWin => Key::LWin,
                VirtualKeyCode::Mail => Key::Mail,
                VirtualKeyCode::MediaSelect => Key::MediaSelect,
                VirtualKeyCode::MediaStop => Key::MediaStop,
                VirtualKeyCode::Minus => Key::Minus,
                VirtualKeyCode::Mute => Key::Mute,
                VirtualKeyCode::MyComputer => Key::MyComputer,
                VirtualKeyCode::NavigateForward => Key::NavigateForward,
                VirtualKeyCode::NavigateBackward => Key::NavigateBackward,
                VirtualKeyCode::NextTrack => Key::NextTrack,
                VirtualKeyCode::NoConvert => Key::NoConvert,
                VirtualKeyCode::OEM102 => Key::OEM102,
                VirtualKeyCode::Period => Key::Period,
                VirtualKeyCode::PlayPause => Key::PlayPause,
                VirtualKeyCode::Plus => Key::Plus,
                VirtualKeyCode::Power => Key::Power,
                VirtualKeyCode::PrevTrack => Key::PrevTrack,
                VirtualKeyCode::RAlt => Key::RAlt,
                VirtualKeyCode::RBracket => Key::RBracket,
                VirtualKeyCode::RControl => Key::RControl,
                VirtualKeyCode::RShift => Key::RShift,
                VirtualKeyCode::RWin => Key::RWin,
                VirtualKeyCode::Semicolon => Key::Semicolon,
                VirtualKeyCode::Slash => Key::Slash,
                VirtualKeyCode::Sleep => Key::Sleep,
                VirtualKeyCode::Stop => Key::Stop,
                VirtualKeyCode::Sysrq => Key::Sysrq,
                VirtualKeyCode::Tab => Key::Tab,
                VirtualKeyCode::Underline => Key::Underline,
                VirtualKeyCode::Unlabeled => Key::Unlabeled,
                VirtualKeyCode::VolumeDown => Key::VolumeDown,
                VirtualKeyCode::VolumeUp => Key::VolumeUp,
                VirtualKeyCode::Wake => Key::Wake,
                VirtualKeyCode::WebBack => Key::WebBack,
                VirtualKeyCode::WebFavorites => Key::WebFavorites,
                VirtualKeyCode::WebForward => Key::WebForward,
                VirtualKeyCode::WebHome => Key::WebHome,
                VirtualKeyCode::WebRefresh => Key::WebRefresh,
                VirtualKeyCode::WebSearch => Key::WebSearch,
                VirtualKeyCode::WebStop => Key::WebStop,
                VirtualKeyCode::Yen => Key::Yen,
                VirtualKeyCode::Copy => Key::Copy,
                VirtualKeyCode::Paste => Key::Paste,
                VirtualKeyCode::Cut => Key::Cut,
                _ => Key::Unknown
            }
        }
    }
}

pub enum PixelWindowControlFlow {
    Continue,
    Exit
}

pub trait PixelWindowHandler: 'static {
    const FRAME_INTERVAL: Duration;
    fn update(&mut self) -> PixelWindowControlFlow;
    fn render(&mut self, buffer: &mut [u32], w: u16, h: u16);
    fn on_key_pressed(&mut self, key: Key);
    fn on_key_released(&mut self, key: Key);
    fn on_mouse_moved(&mut self, x: f64, y: f64);
    fn on_mouse_button_pressed(&mut self, button_id: u8);
    fn on_mouse_button_released(&mut self, button_id: u8);
    fn on_window_closed(&mut self);
}

pub struct WindowParameters {
    pub title: &'static str,
    pub window_width: u16,
    pub window_height: u16,
    pub scale_up: u16,
    pub fullscreen: bool
}

pub fn start_pixel_window<W: PixelWindowHandler>(window: W, window_params: WindowParameters) {
    let mut win = window;
    let actual_w = window_params.window_width * window_params.scale_up.max(1);
    let actual_h = window_params.window_height * window_params.scale_up.max(1);

    let event_loop = EventLoop::new();
    let window_builder = match window_params.fullscreen {
        true => {
            let primary_monitor = event_loop.primary_monitor();
            match primary_monitor {
                None => {WindowBuilder::new()
                    .with_title(window_params.title)
                    .with_resizable(false)
                    .with_inner_size(Size::Logical(
                        LogicalSize::new(
                            actual_w as f64,
                            actual_h as f64
                        )
                    ))
                }
                Some(monitor) => {
                    let video_mode = monitor.video_modes().find(|mode|
                        mode.size().width == actual_w as u32 &&
                        mode.size().height == actual_h as u32
                    ).or(monitor.video_modes().find(|mode|
                        mode.size().width == actual_w as u32
                    )).unwrap(); // fail if not found by design
                    WindowBuilder::new()
                        .with_title(window_params.title)
                        .with_resizable(false)
                        .with_fullscreen(Some(Fullscreen::Exclusive(video_mode)))
                }
            }
        }
        false => WindowBuilder::new()
            .with_title(window_params.title)
            .with_resizable(false)
            .with_inner_size(Size::Logical(
                LogicalSize::new(
                    actual_w as f64,
                    actual_h as f64
                )
            ))
    };

    let windowed_context = ContextBuilder::new()
        .build_windowed(window_builder, &event_loop)
        .unwrap();

    let windowed_context = init_opengl_context(windowed_context);
    let mut scale_factor = windowed_context.window().scale_factor();

    let mut surface = PixelSurface::<OpenGlSurfaceImpl>::create(
        window_params.window_width,
        window_params.window_height
    );

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::LoopDestroyed => {
                win.on_window_closed();
                surface.cleanup();
                return
            },
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::ScaleFactorChanged { scale_factor: factor, .. } => {
                    scale_factor = factor;
                },
                WindowEvent::Resized(physical_size) => {
                    windowed_context.resize(physical_size)
                },
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit
                },
                WindowEvent::KeyboardInput {input, ..} => {
                    match input {
                        KeyboardInput { state, virtual_keycode, .. } => {
                            match (state, virtual_keycode) {
                                (ElementState::Pressed, Some(keycode)) => win.on_key_pressed(Key::map_from_keycode(keycode)),
                                (ElementState::Released, Some(keycode)) => win.on_key_released(Key::map_from_keycode(keycode)),
                                _ => ()
                            }
                        },
                    }
                },
                WindowEvent::MouseInput { state, button, .. } => {
                    match state {
                        ElementState::Pressed => win.on_mouse_button_pressed(match button {
                            MouseButton::Left => 0,
                            MouseButton::Right => 2,
                            MouseButton::Middle => 1,
                            MouseButton::Other(id) => id
                        }),
                        ElementState::Released => win.on_mouse_button_released(match button {
                            MouseButton::Left => 0,
                            MouseButton::Right => 2,
                            MouseButton::Middle => 1,
                            MouseButton::Other(id) => id
                        }),
                    }
                },
                WindowEvent::CursorMoved { position, .. } => {
                    let logical: glutin::dpi::LogicalPosition<f64> = position.to_logical(scale_factor);
                    let mouse_x = logical.x;
                    let mouse_y = logical.y;
                    win.on_mouse_moved(
                        mouse_x / (window_params.scale_up as f64),
                        mouse_y / (window_params.scale_up as f64)
                    );
                },
                _ => (),
            },
            Event::RedrawRequested(_) => {
                surface.actualize_buffer();
                clear_background(0.0, 0.0, 0.0, 1.0);
                surface.draw(0.0, 0.0, 1.0, 1.0);
                windowed_context.swap_buffers().unwrap();
            },
            _ => {
                let instant = Instant::now();
                match win.update() {
                    PixelWindowControlFlow::Continue => {
                        win.render(&mut surface.bytes, window_params.window_width, window_params.window_height);
                        windowed_context.window().request_redraw();
                        *control_flow = ControlFlow::WaitUntil(instant + W::FRAME_INTERVAL);
                    }
                    PixelWindowControlFlow::Exit => {
                        *control_flow = ControlFlow::Exit
                    }
                }
            },
        }
    });
}