use sdl2::mixer::{Channel, Chunk, InitFlag, AUDIO_S8, DEFAULT_CHANNELS};
use std::path::Path;

pub struct AudioMixer {
    ufo: Chunk,
    shoot: Chunk,
    player_death: Chunk,
    invader_death: Chunk,
    invader_1: Chunk,
    invader_2: Chunk,
    invader_3: Chunk,
    invader_4: Chunk,
}

impl AudioMixer {
    pub fn new() -> Self {
        sdl2::mixer::open_audio(44_100, AUDIO_S8, DEFAULT_CHANNELS, 1_024).unwrap();
        sdl2::mixer::init(InitFlag::MID).unwrap();
        sdl2::mixer::allocate_channels(8);

        let ufo = Chunk::from_file("sounds/ufo.wav").unwrap();
        let shoot = Chunk::from_file(Path::new("sounds/shoot.wav")).unwrap();
        let player_death = Chunk::from_file("sounds/player_death.wav").unwrap();
        let invader_death = Chunk::from_file("sounds/invader_death.wav").unwrap();
        let invader_1 = Chunk::from_file("sounds/invader1.wav").unwrap();
        let invader_2 = Chunk::from_file("sounds/invader2.wav").unwrap();
        let invader_3 = Chunk::from_file("sounds/invader3.wav").unwrap();
        let invader_4 = Chunk::from_file("sounds/invader4.wav").unwrap();

        AudioMixer {
            ufo,
            shoot,
            player_death,
            invader_death,
            invader_1,
            invader_2,
            invader_3,
            invader_4,
        }
    }

    pub fn play_ufo(&self) {
        Channel(0).play(&self.ufo, -1).unwrap();
    }

    pub fn stop_ufo(&self) {
        Channel(0).pause();
    }

    pub fn play_shoot(&self) {
        Channel(1).play(&self.shoot, 0).unwrap();
    }

    pub fn play_player_death(&self) {
        Channel(2).play(&self.player_death, 0).unwrap();
    }

    pub fn play_invader_death(&self) {
        Channel(3).play(&self.invader_death, 0).unwrap();
    }

    pub fn play_invader_1(&self) {
        Channel(4).play(&self.invader_1, 0).unwrap();
    }

    pub fn play_invader_2(&self) {
        Channel(5).play(&self.invader_2, 0).unwrap();
    }

    pub fn play_invader_3(&self) {
        Channel(6).play(&self.invader_3, 0).unwrap();
    }

    pub fn play_invader_4(&self) {
        Channel(7).play(&self.invader_4, 0).unwrap();
    }
}
