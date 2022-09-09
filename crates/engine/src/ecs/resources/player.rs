use std::{thread::sleep, time::Duration};

use ambisonic::{
    rodio::{self, source::SineWave, Source},
    Ambisonic, AmbisonicBuilder,
};

use crate::file;

pub struct Player {}

impl Default for Player {
    fn default() -> Self {
        Self {}
    }
}

impl Player {
    pub fn play(&mut self, sound: &String) {
        let scene = AmbisonicBuilder::default().build();

        for _ in 0..500 {
            let source = rodio::source::SineWave::new(440).amplify(0.001);
            let _ = scene.play_omni(source);
        }

        sleep(Duration::from_secs(10));
    }
}
