use rom_loaders_rs::images::sprite::BmpSprite;
use crate::image_rendering::blittable::{Blittable, Rect};

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

    fn components_per_pixel(&self) -> usize {
        1
    }
}