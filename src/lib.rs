mod core;
pub use crate::core::{AudioFormat, AudioSource, Share, StreamState};

pub mod decoders;
pub mod drivers;
pub mod effects;

mod sdl_util;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
