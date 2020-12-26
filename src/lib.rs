pub mod audio;
pub mod video;
pub mod windowing;
pub mod pixel_surface;
pub mod image_rendering;

#[cfg(not(feature = "use-wgpu"))]
mod opengl_wrappings;

#[cfg(not(feature = "use-wgpu"))]
pub mod gl { include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs")); }

#[cfg(not(feature = "use-wgpu"))]
pub mod graphics {
    pub type PixelSurface = super::pixel_surface::PixelSurfaceHolder<super::opengl_wrappings::OpenGlSurfaceImpl>;
    pub use super::opengl_wrappings::clear_background;
    pub use super::opengl_wrappings::init;
}

#[cfg(feature = "use-wgpu")]
mod wgpu_wrappings;

#[cfg(feature = "use-wgpu")]
pub mod graphics {
    pub type PixelSurface = super::pixel_surface::PixelSurfaceHolder<super::wgpu_wrappings::WgpuSurfaceImpl>;
    pub use super::wgpu_wrappings::clear_background;
    pub use super::wgpu_wrappings::init;
}