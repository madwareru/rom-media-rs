use std::sync::mpsc::{Sender, channel};
use super::SoundError;
use super::mixer::MixerMessage;
use cpal::traits::{HostTrait, DeviceTrait, EventLoopTrait};
use cpal::{SampleRate, SupportedFormatsError, SupportedOutputFormats, SampleFormat};
use std::thread;
use crate::audio::mixer::{SoundMixer, MixerInternal};
use cpal::UnknownTypeOutputBuffer::F32;

pub struct SoundDriver {
    event_loop: Option<cpal::EventLoop>,
    format: Option<cpal::Format>,
    stream_id: Option<cpal::StreamId>,
    message_transmitter: Option<Sender<MixerMessage>>,
    mixer: Option<Box<MixerInternal>>,
    err: SoundError,
}

impl SoundDriver {
    /// After calling [`SoundDriver::new`], you can call this function to see if the audio initialization was a success.
    pub(crate) fn get_error(&self) -> SoundError {
        self.err
    }

    /// Initialize the sound device and provide the generator to the driver.
    pub(crate) fn new(generator: Box<MixerInternal>) -> Self {
        // Setup the audio system
        let host = cpal::default_host();
        let event_loop = host.event_loop();

        let device = match host.default_output_device() {
            Some(device) => device,
            None => {
                println!("warning : no sound device detected\n");
                return Self {
                    event_loop: Some(event_loop),
                    format: None,
                    stream_id: None,
                    message_transmitter: None,
                    mixer: Some(generator),
                    err: SoundError::NoDevice,
                };
            }
        };

        let mut output_format = match device.default_output_format() {
            Ok(default_output_format) => default_output_format,
            Err(err) => {
                println!("error : could not get default output format : {:?}\n", err);
                return Self {
                    event_loop: Some(event_loop),
                    format: None,
                    stream_id: None,
                    message_transmitter: None,
                    mixer: Some(generator),
                    err: SoundError::UnknownStreamFormat,
                };
            }
        };
        match device.supported_output_formats() {
            Ok(available_formats) => {
                for available_format in available_formats {
                    if available_format.channels != 2 { continue; }
                    if available_format.data_type != SampleFormat::F32 { continue; }
                    if available_format.min_sample_rate.0 > 44100 { continue; }
                    if available_format.max_sample_rate.0 < 44100 { continue; }
                    output_format.channels = 2;
                    output_format.data_type = SampleFormat::F32;
                    output_format.sample_rate = SampleRate(44100);
                }
            }
            Err(err) => {
                println!("error : could not get supported formats : {:?}\n", err);
            }
        };

        println!(
            "sound device : {} format {:?}\n",
            device.name().unwrap_or("no device name".into()),
            &output_format
        );
        if output_format.sample_rate.0 != 44100 {
            println!("Caution! Output format sample rate is not 44100!");
        }

        let stream_id = match event_loop.build_output_stream(&device, &output_format) {
            Ok(output_stream) => output_stream,
            Err(err) => {
                println!("error : could not build output stream : {:?}\n", err);
                return Self {
                    event_loop: Some(event_loop),
                    format: Some(output_format),
                    stream_id: None,
                    message_transmitter: None,
                    mixer: Some(generator),
                    err: SoundError::OutputStream,
                };
            }
        };

        Self {
            event_loop: Some(event_loop),
            format: Some(output_format),
            stream_id: Some(stream_id),
            message_transmitter: None,
            mixer: Some(generator),
            err: SoundError::NoError,
        }
    }

    /// Send an event to the generator
    pub(crate) fn send_event(&mut self, event: MixerMessage) {
        if let Some(ref mut tx) = self.message_transmitter {
            tx.send(event).unwrap();
        }
    }

    fn get_sample_rate(&self) -> f32 {
        if let Some(ref fmt) = self.format {
            fmt.sample_rate.0 as f32
        } else {
            1.0
        }
    }
    /// This function should be called every frame.
    /// It's only needed on web target to fill the output sound buffer.
    pub fn frame(&mut self) {}
    /// This will call the generator init function.
    /// On native target, it starts the sound thread and the audio loop.
    /// On web target, only the [`SoundDriver::frame`] function produces sound.
    pub fn start(&mut self) {
        let (tx, message_receiver) = channel();
        self.message_transmitter = Some(tx);
        let stream_id = self.stream_id.take().unwrap();
        let sample_rate = self.get_sample_rate();
        let mut generator = self.mixer.take().unwrap();
        if let Some(evt) = self.event_loop.take() {
            evt.play_stream(stream_id).expect("could not play stream");

            thread::spawn(move || {
                println!("starting audio loop");
                generator.init(sample_rate);
                evt.run(move |stream_id, stream_result| {
                    for event in message_receiver.try_iter() {
                        generator.handle_event(event);
                    }

                    let stream_data = match stream_result {
                        Ok(data) => data,
                        Err(err) => {
                            eprintln!("an error occurred on stream {:?}: {:?}", stream_id, err);
                            return;
                        }
                    };

                    match stream_data {
                        cpal::StreamData::Output {
                            buffer: cpal::UnknownTypeOutputBuffer::U16(mut buffer),
                        } => {
                            for elem in buffer.iter_mut() {
                                *elem = ((generator.next_value() * 0.5 + 0.5)
                                    * std::u16::MAX as f32)
                                    as u16;
                            }
                        }
                        cpal::StreamData::Output {
                            buffer: cpal::UnknownTypeOutputBuffer::I16(mut buffer),
                        } => {
                            for elem in buffer.iter_mut() {
                                *elem = (generator.next_value() * std::i16::MAX as f32) as i16;
                            }
                        }
                        cpal::StreamData::Output {
                            buffer: cpal::UnknownTypeOutputBuffer::F32(mut buffer),
                        } => {
                            for elem in buffer.iter_mut() {
                                *elem = generator.next_value();
                            }
                        }
                        _ => panic!("unsupported stream data"),
                    }
                })
            });
        }
    }
}