use glutin::{WindowedContext, NotCurrent, PossiblyCurrent};
use glutin::window::Window;
use super::gl;
use std::ffi::CStr;

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

pub fn init_opengl_context(ctx: WindowedContext<NotCurrent>) -> WindowedContext<PossiblyCurrent> {
    let windowed_context = unsafe { ctx.make_current().unwrap() };
    load(&windowed_context.context());
    windowed_context
}

pub fn clear_background(r: f32, g: f32, b: f32, a: f32) {
    unsafe {
        gl::ClearColor(r, g, b, a);
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }
}