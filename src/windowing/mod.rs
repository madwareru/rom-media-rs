use glutin::PossiblyCurrent;
use std::ffi::CStr;
use glutin::window::{WindowBuilder, Fullscreen};
use glutin::event_loop::{EventLoop, ControlFlow};
use glutin::event::{Event, WindowEvent, KeyboardInput, VirtualKeyCode, ElementState, MouseButton};
use glutin::ContextBuilder;
use glutin::dpi::{Size, LogicalSize};
use std::time::{Instant, Duration};
use super::gl;

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

macro_rules! check_shader {
    ($a:ident, $message:literal) => {
        let mut success = std::mem::zeroed();
        let mut info_log = [0u8;512];
        gl::GetShaderiv($a, gl::COMPILE_STATUS, &mut success);
        if success == gl::FALSE as i32 {
           gl::GetShaderInfoLog($a, 512, std::ptr::null_mut(), info_log.as_mut_ptr() as *mut i8);
           let mut error_string = String::new();
           for i in 0..512 {
               if info_log[i] == 0 {
                   break;
               }
               error_string.push(char::from(info_log[i]));
           }
           panic!("{} {}", $message, error_string);
        }
    }
}

const VERTS: [f32; 12] = [
    -1.0,  1.0, 0.0,
    -1.0, -1.0, 0.0,
     1.0,  1.0, 0.0,
     1.0, -1.0, 0.0
];
const TEX_COORDS: [f32; 8] = [
    0.0, 0.0,
    0.0, 1.0,
    1.0, 0.0,
    1.0, 1.0
];

const VERT_SRC: &[u8] = b"
#version 330 core
layout (location = 0) in vec3 position;
layout (location = 1) in vec2 uv;
out vec2 uv_coords;
void main()
{
    gl_Position = vec4(position.x, position.y, position.z, 1.0);
    uv_coords = uv;
}
\0";

const FRAG_SRC: &[u8] = b"
#version 330 core
in vec2 uv_coords;
out vec4 color;
uniform sampler2D main_texture;
void main()
{
    color = texture(main_texture, uv_coords).zyxw;
}
\0";

fn load(gl_context: &glutin::Context<PossiblyCurrent>) {
    let _ = gl::load_with(|ptr| gl_context.get_proc_address(ptr) as *const _);

    let version = unsafe {
        let data = CStr::from_ptr(gl::GetString(gl::VERSION) as *const _)
            .to_bytes()
            .to_vec();
        String::from_utf8(data).unwrap()
    };

    println!("OpenGL version {}", version);
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
    let actual_w = window_params.window_width * window_params.scale_up;
    let actual_h = window_params.window_height * window_params.scale_up;

    let mut texture_data = vec![
        0xFF000000u32;
        window_params.window_width as usize * window_params.window_height as usize
    ];

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

    let windowed_context = unsafe { windowed_context.make_current().unwrap() };
    let mut scale_factor = windowed_context.window().scale_factor();

    load(&windowed_context.context());

    let mut vbo_vertices = unsafe{
        let mut vbo = std::mem::zeroed();
        gl::GenBuffers(1, &mut vbo);
        vbo
    };

    let mut vbo_uv = unsafe{
        let mut vbo = std::mem::zeroed();
        gl::GenBuffers(1, &mut vbo);
        vbo
    };

    let shader_program = unsafe {
        let vs = gl::CreateShader(gl::VERTEX_SHADER);
        gl::ShaderSource(
            vs,
            1,
            [VERT_SRC.as_ptr() as *const _].as_ptr(),
            std::ptr::null(),
        );
        gl::CompileShader(vs);
        check_shader!(vs, "Error on vertex shader compile!");

        let fs = gl::CreateShader(gl::FRAGMENT_SHADER);
        gl::ShaderSource(
            fs,
            1,
            [FRAG_SRC.as_ptr() as *const _].as_ptr(),
            std::ptr::null(),
        );
        gl::CompileShader(fs);
        check_shader!(fs, "Error on fragment shader compile!");

        let program = gl::CreateProgram();
        gl::AttachShader(program, vs);
        gl::AttachShader(program, fs);
        gl::LinkProgram(program);

        gl::DeleteShader(vs);
        gl::DeleteShader(fs);

        program
    };

    let mut vao = unsafe{
        let mut vao = std::mem::zeroed();
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo_vertices);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (VERTS.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            VERTS.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );
        gl::VertexAttribPointer(
            0, 3, gl::FLOAT, gl::FALSE,
            (3 * std::mem::size_of::<gl::types::GLfloat>()) as i32,
            std::ptr::null_mut()
        );
        gl::EnableVertexAttribArray(0);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo_uv);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (TEX_COORDS.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            TEX_COORDS.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );
        gl::VertexAttribPointer(
            1, 2, gl::FLOAT, gl::FALSE,
            (2 * std::mem::size_of::<gl::types::GLfloat>()) as gl::types::GLsizei,
            std::ptr::null_mut()
        );
        gl::EnableVertexAttribArray(1);
        gl::BindVertexArray(0);
        vao
    };

    let mut texture = unsafe {
        let mut tex = std::mem::zeroed();
        gl::GenTextures(1, &mut tex);
        gl::BindTexture(gl::TEXTURE_2D, tex);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, 0x2600); // GL_NEAREST
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, 0x2600); // GL_NEAREST
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, 0x812F); // GL_CLAMP_TO_EDGE
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, 0x812F); // GL_CLAMP_TO_EDGE
        tex
    };

    let mut pbo = unsafe {
        let mut pbo = std::mem::zeroed();
        gl::GenBuffers(1, &mut pbo);
        gl::BindBuffer(gl::PIXEL_UNPACK_BUFFER, pbo);
        gl::BufferData(
            gl::PIXEL_UNPACK_BUFFER,
            (4 * texture_data.len()) as gl::types::GLsizeiptr,
            std::ptr::null_mut(),
            gl::STREAM_DRAW,
        );
        let ptr = gl::MapBuffer(gl::PIXEL_UNPACK_BUFFER, gl::WRITE_ONLY) as *mut u32;
        if !ptr.is_null() {
            let src = (&texture_data).as_ptr();
            std::ptr::copy_nonoverlapping(src, ptr, texture_data.len());
            gl::UnmapBuffer(gl::PIXEL_UNPACK_BUFFER);
        }
        gl::TexImage2D(
            gl::TEXTURE_2D, 0,
            gl::RGB as gl::types::GLsizei,
            window_params.window_width as gl::types::GLsizei,
            window_params.window_height as gl::types::GLsizei,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            std::ptr::null_mut()
        );
        gl::Finish();

        pbo
    };

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::LoopDestroyed => {
                unsafe {
                    gl::DeleteBuffers(1, &mut pbo);
                    gl::DeleteTextures(1, &mut texture);
                    gl::DeleteVertexArrays(1, &mut vao);
                    gl::DeleteProgram(shader_program);
                    gl::DeleteBuffers(1, &mut vbo_uv);
                    gl::DeleteBuffers(1, &mut vbo_vertices);
                }
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
                    let logical = position.to_logical(scale_factor);
                    win.on_mouse_moved(logical.x, logical.y);
                },
                _ => (),
            },
            Event::RedrawRequested(_) => {
                unsafe {
                    gl::BindBuffer(gl::PIXEL_UNPACK_BUFFER, pbo);
                    gl::BindTexture(gl::TEXTURE_2D, texture);

                    let ptr = gl::MapBuffer(gl::PIXEL_UNPACK_BUFFER, gl::WRITE_ONLY) as *mut u32;
                    if !ptr.is_null() {
                        let src = (&texture_data).as_ptr();
                        std::ptr::copy_nonoverlapping(src, ptr, texture_data.len());
                        gl::UnmapBuffer(gl::PIXEL_UNPACK_BUFFER);
                    }
                    gl::TexImage2D(
                        gl::TEXTURE_2D, 0,
                        gl::RGB as gl::types::GLsizei,
                        window_params.window_width as gl::types::GLsizei,
                        window_params.window_height as gl::types::GLsizei,
                        0,
                        gl::RGBA,
                        gl::UNSIGNED_BYTE,
                        std::ptr::null()
                    );
                    gl::Finish();
                    gl::ClearColor(0.5, 0.5, 0.5, 1.0);
                    gl::Clear(gl::COLOR_BUFFER_BIT);
                    gl::UseProgram(shader_program);
                    gl::BindTexture(gl::TEXTURE_2D, texture);
                    gl::BindVertexArray(vao);
                    gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);
                }
                windowed_context.swap_buffers().unwrap();
            },
            _ => {
                let instant = Instant::now();
                match win.update() {
                    PixelWindowControlFlow::Continue => {
                        win.render(&mut texture_data, window_params.window_width, window_params.window_height);
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