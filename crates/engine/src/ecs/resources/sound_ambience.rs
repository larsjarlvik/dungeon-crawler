use std::io::Cursor;

use crate::file;

pub struct SoundAmbience {
    sound: Option<Vec<u8>>,
    sink: rodio::Sink,
    _handle: rodio::OutputStreamHandle,
    _stream: rodio::OutputStream,
}

impl Default for SoundAmbience {
    fn default() -> Self {
        let (stream, handle) = rodio::OutputStream::try_default().unwrap();
        let sink = rodio::Sink::try_new(&handle).unwrap();

        Self {
            sound: None,
            sink,
            _handle: handle,
            _stream: stream,
        }
    }
}

impl SoundAmbience {
    pub fn load(&mut self, sound: String) {
        let path = format!("sounds/{}.ogg", sound);
        if !file::exists(path.as_str()) {
            panic!("Failed to load sound {}!", sound);
        }

        self.sound = Some(file::read_bytes(path.as_str()));

        let file = Cursor::new(self.sound.clone().unwrap());
        self.sink.append(rodio::Decoder::new_looped(file).unwrap());
        self.sink.set_volume(0.5);
    }
}
