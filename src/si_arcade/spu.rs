use crate::binary_lib::get_bit;

// UFO
pub const SOUND_0: &[u8] = include_bytes!("../../game_audios/0.wav");
// Shoot
pub const SOUND_1: &[u8] = include_bytes!("../../game_audios/1.wav");
// Player Explosion
pub const SOUND_2: &[u8] = include_bytes!("../../game_audios/2.wav");
// Invaders explosion
pub const SOUND_3: &[u8] = include_bytes!("../../game_audios/3.wav");
// Invaders march 1
pub const SOUND_4: &[u8] = include_bytes!("../../game_audios/4.wav");
// Invaders march 2
pub const SOUND_5: &[u8] = include_bytes!("../../game_audios/5.wav");
// Invaders march 3
pub const SOUND_6: &[u8] = include_bytes!("../../game_audios/6.wav");
// Invaders march 4
pub const SOUND_7: &[u8] = include_bytes!("../../game_audios/7.wav");
// Bonus UFO destroyed
pub const SOUND_8: &[u8] = include_bytes!("../../game_audios/8.wav");

pub struct Spu {
    sounds_to_play: Vec<u8>,
}

impl Spu {
    pub fn new() -> Spu {
        Spu {
            sounds_to_play: Vec::new(),
        }
    }

    pub fn update(&mut self, port: u8, data: u8) {
        match port {
            3 => {
                if get_bit(data, 0) {
                    self.sounds_to_play.push(0);
                }
                if get_bit(data, 1) {
                    self.sounds_to_play.push(1);
                }
                if get_bit(data, 2) {
                    self.sounds_to_play.push(2);
                }
                if get_bit(data, 3) {
                    self.sounds_to_play.push(3);
                }
            }
            5 => {
                if get_bit(data, 0) {
                    self.sounds_to_play.push(4);
                }
                if get_bit(data, 1) {
                    self.sounds_to_play.push(5);
                }
                if get_bit(data, 2) {
                    self.sounds_to_play.push(6);
                }
                if get_bit(data, 3) {
                    self.sounds_to_play.push(7);
                }
                if get_bit(data, 4) {
                    self.sounds_to_play.push(8);
                }
            }
            _ => {}
        }
    }

    pub fn fetch_sound_to_play(&mut self) -> Option<u8> {
        if self.sounds_to_play.len() > 0 {
            Some(self.sounds_to_play.pop().unwrap())
        } else {
            None
        }
    }

    pub fn remove_all_sounds_to_play(&mut self) {
        self.sounds_to_play.clear();
    }
}
