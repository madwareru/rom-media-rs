use std::collections::HashMap;
use crate::audio::sound_driver::SoundDriver;
use rom_loaders_rs::multimedia::WavContent;
use std::io::Cursor;

pub(crate) enum MixerMessage {
    Play(SoundId, Sound, Volume),
    SetVolume(SoundId, Volume),
    StreamContent(SoundId, Vec<f32>),
    SetVolumeSelf(Volume),
    Stop(SoundId),
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct SoundId(usize);

#[derive(Clone, Copy, Debug)]
pub struct Volume(pub f32);

#[derive(Clone, PartialEq, Debug)]
pub enum PlaybackStyle {
    Once,
    Looped,
    Streamed
}

#[derive(Copy, Clone)]
struct SampleRateCorrection {
    progress_increment_amount: usize,
    ticks_pre_increment: usize
}

#[derive(Clone)]
pub struct Sound {
    pub sample_rate: f32,
    pub channels: u16,
    pub samples: Vec<f32>,
    pub playback_style: PlaybackStyle
}
impl From<&WavContent> for Sound {
    fn from(content: &WavContent) -> Self {
        let samples = content.data.iter()
            .map(|sample| *sample as f32 / std::i16::MAX as f32)
            .collect();
        Self {
            sample_rate: content.fmt.sampling_rate as f32,
            channels: content.fmt.channels,
            samples,
            playback_style: PlaybackStyle::Once
        }
    }
}
impl Sound {
    pub fn from_bytes(bytes: &[u8]) -> std::io::Result<Self> {
        Self::from_bytes_ext(bytes, PlaybackStyle::Once)
    }
    pub fn from_bytes_ext(bytes: &[u8], playback_style: PlaybackStyle) -> std::io::Result<Self> {
        let mut cursor = Cursor::new(bytes);
        let content = WavContent::read(&mut cursor)?;
        let sound = Sound::from(&content);
        Ok(Self {
            playback_style,
            ..sound
        })
    }
    fn get_sample_rate_correction(&self) -> SampleRateCorrection {
        let sample_rate = self.sample_rate as usize;
        let progress_increment_amount = if sample_rate > 44100 {
            sample_rate / 44100
        } else {
            1
        } * self.channels as usize;
        let ticks_pre_increment = if sample_rate >= 44100 {
            1
        } else {
            44100 / sample_rate
        } * 2;
        SampleRateCorrection {
            progress_increment_amount,
            ticks_pre_increment
        }
    }
}

struct SoundInternal {
    data: Sound,
    progress: usize,
    volume: Volume,
    ear: EarState,
    sample_rate_correction: SampleRateCorrection,
    ticks: usize
}

pub(crate) struct MixerInternal {
    sample_rate: f32,
    sounds: HashMap<SoundId, SoundInternal>,
    dead_sounds: Vec<SoundId>,
    volume: Volume,
    ear: EarState,
}

#[derive(PartialEq, Clone, Copy)]
enum EarState {
    Left,
    Right
}
impl EarState {
    fn switch(&mut self) {
        *self = match self {
            EarState::Left => EarState::Right,
            EarState::Right => EarState::Left
        }
    }
}

pub struct SoundMixer {
    driver: super::sound_driver::SoundDriver,
    uid: usize
}

pub struct PlaybackBuilder {
    sound: Option<Sound>,
    volume: Volume
}
impl PlaybackBuilder {
    pub fn new() -> Self {
        Self {
            sound: None,
            volume: Volume(1.0)
        }
    }
    pub fn with_volume(self, volume: Volume) -> Self {
        Self {
            volume,
            ..self
        }
    }
    pub fn with_sound(self, sound: Sound) -> Self {
        Self {
            sound: Some(sound),
            ..self
        }
    }
}

impl SoundMixer {
    pub fn new() -> SoundMixer {
        let mut driver = SoundDriver::new(Box::new(MixerInternal {
            sample_rate: 0.,
            sounds: HashMap::new(),
            dead_sounds: Vec::new(),
            volume: Volume(1.0),
            ear: EarState::Left
        }));
        driver.start();
        SoundMixer { driver, uid: 0 }
    }

    pub fn new_ext(initial_volume: Volume) -> SoundMixer {
        let mut driver = SoundDriver::new(Box::new(MixerInternal {
            sample_rate: 0.,
            sounds: HashMap::new(),
            dead_sounds: Vec::new(),
            volume: initial_volume,
            ear: EarState::Left
        }));
        driver.start();
        SoundMixer { driver, uid: 0 }
    }

    pub fn play(&mut self, playback_builder: PlaybackBuilder) -> Option<SoundId> {
        if playback_builder.sound.is_none() {
            None
        } else {
            let sound_id = SoundId(self.uid);
            self.uid += 1;
            self.driver.send_event(MixerMessage::Play(sound_id, playback_builder.sound.unwrap(), playback_builder.volume));
            Some(sound_id)
        }
    }

    pub fn stream_sound(&mut self, sound_id: SoundId, content: Vec<f32>) {
        self.driver.send_event(MixerMessage::StreamContent(sound_id, content))
    }

    pub fn set_volume(&mut self, sound_id: SoundId, volume: Volume) {
        self.driver.send_event(MixerMessage::SetVolume(sound_id, volume));
    }

    pub fn set_volume_self(&mut self, volume: Volume) {
        self.driver.send_event(MixerMessage::SetVolumeSelf(volume));
    }

    pub fn stop(&mut self, sound_id: SoundId) {
        self.driver.send_event(MixerMessage::Stop(sound_id));
    }

    pub fn frame(&mut self) {
        self.driver.frame();
    }
}

impl MixerInternal {
    pub(crate)fn init(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
    }

    pub(crate) fn handle_event(&mut self, evt: MixerMessage) {
        match evt {
            MixerMessage::Play(id, sound, volume) => {
                assert!(volume.0 <= 1.0);
                let sample_rate_correction = sound.get_sample_rate_correction();
                self.sounds.insert(
                    id,
                    SoundInternal {
                        data: sound,
                        progress: 0,
                        volume,
                        ear: EarState::Left,
                        sample_rate_correction,
                        ticks: sample_rate_correction.ticks_pre_increment
                    },
                );
            },
            MixerMessage::StreamContent(id, content) => {
                if let Some(sound) = self.sounds.get_mut(&id) {
                    assert_eq!(sound.data.playback_style, PlaybackStyle::Streamed);
                    sound.data.samples.extend(content);
                }
            }
            MixerMessage::SetVolume(id, volume) => {
                if let Some(sound) = self.sounds.get_mut(&id) {
                    assert!(volume.0 <= 1.0);
                    sound.volume = volume;
                }
            },
            MixerMessage::SetVolumeSelf( volume) => {
                self.volume = volume;
            },
            MixerMessage::Stop(id) => {
                self.sounds.remove(&id);
            }
        }
    }

    pub(crate) fn next_value(&mut self) -> f32 {
        let mut value = 0.;

        for (sound_id, mut sound) in &mut self.sounds {
            if self.ear != sound.ear {
                continue;
            }

            if sound.progress >= sound.data.samples.len() {
                match sound.data.playback_style {
                    PlaybackStyle::Once => {
                        self.dead_sounds.push(*sound_id);
                        continue;
                    }
                    PlaybackStyle::Looped => {
                        sound.progress = 0;
                    }
                    PlaybackStyle::Streamed => {
                        continue;
                    }
                }
            }

            let volume = sound.volume.0 * self.volume.0;
            // it's better to remap volume exponentially
            // so user hears difference instantly
            let volume = volume * volume;

            let next_index = match sound.data.channels {
                1 => sound.progress,
                2 => match sound.ear {
                    EarState::Left => sound.progress,
                    EarState::Right => sound.progress + 1
                },
                _ => unreachable!()
            };
            sound.ticks -= 1;

            value += sound.data.samples[next_index] * volume;
            if sound.ticks == 0 {
                sound.progress += sound.sample_rate_correction.progress_increment_amount;
                sound.ticks = sound.sample_rate_correction.ticks_pre_increment;
            }
            sound.ear.switch();
        }

        for sound_id in self.dead_sounds.iter() {
            self.sounds.remove(sound_id);
        }
        self.dead_sounds.clear();

        self.ear.switch();

        value
    }
}