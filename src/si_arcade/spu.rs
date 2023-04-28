use crate::binary_lib::get_bit;

// UFO
// pub const SOUND_0: &[u8] = include_bytes!("../../game_audios/0.wav");
pub const SOUND_0: &[u8] = include_bytes!("../../game_audios/ufo_loop.wav");
// Shoot
// pub const SOUND_1: &[u8] = include_bytes!("../../game_audios/1.wav");
pub const SOUND_1: &[u8] = include_bytes!("../../game_audios/shoot_v1.wav");
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
// Extra ship sound
pub const SOUND_9: &[u8] = include_bytes!("../../game_audios/9.wav");

pub struct Spu {
    sounds_states: Vec<bool>,
}

impl Spu {
    pub fn new() -> Spu {
        Spu {
            sounds_states: vec![false; 9],
        }
    }

    pub fn update(&mut self, port: u8, data: u8) {
        match port {
            3 => {
                self.sounds_states[0] = get_bit(data, 0); // UFO
                self.sounds_states[1] = get_bit(data, 1); // Shoot
                self.sounds_states[2] = get_bit(data, 2); // Player Explosion
                self.sounds_states[3] = get_bit(data, 3); // Invader Explosion
                self.sounds_states[9] = get_bit(data, 4); // Extra Ship Sound
            }
            5 => {
                self.sounds_states[4] = get_bit(data, 0); // Invaders March 1
                self.sounds_states[5] = get_bit(data, 1); // Invaders March 2
                self.sounds_states[6] = get_bit(data, 2); // Invaders March 3
                self.sounds_states[7] = get_bit(data, 3); // Invaders March 4
                self.sounds_states[8] = get_bit(data, 4); // Bonus UFO Destroyed
            }
            _ => {}
        }
    }

    pub fn get_sounds_states(&self) -> &[bool] {
        &self.sounds_states
    }

    // pub fn fetch_sound_to_play(&mut self) -> Option<u8> {
    //     if self.sounds_to_play.len() > 0 {
    //         Some(self.sounds_to_play.pop().unwrap())
    //     } else {
    //         None
    //     }
    // }

    // pub fn remove_all_sounds_to_play(&mut self) {
    //     self.sounds_to_play.clear();
    // }
}
