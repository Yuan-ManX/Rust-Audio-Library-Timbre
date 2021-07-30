use crate::{AudioFormat, AudioSource, ReadResult, Sample};
use tracing::instrument;

/// An [`AudioSource`](crate::AudioSource) that generates a sine wave.
///
/// # Examples
/// ```
/// # use timbre::{AudioFormat, generators::SineWave};
/// let sin = SineWave::new(AudioFormat::STEREO_DVD, 1.0, 440.0);
/// ```
#[derive(Clone)]
pub struct SineWave {
    amplitude: f32,
    format: AudioFormat,
    phase: f32,
    frequency: f32,
}

impl SineWave {
    /// Construct a new sine wave generator with the given amplitude and frequency.
    ///
    /// # Arguments
    ///
    /// * `format` -- The format for the generated stream.
    /// * `amplitude` -- The peak value of samples generated by the generator.
    /// * `frequency` -- The frequency of the wave generated, in Hz.
    pub fn new(format: AudioFormat, amplitude: f32, frequency: f32) -> Self {
        SineWave {
            amplitude,
            format,
            phase: 0.0,
            frequency,
        }
    }
}

impl AudioSource for SineWave {
    fn format(&self) -> crate::AudioFormat {
        self.format
    }

    #[instrument(name = "SineWave::read", skip(self, buffer))]
    fn read(&mut self, buffer: &mut [Sample]) -> crate::ReadResult {
        let increment =
            std::f32::consts::PI * 2.0 * self.frequency / self.format.sample_rate as f32;

        let channels = self.format.channels as usize;
        let frames = buffer.len() / channels;

        for i in 0..frames {
            let amplitude = self.amplitude * self.phase.sin();
            for channel in 0..channels as usize {
                buffer[i * channels + channel] = amplitude;
            }
            self.phase += increment;
        }

        ReadResult::good(buffer.len())
    }
}
