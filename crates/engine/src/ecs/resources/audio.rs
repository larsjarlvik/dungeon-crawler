use crate::file;
use std::io::Cursor;

pub struct Player {
    sound: Vec<u8>,
    sink: rodio::Sink,
    _handle: rodio::OutputStreamHandle,
    _stream: rodio::OutputStream,
}

impl Default for Player {
    fn default() -> Self {
        let sound = file::read_bytes("sounds/steps-stone.ogg");
        let (stream, handle) = rodio::OutputStream::try_default().unwrap();
        let sink = rodio::Sink::try_new(&handle).unwrap();

        Self {
            sound,
            sink,
            _handle: handle,
            _stream: stream,
        }
    }
}

impl Player {
    pub fn play(&mut self) {
        let sound = self.sound.clone();

        let file = Cursor::new(sound);
        self.sink.append(rodio::Decoder::new(file).unwrap());
    }
}
