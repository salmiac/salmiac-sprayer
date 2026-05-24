use rodio::{
    source::{SineWave, Source},
    OutputStream, OutputStreamHandle, Sink,
};
use std::time::Duration;

pub struct AudioService {
    _stream: Option<OutputStream>,
    handle: Option<OutputStreamHandle>,
}

impl Default for AudioService {
    fn default() -> Self {
        Self::new()
    }
}

impl AudioService {
    pub fn new() -> Self {
        match OutputStream::try_default() {
            Ok((stream, handle)) => Self {
                _stream: Some(stream),
                handle: Some(handle),
            },
            Err(e) => {
                log::error!("Failed to initialize audio output: {}", e);
                Self {
                    _stream: None,
                    handle: None,
                }
            }
        }
    }

    pub fn play_beep(&self) {
        if let Some(handle) = &self.handle {
            if let Ok(sink) = Sink::try_new(handle) {
                let source = SineWave::new(880.0) // A5
                    .take_duration(Duration::from_millis(150))
                    .amplify(0.2);
                sink.append(source);
                sink.detach();
            }
        }
    }
}
