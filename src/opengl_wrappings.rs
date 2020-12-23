use glutin::{WindowedContext, NotCurrent, PossiblyCurrent};
use glutin::window::Window;
use super::gl;
use std::ffi::CStr;
use crate::pixel_surface::{PixelSurfaceImpl, PixelSurface};

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
uniform vec4 pos_scale;
out vec2 uv_coords;
void main()
{
    gl_Position = vec4(position.xy * pos_scale.zw + pos_scale.xy, position.z, 1.0);
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

pub struct OpenGlSurfaceHandle {
    pbo: gl::types::GLuint,
    vbo_vertices: gl::types::GLuint,
    vbo_uv: gl::types::GLuint,
    vao: gl::types::GLuint,
    shader_program: gl::types::GLuint,
    texture: gl::types::GLuint,
    main_tex_location: gl::types::GLint,
    pos_scale_location: gl::types::GLint,
    width: u16,
    height: u16
}

pub struct OpenGlSurfaceImpl;
impl PixelSurfaceImpl for OpenGlSurfaceImpl {
    type TextureHandle = OpenGlSurfaceHandle;

    fn create_texture_handle(width: u16, height: u16, initial_bytes: &[u32]) -> Self::TextureHandle {
        let vbo_vertices = unsafe {
            let mut vbo = std::mem::zeroed();
            gl::GenBuffers(1, &mut vbo);
            vbo
        };
        let vbo_uv = unsafe {
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

            gl::DetachShader(program, vs);
            gl::DeleteShader(vs);

            gl::DetachShader(program, fs);
            gl::DeleteShader(fs);

            program
        };
        let (main_tex_location, pos_scale_location) = unsafe{
            (
                gl::GetUniformLocation(
                    shader_program,
                    "main_texture\0".as_ptr() as *const i8
                ),
                gl::GetUniformLocation(
                    shader_program,
                    "pos_scale\0".as_ptr() as *const i8
                )
            )
        };
        let vao = unsafe {
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
        let texture = unsafe {
            let mut tex = std::mem::zeroed();
            gl::GenTextures(1, &mut tex);
            gl::BindTexture(gl::TEXTURE_2D, tex);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, 0x2600); // GL_NEAREST
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, 0x2600); // GL_NEAREST
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, 0x812F); // GL_CLAMP_TO_EDGE
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, 0x812F); // GL_CLAMP_TO_EDGE
            tex
        };
        let pbo = unsafe {
            let mut pbo = std::mem::zeroed();
            gl::GenBuffers(1, &mut pbo);
            gl::BindBuffer(gl::PIXEL_UNPACK_BUFFER, pbo);
            gl::BufferData(
                gl::PIXEL_UNPACK_BUFFER,
                (4 * initial_bytes.len()) as gl::types::GLsizeiptr,
                std::ptr::null_mut(),
                gl::STREAM_DRAW,
            );
            let ptr = gl::MapBuffer(gl::PIXEL_UNPACK_BUFFER, gl::WRITE_ONLY) as *mut u32;
            if !ptr.is_null() {
                let src = initial_bytes.as_ptr();
                std::ptr::copy_nonoverlapping(src, ptr, initial_bytes.len());
                gl::UnmapBuffer(gl::PIXEL_UNPACK_BUFFER);
            }
            gl::TexImage2D(
                gl::TEXTURE_2D, 0,
                gl::RGBA as gl::types::GLsizei,
                width as gl::types::GLsizei,
                height as gl::types::GLsizei,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                std::ptr::null_mut()
            );
            gl::Finish();

            pbo
        };
        Self::TextureHandle {
            pbo,
            vbo_vertices,
            vbo_uv,
            vao,
            shader_program,
            texture,
            main_tex_location,
            pos_scale_location,
            width,
            height
        }
    }

    fn stream(handle: &mut Self::TextureHandle, src: &[u32]) {
        unsafe {
            gl::BindBuffer(gl::PIXEL_UNPACK_BUFFER, handle.pbo);
            gl::BindTexture(gl::TEXTURE_2D, handle.texture);

            let ptr = gl::MapBuffer(gl::PIXEL_UNPACK_BUFFER, gl::WRITE_ONLY) as *mut u32;
            if !ptr.is_null() {
                let src_ptr = src.as_ptr();
                std::ptr::copy_nonoverlapping(src_ptr, ptr, src.len());
                gl::UnmapBuffer(gl::PIXEL_UNPACK_BUFFER);
            }
            gl::TexImage2D(
                gl::TEXTURE_2D, 0,
                gl::RGBA as gl::types::GLsizei,
                handle.width as gl::types::GLsizei,
                handle.height as gl::types::GLsizei,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                std::ptr::null()
            );
            gl::Finish();
        }
    }

    fn draw(handle: &Self::TextureHandle, x: f32, y: f32, scale_x: f32, scale_y: f32) {
        unsafe {
            gl::UseProgram(handle.shader_program);
            gl::Uniform1i(handle.main_tex_location, 0);
            gl::Uniform4f(handle.pos_scale_location, x, y, scale_x, scale_y);

            gl::ActiveTexture(gl::TEXTURE0 + 0);
            gl::BindTexture(gl::TEXTURE_2D, handle.texture);

            gl::BindVertexArray(handle.vao);
            gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);
        }
    }

    fn cleanup(handle: &mut Self::TextureHandle) {
        unsafe {
            gl::DeleteBuffers(1, &mut handle.pbo);
            gl::DeleteTextures(1, &mut handle.texture);
            gl::DeleteVertexArrays(1, &mut handle.vao);
            gl::DeleteProgram(handle.shader_program);
            gl::DeleteBuffers(1, &mut handle.vbo_uv);
            gl::DeleteBuffers(1, &mut handle.vbo_vertices);
        }
    }
}