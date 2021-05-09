use rom_loaders_rs::images::ingame_sprite::{ImageData, ImageType};
use crate::image_rendering::blittable::{Blittable, Rect};

pub struct PalettedSpriteRenderingScope<'a> {
    pub palette: &'a [u32],
    pub image_data: &'a ImageData,
    pub img_id: usize
}

pub struct SpriteRenderingScope<'a> {
    pub image_data: &'a ImageData,
    pub img_id: usize
}

impl<'a> Blittable<u32> for PalettedSpriteRenderingScope<'a> {
    fn blit_impl(&self, buffer: &mut [u32], buffer_width: usize, self_rect: Rect, dst_rect: Rect) {
        const BLANK_LINE: u8 = 0x40;
        const EMPTY_AREA_BITS: u8 = 0xC0;
        const CHUNK_SIZE_BITS: u8 = 0x3F;
        match self.image_data.image_type {
            ImageType::Dot256 => {
                let current_frame = &self.image_data.frames[self.img_id % self.image_data.frames.len()];
                let start = current_frame.data_range.start;
                let end = current_frame.data_range.end;
                let slice = &self.image_data.raw[start..end];

                let sw = current_frame.width as i32;
                let (mut dx, mut dy) = (dst_rect.x_range.start as i32, dst_rect.y_range.start as i32);
                let (mut sx, mut sy) = (0, 0);

                let mut i = 0;
                while i < slice.len() {
                    let ipx = slice[i];
                    let is_empty_area_mask = ipx & EMPTY_AREA_BITS;
                    let chunk_size = (ipx & CHUNK_SIZE_BITS) as i32;
                    i += 1;

                    if is_empty_area_mask > 0 {
                        if is_empty_area_mask == BLANK_LINE {
                            dy += chunk_size;
                            sy += chunk_size;
                        } else {
                            sx += chunk_size;
                            dx += chunk_size;
                            if sx > sw {
                                dy += 1;
                                sy += 1;
                                dx -= sw;
                                sx -= sw;
                            }
                        }
                        continue;
                    }
                    for _ in 0..chunk_size {
                        let palette_id = slice[i] as usize; i += 1;
                        if sx >= sw {
                            dy += 1;
                            sy += 1;
                            dx -= sw;
                            sx -= sw;
                        }
                        if sx >= self_rect.x_range.start as i32 &&
                            sx < self_rect.x_range.end as i32 &&
                            sy >= self_rect.y_range.start as i32 &&
                            sy < self_rect.y_range.end as i32 &&
                            dx >= dst_rect.x_range.start as i32 &&
                            dx < dst_rect.x_range.end as i32 &&
                            dy >= dst_rect.y_range.start as i32 &&
                            dy < dst_rect.y_range.end as i32{
                            let offset = (dx + dy * buffer_width as i32) as usize;
                            buffer[offset] = self.palette[palette_id];
                        }
                        dx += 1;
                        sx += 1;
                    }
                }
            }
            ImageType::Dot16 => {

            }
            ImageType::Dot16a => {
                let current_frame = &self.image_data.frames[self.img_id % self.image_data.frames.len()];
                let start = current_frame.data_range.start;
                let end = current_frame.data_range.end;
                let slice = &self.image_data.raw[start..end];

                let sw = current_frame.width as i32;
                let (mut dx, mut dy) = (dst_rect.x_range.start as i32, dst_rect.y_range.start as i32);
                let (mut sx, mut sy) = (0, 0);

                let mut i = 0;
                while i < slice.len() {
                    let chunk_size = (slice[i] & CHUNK_SIZE_BITS) as i32; i += 1;
                    let is_empty_area_mask = slice[i] & EMPTY_AREA_BITS; i += 1;
                    if is_empty_area_mask > 0 {
                        if is_empty_area_mask == BLANK_LINE {
                            dy += chunk_size;
                            sy += chunk_size;
                        } else {
                            sx += chunk_size;
                            dx += chunk_size;
                            if sx > sw {
                                dy += 1;
                                sy += 1;
                                dx -= sw;
                                sx -= sw;
                            }
                        }
                        continue;
                    }
                    for _ in 0..chunk_size {
                        let psh = slice[i] as u16 | ((slice[i+1] as u16) * 0x100); i += 2;

                        // we first shift everything one bit right to get palette identifier
                        // then we extract alpha. Alpha is four-bit, therefore it takes values
                        // in a range [0x0..0xF]
                        let palette_id = ((psh / 0x002) & 0xFF) as usize;
                        let alpha = ((psh / 0x200) & 0xF) as u32 * 0x11_00_00_00;

                        if sx >= sw {
                            dy += 1;
                            sy += 1;
                            dx -= sw;
                            sx -= sw;
                        }
                        if sx >= self_rect.x_range.start as i32 &&
                            sx < self_rect.x_range.end as i32 &&
                            sy >= self_rect.y_range.start as i32 &&
                            sy < self_rect.y_range.end as i32 &&
                            dx >= dst_rect.x_range.start as i32 &&
                            dx < dst_rect.x_range.end as i32 &&
                            dy >= dst_rect.y_range.start as i32 &&
                            dy < dst_rect.y_range.end as i32{
                            let offset = (dx + dy * buffer_width as i32) as usize;
                            buffer[offset] = alpha | (self.palette[palette_id] & 0x00_FF_FF_FF);
                        }
                        dx += 1;
                        sx += 1;
                    }
                }
            }
        }
    }

    fn get_width(&self) -> usize {
        self.image_data.frames[self.img_id].width as usize
    }

    fn get_height(&self) -> usize {
        self.image_data.frames[self.img_id].height as usize
    }
}

/// Implementation for the paletted usage
impl<'a> Blittable<u16> for SpriteRenderingScope<'a> {
    fn blit_impl(&self, buffer: &mut [u16], buffer_width: usize, self_rect: Rect, dst_rect: Rect) {
        const BLANK_LINE: u8 = 0x40;
        const EMPTY_AREA_BITS: u8 = 0xC0;
        const CHUNK_SIZE_BITS: u8 = 0x3F;
        match self.image_data.image_type {
            ImageType::Dot256 => {
                let current_frame = &self.image_data.frames[self.img_id];
                let start = current_frame.data_range.start;
                let end = current_frame.data_range.end;
                let slice = &self.image_data.raw[start..end];

                let sw = current_frame.width as i32;
                let (mut dx, mut dy) = (dst_rect.x_range.start as i32, dst_rect.y_range.start as i32);
                let (mut sx, mut sy) = (0, 0);

                let mut i = 0;
                while i < slice.len() {
                    let ipx = slice[i];
                    let is_empty_area_mask = ipx & EMPTY_AREA_BITS;
                    let chunk_size = (ipx & CHUNK_SIZE_BITS) as i32;
                    i += 1;

                    if is_empty_area_mask > 0 {
                        if is_empty_area_mask == BLANK_LINE {
                            dy += chunk_size;
                            sy += chunk_size;
                        } else {
                            sx += chunk_size;
                            dx += chunk_size;
                            if sx > sw {
                                dy += 1;
                                sy += 1;
                                dx -= sw;
                                sx -= sw;
                            }
                        }
                        continue;
                    }
                    for _ in 0..chunk_size {
                        let palette_id = slice[i] as u16; i += 1;
                        if sx >= sw {
                            dy += 1;
                            sy += 1;
                            dx -= sw;
                            sx -= sw;
                        }
                        if sx >= self_rect.x_range.start as i32 &&
                            sx < self_rect.x_range.end as i32 &&
                            sy >= self_rect.y_range.start as i32 &&
                            sy < self_rect.y_range.end as i32 &&
                            dx >= dst_rect.x_range.start as i32 &&
                            dx < dst_rect.x_range.end as i32 &&
                            dy >= dst_rect.y_range.start as i32 &&
                            dy < dst_rect.y_range.end as i32{
                            let offset = (dx + dy * buffer_width as i32) as usize;
                            buffer[offset] = palette_id | 0xFF00;
                        }
                        dx += 1;
                        sx += 1;
                    }
                }
            }
            ImageType::Dot16 => {

            }
            ImageType::Dot16a => {
                let current_frame = &self.image_data.frames[self.img_id % self.image_data.frames.len()];
                let start = current_frame.data_range.start;
                let end = current_frame.data_range.end;
                let slice = &self.image_data.raw[start..end];

                let sw = current_frame.width as i32;
                let (mut dx, mut dy) = (dst_rect.x_range.start as i32, dst_rect.y_range.start as i32);
                let (mut sx, mut sy) = (0, 0);

                let mut i = 0;
                while i < slice.len() {
                    let chunk_size = (slice[i] & CHUNK_SIZE_BITS) as i32; i += 1;
                    let is_empty_area_mask = slice[i] & EMPTY_AREA_BITS; i += 1;
                    if is_empty_area_mask > 0 {
                        if is_empty_area_mask == BLANK_LINE {
                            dy += chunk_size;
                            sy += chunk_size;
                        } else {
                            sx += chunk_size;
                            dx += chunk_size;
                            if sx > sw {
                                dy += 1;
                                sy += 1;
                                dx -= sw;
                                sx -= sw;
                            }
                        }
                        continue;
                    }
                    for _ in 0..chunk_size {
                        let psh = slice[i] as u16 | ((slice[i+1] as u16) * 0x100); i += 2;

                        // we first shift everything one bit right to get palette identifier
                        // then we extract alpha. Alpha is four-bit, therefore it takes values
                        // in a range [0x0..0xF]
                        let palette_id = (psh / 0x002) & 0xFF;
                        let alpha = ((psh / 0x200) & 0xF) * 0x11_00;

                        if sx >= sw {
                            dy += 1;
                            sy += 1;
                            dx -= sw;
                            sx -= sw;
                        }
                        if sx >= self_rect.x_range.start as i32 &&
                            sx < self_rect.x_range.end as i32 &&
                            sy >= self_rect.y_range.start as i32 &&
                            sy < self_rect.y_range.end as i32 &&
                            dx >= dst_rect.x_range.start as i32 &&
                            dx < dst_rect.x_range.end as i32 &&
                            dy >= dst_rect.y_range.start as i32 &&
                            dy < dst_rect.y_range.end as i32{
                            let offset = (dx + dy * buffer_width as i32) as usize;
                            buffer[offset] = alpha | palette_id;
                        }
                        dx += 1;
                        sx += 1;
                    }
                }
            }
        }
    }

    fn get_width(&self) -> usize {
        self.image_data.frames[self.img_id].width as usize
    }

    fn get_height(&self) -> usize {
        self.image_data.frames[self.img_id].height as usize
    }
}