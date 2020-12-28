use std::ops::{Deref, DerefMut};

pub trait PixelSurfaceImpl {
    type TextureHandle;
    fn create_texture_handle(width: u16, height: u16, initial_bytes: &[u32]) -> Self::TextureHandle;
    fn stream(handle: &mut Self::TextureHandle, src: &[u32]);
    fn draw(handle: &Self::TextureHandle, x: f32, y: f32, scale_x: f32, scale_y: f32);
    fn cleanup(handle: &mut Self::TextureHandle);
}

pub struct PixelSurfaceHolder<Impl : PixelSurfaceImpl> {
    handle: Impl::TextureHandle,
    width: u16,
    height: u16,
    bytes: Vec<u32>
}

impl<Impl : PixelSurfaceImpl> PixelSurfaceHolder<Impl> {
    pub fn create(width: u16, height: u16) -> Self {
        let len = width as usize * height as usize;
        let mut bytes = Vec::with_capacity(len);
        bytes.resize(len, 0xFF000000);
        Self {
            handle: Impl::create_texture_handle(width, height, &bytes),
            width,
            height,
            bytes
        }
    }

    pub fn get_buffer_mut(&mut self) -> PixelSurfaceMutableReference<Impl> {
        PixelSurfaceMutableReference { holder: self }
    }

    pub fn get_buffer(&mut self) -> PixelSurfaceImmutableReference<Impl> {
        PixelSurfaceImmutableReference { holder: self }
    }

    pub fn draw(&self, x: f32, y: f32, scale_x: f32, scale_y: f32) {
        Impl::draw(&self.handle, x, y, scale_x, scale_y)
    }
}

impl<Impl : PixelSurfaceImpl> Drop for PixelSurfaceHolder<Impl> {
    fn drop(&mut self) {
        Impl::cleanup(&mut self.handle)
    }
}

pub struct PixelSurfaceMutableReference<'a, Impl: PixelSurfaceImpl> {
    holder: &'a mut PixelSurfaceHolder<Impl>
}

impl<'a, Impl: PixelSurfaceImpl> Deref for PixelSurfaceMutableReference<'a, Impl> {
    type Target = [u32];

    fn deref(&self) -> &Self::Target {
        &self.holder.bytes
    }
}

impl<'a, Impl: PixelSurfaceImpl> DerefMut for PixelSurfaceMutableReference<'a, Impl> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.holder.bytes
    }
}

impl<'a, Impl: PixelSurfaceImpl> Drop for PixelSurfaceMutableReference<'a, Impl> {
    fn drop(&mut self) {
        Impl::stream(&mut self.holder.handle, &self.holder.bytes)
    }
}

impl<'a, Impl: PixelSurfaceImpl> PixelSurfaceMutableReference<'a, Impl> {
    pub fn width(&self) -> usize {
        self.holder.width as usize
    }
    pub fn height(&self) -> usize {
        self.holder.height as usize
    }
}

pub struct PixelSurfaceImmutableReference<'a, Impl: PixelSurfaceImpl> {
    holder: &'a PixelSurfaceHolder<Impl>
}

impl<'a, Impl: PixelSurfaceImpl> Deref for PixelSurfaceImmutableReference<'a, Impl> {
    type Target = [u32];

    fn deref(&self) -> &Self::Target {
        &self.holder.bytes
    }
}

impl<'a, Impl: PixelSurfaceImpl> PixelSurfaceImmutableReference<'a, Impl> {
    pub fn width(&self) -> usize {
        self.holder.width as usize
    }
    pub fn height(&self) -> usize {
        self.holder.height as usize
    }
}