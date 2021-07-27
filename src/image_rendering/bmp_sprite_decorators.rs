use rom_loaders_rs::images::sprite::BmpSprite;
use crate::image_rendering::blittable::{Blittable, Rect, BlitDestination, BlitBuilder};

pub struct ColorKeyedBmp {
    color_key: u32,
    decorated: BmpSprite
}

impl ColorKeyedBmp {
    pub fn new(decorated: BmpSprite, color_key: u32) -> Self {
        Self { color_key, decorated }
    }
}

impl Blittable<u32> for ColorKeyedBmp {
    fn blit_impl(&self, buffer: &mut [u32], buffer_width: usize, self_rect: Rect, dst_rect: Rect) {
        let src_rect = self_rect;
        let dst_rect = dst_rect;
        let span_length = (
            src_rect.x_range.end - src_rect.x_range.start
        ).min(
            dst_rect.x_range.end - dst_rect.x_range.start
        );
        let span_count = (
            src_rect.y_range.end - src_rect.y_range.start
        ).min(
            dst_rect.y_range.end - dst_rect.y_range.start
        );
        match &self.decorated {
            BmpSprite::Paletted { width, palette, palette_indexes, .. } => {
                let src = palette_indexes.as_ptr();
                let dst = buffer.as_mut_ptr();

                let mut src_stride = src_rect.y_range.start * *width + src_rect.x_range.start;
                let mut dst_stride = dst_rect.y_range.start * buffer_width + dst_rect.x_range.start;
                for _ in 0..span_count {
                    unsafe {
                        let mut src_entry = src;
                        src_entry = src_entry.add(src_stride);
                        let mut dst_entry = dst;
                        dst_entry = dst_entry.add(dst_stride);
                        for _ in 0..span_length {
                            let idx = *src_entry;
                            let color = palette[idx as usize];
                            if color != self.color_key {
                                *dst_entry = color;
                            }
                            src_entry = src_entry.add(1);
                            dst_entry = dst_entry.add(1);
                        }
                    }
                    src_stride += *width;
                    dst_stride += buffer_width;
                }
            }
            BmpSprite::TrueColor { width, colors, .. } => {
                let src = colors.as_ptr();
                let dst = buffer.as_mut_ptr();

                let mut src_stride = src_rect.y_range.start * *width + src_rect.x_range.start;
                let mut dst_stride = dst_rect.y_range.start * buffer_width + dst_rect.x_range.start;
                for _ in 0..span_count {
                    unsafe {
                        let mut src_entry = src;
                        src_entry = src_entry.add(src_stride);
                        let mut dst_entry = dst;
                        dst_entry = dst_entry.add(dst_stride);
                        for _ in 0..span_length {
                            let color = *src_entry;
                            if color != self.color_key {
                                *dst_entry = color;
                            }
                            src_entry = src_entry.add(1);
                            dst_entry = dst_entry.add(1);
                        }
                    }
                    src_stride += *width;
                    dst_stride += buffer_width;
                }
            }
            BmpSprite::NotSupported => ()
        }
    }

    fn get_width(&self) -> usize {
        self.decorated.get_width()
    }

    fn get_height(&self) -> usize {
        self.decorated.get_height()
    }
}

pub struct TrueColorSurfaceSprite(BmpSprite);
impl TrueColorSurfaceSprite {
    pub fn new(width: usize, height: usize) -> Self {
        let mut buffer = Vec::with_capacity(width * height);
        buffer.resize(width * height, 0);
        Self(BmpSprite::TrueColor {width, height, colors: buffer })
    }
    pub fn color_data(&self) -> &[u32] {
        if let BmpSprite::TrueColor { colors, .. } = &self.0 {
            return &colors[..];
        }
        unreachable!("Something very bad happened")
    }
}

pub struct AlphaBlendedSprite<'a> {
    decorated: &'a TrueColorSurfaceSprite,
    amount: i64,
    count: i64
}

impl<'a> Blittable<u32> for AlphaBlendedSprite<'a> {
    fn blit_impl(&self, buffer: &mut [u32], buffer_width: usize, self_rect: Rect, dst_rect: Rect) {
        let src_rect = self_rect;
        let dst_rect = dst_rect;
        let span_length = (
            src_rect.x_range.end - src_rect.x_range.start
        ).min(
            dst_rect.x_range.end - dst_rect.x_range.start
        );
        let span_count = (
            src_rect.y_range.end - src_rect.y_range.start
        ).min(
            dst_rect.y_range.end - dst_rect.y_range.start
        );
        match &self.decorated.0 {
            BmpSprite::TrueColor { width, colors, .. } => {
                let mut src_stride = src_rect.y_range.start * *width + src_rect.x_range.start;
                let mut dst_stride = dst_rect.y_range.start * buffer_width + dst_rect.x_range.start;
                for _ in 0..span_count {
                    let zipped = (&mut buffer[dst_stride..dst_stride+span_length])
                        .iter_mut()
                        .zip(&colors[src_stride..src_stride+span_length]);
                    for (dest, src) in zipped {
                        let mut src_color = *src;
                        let mut dst_color = *dest;

                        *dest = 0;

                        for _ in 0..4 {
                            *dest = *dest * 0x100;

                            let d = (dst_color & 0xFF) as i64; dst_color = dst_color / 0x100;
                            let s = (src_color & 0xFF) as i64; src_color = src_color / 0x100;

                            *dest += ((d * (self.count - self.amount) + s * self.amount) / self.count) as u32;

                        }
                    }
                    src_stride += *width;
                    dst_stride += buffer_width;
                }
            },
            _ => ()
        }
    }

    fn get_width(&self) -> usize {
        self.decorated.get_width()
    }

    fn get_height(&self) -> usize {
        self.decorated.get_height()
    }
}

impl Blittable<u32> for TrueColorSurfaceSprite {
    fn blit_impl(&self, buffer: &mut [u32], buffer_width: usize, self_rect: Rect, dst_rect: Rect) {
        self.0.blit_impl(buffer, buffer_width, self_rect, dst_rect)
    }

    fn get_width(&self) -> usize {
        self.0.get_width()
    }

    fn get_height(&self) -> usize {
        self.0.get_height()
    }
}

impl<'a, TBlittable: Blittable<u32>> BlitDestination<'a, u32, TBlittable> for TrueColorSurfaceSprite {
    fn try_initiate_blit_on_self(
        &'a mut self, source_blittable: &'a TBlittable
    ) -> Option<BlitBuilder<'a, u32, TBlittable>> {
        self.0.try_initiate_blit_on_self(source_blittable)
    }
}