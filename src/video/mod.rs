use std::io::Cursor;
use rom_loaders_rs::multimedia::{SmackerFile, Audio};
use std::time::Instant;
use crate::audio::{SoundMixer, Sound, PlaybackBuilder};
use crate::audio::mixer::PlaybackStyle;

#[derive(Copy, Clone, PartialEq)]
pub enum PlayerState {
    PreloadingAudio,
    FinishedAudioPreload,
    Playing,
    RenderedNewFrame,
    FinishedPlaying
}

pub struct SmackerPlayer {
    pub state: PlayerState,
    pub frame_width: usize,
    pub frame_height: usize,
    fade_in_frames: usize,
    fade_out_frames: usize,
    delta: f32,
    frame: usize,
    audio_frame: usize,
    smacker_file: SmackerFile,
    sound_mixer: SoundMixer,
    brightness: u8
}
impl SmackerPlayer {
    pub fn load_from_stream(stream: &mut Cursor<&[u8]>) -> std::io::Result<Self> {
        let smacker_file = SmackerFile::load(stream)?;
        let sound_mixer = SoundMixer::new();
        Ok(Self {
            delta: 0.0,
            frame: 0,
            fade_in_frames: 0, // todo: implement fade in
            fade_out_frames: 0, // todo: implement fade out
            audio_frame: 0,
            state: PlayerState::PreloadingAudio,
            frame_width: smacker_file.file_info.width as usize,
            frame_height: smacker_file.file_info.height as usize,
            smacker_file,
            sound_mixer,
            brightness: 255
        })
    }
    pub fn frame(&mut self, delta_time: f32) -> std::io::Result<PlayerState> {
        if self.state == PlayerState::FinishedPlaying {
            return Ok(self.state);
        }
        if self.state == PlayerState::PreloadingAudio {
            let next_bulk_frame = self.audio_frame + 256;
            while self.audio_frame < self.smacker_file.file_info.frames.len() && self.audio_frame < next_bulk_frame {
                self.smacker_file.unpack(self.audio_frame, true, false)?;
                self.audio_frame += 1;
            }
            if self.audio_frame == self.smacker_file.file_info.frames.len() {
                self.state = PlayerState::FinishedAudioPreload;
                for i in 0..self.smacker_file.file_info.audio_flags.len() {
                    if !self.smacker_file.file_info.audio_flags[i].contains(Audio::PRESENT) {
                        continue;
                    }
                    let sound = Sound {
                        sample_rate: self.smacker_file.file_info.audio_rate[i] as f32,
                        channels: if self.smacker_file.file_info.audio_flags[i].contains(Audio::IS_STEREO) {
                            2
                        } else {
                            1
                        },
                        samples: self.smacker_file.file_info.audio_tracks[i].clone(),
                        playback_style: PlaybackStyle::Once
                    };
                    self.sound_mixer.play(PlaybackBuilder::new().with_sound(sound)).unwrap();
                }
            }
            return Ok(self.state);
        }

        self.delta += delta_time as f32;

        self.state = PlayerState::Playing;
        while self.delta >= self.smacker_file.file_info.frame_interval {
            if self.frame < self.smacker_file.file_info.frames.len() {
                self.smacker_file.unpack(self.frame, false, true)?;
                self.frame += 1;
                self.state = PlayerState::RenderedNewFrame;
            } else {
                self.state = PlayerState::FinishedPlaying;
            }
            self.delta -= self.smacker_file.file_info.frame_interval;
        }
        Ok(self.state)
    }
    pub fn blit_picture(
        &self,
        buffer: &mut[u32],
        x: usize, y: usize,
        buffer_width: usize
    ) {
        let mut offset = 0;
        let mut buffer_offset = x + y * buffer_width;
        let ctx = &self.smacker_file.file_info.smacker_decode_context;
        for _ in 0..self.smacker_file.file_info.height as usize {
            if buffer_offset >= buffer.len() {
                break;
            }
            for i in 0..self.smacker_file.file_info.width as usize {
                if i + x < buffer_width {
                    let palette_index = ctx.image[offset] as usize;
                    let (r, g, b) = ctx.palette[palette_index];
                    let r = (r as u32 * self.brightness as u32) / 255;
                    let g = (g as u32 * self.brightness as u32) / 255;
                    let b = (b as u32 * self.brightness as u32) / 255;
                    buffer[buffer_offset + i] = 0xFF_00_00_00 + b * 0x1_00_00 + g * 0x1_00 + r;
                }
                offset += 1;
            }
            buffer_offset += buffer_width;
        }
    }
}