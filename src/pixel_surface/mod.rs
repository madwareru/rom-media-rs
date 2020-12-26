
pub trait PixelSurfaceImpl {
    type TextureHandle;
    fn create_texture_handle(width: u16, height: u16, initial_bytes: &[u32]) -> Self::TextureHandle;
    fn stream(handle: &mut Self::TextureHandle, src: &[u32]);
    fn draw(handle: &Self::TextureHandle, x: f32, y: f32, scale_x: f32, scale_y: f32);
    fn cleanup(handle: &mut Self::TextureHandle);
}

pub struct PixelSurfaceHolder<Impl : PixelSurfaceImpl> {
    handle: Impl::TextureHandle,
    pub width: u16,
    pub height: u16,
    pub bytes: Vec<u32>
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
    pub fn actualize_buffer(&mut self) {
        Impl::stream(&mut self.handle, &self.bytes)
    }
    pub fn draw(&self, x: f32, y: f32, scale_x: f32, scale_y: f32) {
        Impl::draw(&self.handle, x, y, scale_x, scale_y)
    }
    pub fn cleanup(&mut self) {
        Impl::cleanup(&mut self.handle)
    }
}