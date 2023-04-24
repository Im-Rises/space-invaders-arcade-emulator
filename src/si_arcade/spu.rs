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

//TODO:
// Create a function which returns the index of the audio to be played

pub fn get_audio_index(port: u8, data: u8) -> Option<usize> {
    match port {
        3 => {
            if get_bit(data, 0) {
                Some(0)
            } else if get_bit(data, 1) {
                Some(1)
            } else if get_bit(data, 2) {
                Some(2)
            } else if get_bit(data, 3) {
                Some(3)
            } else {
                None
            }
        }
        5 => {
            if get_bit(data, 0) {
                Some(4)
            } else if get_bit(data, 1) {
                Some(5)
            } else if get_bit(data, 2) {
                Some(6)
            } else if get_bit(data, 3) {
                Some(7)
            } else if get_bit(data, 4) {
                Some(8)
            } else {
                None
            }
        }
        _ => None,
    }
}

// pub fn play_audio_sound(&mut self, port: u8, data: u8) {
//     match port {
//         3 => {
//             if get_bit(data, 0) {
//                 self.sdl_audio.play_ufo();
//             }
//             if get_bit(data, 1) && !get_bit(self.port3_previous_outputs, 1) {
//                 self.sdl_audio.play_shot();
//             }
//             if get_bit(data, 2) && !get_bit(self.port3_previous_outputs, 2) {
//                 self.sdl_audio.play_player_die();
//             }
//             if get_bit(data, 3) && !get_bit(self.port3_previous_outputs, 3) {
//                 self.sdl_audio.play_invader_die();
//             }
//             self.port3_previous_outputs = data;
//         }
//         5 => {
//             if get_bit(data, 0) {
//                 self.sdl_audio.play_fleet_movement_1();
//             }
//             if get_bit(data, 1) {
//                 self.sdl_audio.play_fleet_movement_2();
//             }
//             if get_bit(data, 2) {
//                 self.sdl_audio.play_fleet_movement_3();
//             }
//             if get_bit(data, 3) {
//                 self.sdl_audio.play_fleet_movement_4();
//             }
//             if get_bit(data, 4) && !get_bit(self.port5_previous_outputs, 4) {
//                 self.sdl_audio.play_ufo_hit();
//             }
//             self.port5_previous_outputs = data;
//         }
//         _ => {
//             panic!("Error: Trying to use port {} as audio port", port)
//         }
//     }
// }
