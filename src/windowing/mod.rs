use std::time::{Duration};

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

pub enum PixelWindowControlFlow {
    Continue,
    Exit
}

pub trait PixelWindowHandler: 'static {
    const FRAME_INTERVAL: Duration;
    fn create(window_params: &WindowParameters) -> Self;
    fn update(&mut self) -> PixelWindowControlFlow;
    fn prerender(&mut self);
    fn render(&mut self);
    fn on_key_pressed(&mut self, key: Key);
    fn on_key_released(&mut self, key: Key);
    fn on_mouse_moved(&mut self, x: f64, y: f64);
    fn on_mouse_button_pressed(&mut self, button_id: u16);
    fn on_mouse_button_released(&mut self, button_id: u16);
    fn on_window_closed(&mut self);
    fn cleanup(&mut self);
}

pub struct WindowParameters {
    pub title: &'static str,
    pub window_width: u16,
    pub window_height: u16,
    pub scale_up: u16,
    pub fullscreen: bool
}

#[cfg(not(feature = "use-wgpu"))]
pub mod glutin_window;

#[cfg(not(feature = "use-wgpu"))]
pub use glutin_window::start_pixel_window;

#[cfg(not(feature = "use-wgpu"))]
use glutin::event::VirtualKeyCode;

#[cfg(not(feature = "use-wgpu"))]
impl Key {
    pub(crate) fn map_from_keycode(key_code: VirtualKeyCode) -> Self {
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

#[cfg(feature = "use-wgpu")]
pub mod wgpu_window;

#[cfg(feature = "use-wgpu")]
pub use wgpu_window::start_pixel_window;

#[cfg(feature = "use-wgpu")]
use glutin::event::VirtualKeyCode;

#[cfg(feature = "use-wgpu")]
impl Key {
    pub(crate) fn map_from_keycode(key_code: VirtualKeyCode) -> Self {
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