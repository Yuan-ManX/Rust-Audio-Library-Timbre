use crate::{core::AudioSource, ReadResult, Sample};

use tracing::instrument;

/// An effect that simulates an echo.
///
/// # Examples
/// ```
/// # use timbre::{generators::SineWave, effects::Echo, IntoShared};
/// # use std::time::Duration;
/// let sin = SineWave::new(1.0, 440.0);
/// let echo = Echo::new(sin, Duration::from_secs_f32(0.5), 0.8);
/// ```
pub struct Echo<S: AudioSource> {
    source: S,
    delay: f32,
    decay: f32,
    buffer: Vec<f32>,
    position: usize,
}

impl<S: AudioSource> Echo<S> {
    /// Construct a new `Echo` effect.
    ///
    /// # Arguments
    ///
    /// * `source` -- The source of audio for this effect.
    /// * `delay` -- The length of time before the echo plays back.
    /// * `decay` -- The amount by which to decay the echo on each repitition. Should
    ///              be between 0.0 and 1.0, unless you like feedback.
    pub fn new(source: S, delay: std::time::Duration, decay: f32) -> Self {
        let delay = delay.as_secs_f32();
        Echo {
            source,
            delay,
            decay,
            buffer: Vec::new(),
            position: 0,
        }
    }
}

impl<S: AudioSource> AudioSource for Echo<S> {
    fn format(&self) -> crate::AudioFormat {
        self.source.format()
    }

    #[instrument(name = "Echo::read", skip(self, buffer))]
    fn read(&mut self, buffer: &mut [Sample]) -> ReadResult {
        let format = self.source.format();
        let delay: usize =
            (format.sample_rate as f32 * self.delay).ceil() as usize * format.channels as usize;
        self.buffer.resize(delay, 0.0);

        let status = self.source.read(buffer);
        let written = status.read;

        echo(
            &mut self.buffer,
            buffer,
            written,
            &mut self.position,
            delay,
            self.decay,
        );

        status
    }
}

fn echo(
    buffer: &mut Vec<f32>,
    samples: &mut [f32],
    written: usize,
    position: &mut usize,
    delay: usize,
    decay: f32,
) {
    let mut i = 0;
    while i < written {
        let count = std::cmp::min(delay - *position, written - i);
        (&mut buffer[*position..delay])
            .iter_mut()
            .zip((&mut samples[i..written]).iter_mut())
            .for_each(|(b, s)| {
                *b = *b * decay + *s;
                *s = *b;
            });

        i += count;
        *position = (*position + count) % delay;
    }
}
