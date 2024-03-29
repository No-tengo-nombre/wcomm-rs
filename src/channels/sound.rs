use crate::channels::Channel;
use crate::modulation::DigitalModulator;
use crate::Message;
use anyhow;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use fundsp::hacker::*;
use std::fs::File;
use std::path::Path;
use wav;

fn run<T>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    frequency: u32,
    time: u32,
) -> Result<(), anyhow::Error>
where
    T: cpal::Sample,
{
    let sample_rate = config.sample_rate.0 as f64;
    let channels = config.channels as usize;

    let mut c = sine_hz(frequency.into());

    c.reset(Some(sample_rate));

    let mut next_value = move || c.get_stereo();

    let err_fn = |err| eprintln!("An error occurred on stream: {}", err);

    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            write_data(data, channels, &mut next_value)
        },
        err_fn,
    )?;
    stream.play()?;

    std::thread::sleep(std::time::Duration::from_millis(time.into()));

    Ok(())
}

fn write_data<T>(output: &mut [T], channels: usize, next_sample: &mut dyn FnMut() -> (f64, f64))
where
    T: cpal::Sample,
{
    for frame in output.chunks_mut(channels) {
        let sample = next_sample();
        let left: T = cpal::Sample::from::<f32>(&(sample.0 as f32));
        let right: T = cpal::Sample::from::<f32>(&(sample.1 as f32));

        for (channel, sample) in frame.iter_mut().enumerate() {
            if channel & 1 == 0 {
                *sample = left;
            } else {
                *sample = right;
            }
        }
    }
}

/**
 * Channel to play messages through sound.
 */
pub struct Sound<'a> {
    _modulator: &'a dyn DigitalModulator,
    _host: cpal::Host,
    _device: cpal::Device,
    _config: cpal::SupportedStreamConfig,
}

impl<'a> Channel for Sound<'a> {
    fn send(&self, msg: &Message, time: u32) {
        self._modulator.send_msg(self, msg, time);
    }

    fn play(&self, frequency: u32, time: u32) {
        match self._config.sample_format() {
            cpal::SampleFormat::F32 => {
                run::<f32>(&self._device, &self._config.clone().into(), frequency, time).unwrap()
            }
            cpal::SampleFormat::I16 => {
                run::<i16>(&self._device, &self._config.clone().into(), frequency, time).unwrap()
            }
            cpal::SampleFormat::U16 => {
                run::<u16>(&self._device, &self._config.clone().into(), frequency, time).unwrap()
            }
        }
    }

    fn listen(&self, frequency: u32) {
        // TODO: Implement listening to a specific frequency
    }
}

impl<'a> Sound<'a> {
    /**
     * Create a sound channel with an attached modulator.
     */
    pub fn new(modulator: &dyn DigitalModulator) -> Sound {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .expect("failed to find a default output device");
        let config = device.default_output_config().unwrap();

        return Sound {
            _modulator: modulator,
            _host: host,
            _device: device,
            _config: config,
        };
    }

    /**
     * Set the modulator of the channel.
     */
    pub fn modulator(mut self, new_mod: &'a dyn DigitalModulator) -> Sound {
        self._modulator = new_mod;
        return self;
    }

    /**
     * Export the modulated message to a wav file.
     */
    pub fn export_wav(&self, msg: &Message, filename: &str, time: u32) {
        // Obtain the data for the file
        let data = self._modulator.get_raw_data(msg, time);

        // Export the data
        let mut out_file = File::create(Path::new(filename)).unwrap();
        let header = wav::Header::new(
            wav::header::WAV_FORMAT_IEEE_FLOAT,
            1,
            self._modulator.get_sampling_frequency(),
            32,
        );
        let out_data = wav::BitDepth::ThirtyTwoFloat(data);
        wav::write(header, &out_data, &mut out_file).unwrap();
    }
}
