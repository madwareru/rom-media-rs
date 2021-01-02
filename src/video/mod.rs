use std::io::Cursor;
use rom_loaders_rs::multimedia::{SmackerFile, Audio};
use crate::audio::{SoundMixer, Sound, PlaybackBuilder};
use crate::audio::mixer::PlaybackStyle;
use crate::image_rendering::blittable::{Blittable, Rect};

#[derive(Copy, Clone, PartialEq)]
pub enum PreloadingAudioState {
    InProgress,
    Complete
}

#[derive(Copy, Clone, PartialEq)]
pub enum FadeInState {
    InProgress { t: f32, t_max: f32 },
    Complete
}

#[derive(Copy, Clone, PartialEq)]
pub enum RenderingFramesState {
    InProgress,
    RenderedNewFrame,
    Complete
}

#[derive(Copy, Clone, PartialEq)]
pub enum FadeOutState {
    InProgress { t: f32, t_max: f32 },
    Complete
}

#[derive(Copy, Clone, PartialEq)]
pub enum PlayerState {
    PreloadingAudio{ frame: usize, state: PreloadingAudioState },
    FadeIn(FadeInState),
    IsRendering{ frame: usize, delta: f32, state: RenderingFramesState },
    FadeOut(FadeOutState),
    FinishedPlaying
}

pub struct SmackerPlayer {
    pub state: PlayerState,
    pub frame_width: usize,
    pub frame_height: usize,
    fade_in_ms: usize,
    fade_out_ms: usize,
    smacker_file: SmackerFile,
    sound_mixer: SoundMixer,
    brightness: u8
}
impl Blittable<u32> for SmackerPlayer {
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
        let width = self.frame_width as usize;

        let ctx = &self.smacker_file.file_info.smacker_decode_context;

        let mut src_stride = src_rect.y_range.start * width + src_rect.x_range.start;
        let mut dst_stride = dst_rect.y_range.start * buffer_width + dst_rect.x_range.start;
        for _ in 0..span_count {
            match self.brightness {
                0 => for dest in (&mut buffer[dst_stride..dst_stride + span_length]).iter_mut() {
                    *dest = 0xFF_00_00_00;
                },
                255 => {
                    let zipped = (&mut buffer[dst_stride..dst_stride + span_length])
                        .iter_mut()
                        .zip(&ctx.image[src_stride..src_stride + span_length]);
                    for (dest, src) in zipped {
                        let idx = *src;
                        let clr = ctx.palette[idx as usize];
                        let (r, g, b) = (clr.0 as u32, clr.1 as u32, clr.2 as u32);
                        *dest = b * 0x1_00_00 | g * 0x1_00 | r;
                    }
                },
                _ => {
                    let zipped = (&mut buffer[dst_stride..dst_stride + span_length])
                        .iter_mut()
                        .zip(&ctx.image[src_stride..src_stride + span_length]);
                    for (dest, src) in zipped {
                        let idx = *src;
                        let clr = ctx.palette[idx as usize];
                        let (r, g, b) = (clr.0 as u32, clr.1 as u32, clr.2 as u32);
                        let r = (r * self.brightness as u32) / 255;
                        let g = (g * self.brightness as u32) / 255;
                        let b = (b * self.brightness as u32) / 255;
                        *dest = b * 0x1_00_00 | g * 0x1_00 | r;
                    }
                }
            }
            src_stride += width;
            dst_stride += buffer_width;
        }
    }

    fn get_width(&self) -> usize {
        self.frame_width as usize
    }

    fn get_height(&self) -> usize {
        self.frame_height as usize
    }
}

impl SmackerPlayer {
    pub fn load_from_stream(stream: &mut Cursor<&[u8]>) -> std::io::Result<Self> {
        let smacker_file = SmackerFile::load(stream)?;
        let sound_mixer = SoundMixer::new();
        Ok(Self {
            fade_in_ms: 0,
            fade_out_ms: 0,
            state: PlayerState::PreloadingAudio{
                frame: 0,
                state: PreloadingAudioState::InProgress
            },
            frame_width: smacker_file.file_info.width as usize,
            frame_height: smacker_file.file_info.height as usize,
            smacker_file,
            sound_mixer,
            brightness: 0
        })
    }
    pub fn set_fade_in_ms(&mut self, fade_in_ms: usize) {
        self.fade_in_ms = fade_in_ms;
    }
    pub fn set_fade_out_ms(&mut self, fade_out_ms: usize) {
        self.fade_out_ms = fade_out_ms;
    }
    pub fn frame(&mut self, delta_time: f32) -> std::io::Result<PlayerState> {
        match &mut self.state {
            PlayerState::FinishedPlaying => Ok(self.state),
            PlayerState::PreloadingAudio { frame, state }  => match state {
                PreloadingAudioState::InProgress => {
                    let next_bulk_frame = *frame + 256;
                    while *frame < self.smacker_file.file_info.frames.len() && *frame < next_bulk_frame {
                        self.smacker_file.unpack(*frame, true, false)?;
                        *frame += 1;
                    }
                    if *frame == self.smacker_file.file_info.frames.len() {
                        *state = PreloadingAudioState::Complete;
                    }
                    Ok(self.state)
                },
                PreloadingAudioState::Complete => {
                    self.smacker_file.unpack(0, false, true)?;
                    self.state = if self.fade_in_ms > 0 {
                        PlayerState::FadeIn(FadeInState::InProgress {
                            t: 0.0,
                            t_max: self.fade_in_ms as f32
                        })
                    } else {
                        PlayerState::FadeIn(FadeInState::Complete)
                    };
                    Ok(self.state)
                },
            },
            PlayerState::FadeIn(state) => match state {
                FadeInState::InProgress { t, t_max } => {
                    if *t >= *t_max {
                        *state = FadeInState::Complete;
                    } else {
                        let alpha = *t / *t_max;
                        let alpha = alpha * alpha;
                        self.brightness = (alpha * 255.0) as u8;
                        *t += delta_time;
                    }
                    Ok(self.state)
                },
                FadeInState::Complete => {
                    self.brightness = 255;
                    self.state = PlayerState::IsRendering {
                        frame: 1, // since we prerendered one frame we want to start from the second
                        delta: 0.0,
                        state: RenderingFramesState::RenderedNewFrame // we need to renderize frame immediately
                    };
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
                    Ok(self.state)
                },
            },
            PlayerState::IsRendering { frame, delta, state } => {
                if *state == RenderingFramesState::Complete {
                    self.state = if self.fade_out_ms > 0 {
                        PlayerState::FadeOut(FadeOutState::InProgress {
                            t: 0.0,
                            t_max: self.fade_out_ms as f32
                        })
                    } else {
                        PlayerState::FadeOut(FadeOutState::Complete)
                    };
                    return Ok(self.state);
                }
                *delta += delta_time;
                *state = if *frame == self.smacker_file.file_info.frames.len() {
                    RenderingFramesState::Complete
                } else if *delta < self.smacker_file.file_info.frame_interval {
                    RenderingFramesState::InProgress
                } else {
                    while *delta >= self.smacker_file.file_info.frame_interval &&
                        *frame < self.smacker_file.file_info.frames.len()
                    {
                        self.smacker_file.unpack(*frame, false, true)?;
                        *frame += 1;
                        *delta -= self.smacker_file.file_info.frame_interval;
                    }
                    RenderingFramesState::RenderedNewFrame
                };
                Ok(self.state)
            },
            PlayerState::FadeOut(state) => match state {
                FadeOutState::InProgress { t, t_max } => {
                    if *t >= *t_max {
                        *state = FadeOutState::Complete;
                    } else {
                        let alpha = *t / *t_max;
                        let alpha = alpha * alpha;
                        self.brightness = ((1.0 - alpha) * 255.0) as u8;
                        *t += delta_time;
                    }
                    Ok(self.state)
                },
                FadeOutState::Complete => {
                    self.brightness = 0;
                    self.state = PlayerState::FinishedPlaying;
                    Ok(self.state)
                }
            }
        }
    }
}