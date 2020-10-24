use rom_loaders_rs::images::sprite::BmpSprite;
use std::ops::Range;

pub struct Rect {
    pub x_range: Range<usize>,
    pub y_range: Range<usize>
}

pub struct DrawableRenderBuilder<'a, TDrawable: Drawable> {
    drawable: &'a TDrawable,
    buffer: &'a mut [u32],
    buffer_width: usize,
    src_x: usize,
    src_y: usize,
    src_width: usize,
    src_height: usize,
    dst_x: i32,
    dst_y: i32,
    dst_width: usize,
    dst_height: usize
}
impl<'a, TDrawable: Drawable> DrawableRenderBuilder<'a, TDrawable> {
    pub fn new(buffer: &'a mut [u32], buffer_width: usize, drawable: &'a TDrawable) -> Self {
        let dst_height = buffer.len() / buffer_width;
        Self {
            drawable,
            buffer,
            buffer_width,
            src_x: 0,
            src_y: 0,
            src_width: drawable.get_width(),
            src_height: drawable.get_height(),
            dst_x: 0,
            dst_y: 0,
            dst_width: buffer_width,
            dst_height
        }
    }
    pub fn with_dest_pos(self, dst_x: i32, dst_y: i32) -> Self {
        Self {
            dst_x,
            dst_y,
            ..self
        }
    }
    pub fn with_source_subrect(self, src_x: usize, src_y: usize, src_width: usize, src_height: usize) -> Self {
        Self {
            src_x,
            src_y,
            src_width,
            src_height,
            ..self
        }
    }
    pub fn with_dest_subrect(self, dst_x: i32, dst_y: i32, dst_width: usize, dst_height: usize) -> Self {
        Self {
            dst_x,
            dst_y,
            dst_width,
            dst_height,
            ..self
        }
    }
    pub fn render(&mut self) {
        draw_ext(
            self.drawable,
            self.buffer,
            self.buffer_width,
            self.src_x,
            self.src_y,
            self.src_width,
            self.src_height,
            self.dst_x,
            self.dst_y,
            self.dst_width,
            self.dst_height
        )
    }
}

fn draw_ext<TDrawable: Drawable>(drawable: &TDrawable, buffer: &mut [u32], buffer_width: usize,
            src_x: usize, src_y: usize,
            src_width: usize, src_height: usize,
            dst_x: i32, dst_y: i32,
            dst_width: usize, dst_height: usize
) {
    let src_width_max = (src_width + src_x).min(drawable.get_width());
    let src_height_max = (src_height + src_y).min(drawable.get_height());

    let dst_width_max = ((dst_width as i32 + dst_x) as usize).min(buffer_width);
    let dst_height_max = ((dst_height as i32 + dst_y) as usize).min(buffer.len() / buffer_width);

    let mut src_rect = Rect {
        x_range: src_x.min(src_width_max)..src_width_max,
        y_range: src_y.min(src_height_max)..src_height_max
    };
    let mut dst_rect = Rect{
        x_range: 0..dst_width_max,
        y_range: 0..dst_height_max
    };

    if dst_x < 0 {
        src_rect.x_range.start = (src_rect.x_range.start + (-dst_x) as usize)
            .min(src_rect.x_range.end);
    } else {
        dst_rect.x_range.start = ((dst_rect.x_range.start as i32 + dst_x) as usize)
            .min(dst_rect.x_range.end);
    }
    if dst_y < 0 {
        src_rect.y_range.start = (src_rect.y_range.start + (-dst_y) as usize)
            .min(src_rect.y_range.end);
    } else {
        dst_rect.y_range.start = ((dst_rect.y_range.start as i32 + dst_y) as usize)
            .min(dst_rect.y_range.end);
    }

    drawable.draw_impl(
        buffer,
        buffer_width,
        src_rect,
        dst_rect
    )
}

pub trait Drawable {
    fn draw_impl(&self, buffer: &mut [u32], buffer_width: usize, self_rect: Rect, dst_rect: Rect);
    fn get_width(&self) -> usize;
    fn get_height(&self) -> usize;
}

impl Drawable for BmpSprite {
    fn draw_impl(&self, buffer: &mut [u32], buffer_width: usize, self_rect: Rect, dst_rect: Rect) {
        let mut src_rect = self_rect;
        let mut dst_rect = dst_rect;
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
        match self {
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
                            *dst_entry = color;
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
                        src_entry.copy_to_nonoverlapping(dst_entry, span_length);
                    }
                    src_stride += *width;
                    dst_stride += buffer_width;
                }
            }
            BmpSprite::NotSupported => ()
        }
    }

    fn get_width(&self) -> usize {
        match self {
            BmpSprite::Paletted { width, .. } => *width,
            BmpSprite::TrueColor { width, .. } => *width,
            BmpSprite::NotSupported => 0
        }
    }

    fn get_height(&self) -> usize {
        match self {
            BmpSprite::Paletted { height, .. } => *height,
            BmpSprite::TrueColor { height, .. } => *height,
            BmpSprite::NotSupported => 0
        }
    }
}