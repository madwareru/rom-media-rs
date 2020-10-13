use std::io::Cursor;
use rom_loaders_rs::multimedia::SmackerFile;
use std::time::Instant;

#[derive(Copy, Clone, PartialEq)]
pub enum PlayerState {
    Entry,
    Playing,
    RenderedNewFrame,
    FinishedPlaying
}

pub struct SmackerPlayer {
    pub state: PlayerState,
    instant_timer: std::time::Instant,
    delta: f32,
    frame: usize,
    smacker_file: SmackerFile
}
impl SmackerPlayer {
    pub fn load_from_stream(stream: &mut Cursor<&[u8]>) -> std::io::Result<Self> {
        let smacker_file = SmackerFile::load(stream)?;
        let instant_timer = Instant::now();
        Ok(Self {
            instant_timer,
            delta: smacker_file.file_info.frame_interval,
            frame: 0,
            state: PlayerState::Entry,
            smacker_file
        })
    }
    pub fn frame(&mut self, skip_video: bool, skip_audio: bool) -> std::io::Result<PlayerState> {
        if self.state == PlayerState::FinishedPlaying {
            return Ok(self.state);
        }
        self.state = PlayerState::Playing;
        if self.delta >= self.smacker_file.file_info.frame_interval {
            if self.frame < self.smacker_file.file_info.frames.len() {
                self.smacker_file.unpack(self.frame, skip_video, skip_audio)?;
                self.frame += 1;
                self.state = PlayerState::RenderedNewFrame;
            } else {
                self.state = PlayerState::FinishedPlaying;
            }
        }
        self.delta += self.instant_timer.elapsed().as_secs_f32() * 1000.0;
        self.instant_timer = Instant::now();
        Ok(self.state)
    }
    pub fn blit_picture(
        &self,
        buffer: &mut[u32],
        x: usize, y: usize,
        buffer_width: usize
    ) {
        let mut offset = 0;
        let mut buffer_offset = y * buffer_width;
        let ctx = &self.smacker_file.file_info.smacker_decode_context;
        for _ in 0..self.smacker_file.file_info.height as usize {
            if buffer_offset >= buffer.len() {
                break;
            }
            for i in 0..self.smacker_file.file_info.width as usize {
                if i + x >= buffer_width {
                    offset += 1;
                    continue;
                }
                let palette_index = ctx.image[offset] as usize;
                buffer[buffer_offset + i] = ctx.palette[palette_index];
                offset += 1;
            }
            buffer_offset += buffer_width;
        }
    }
}