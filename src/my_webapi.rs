mod my_webaudio;
mod my_webgl2;

pub use crate::my_webapi::my_webaudio::SoundType;

pub struct MyWebApi {
    my_webgl2: my_webgl2::MyWebGl2,
    my_webaudio: my_webaudio::MyWebAudio,
}

impl MyWebApi {
    pub fn new(canvas_id: String, canvas_width: u32, canvas_height: u32, sounds: Vec<(&[u8], SoundType)>) -> MyWebApi {
        MyWebApi {
            my_webgl2: my_webgl2::MyWebGl2::new(canvas_id, canvas_width, canvas_height).unwrap(),
            my_webaudio: my_webaudio::MyWebAudio::new(sounds),
        }
    }

    pub fn update_u8array_to_game_texture(&self, data: &[u8], width: i32, height: i32) {
        self.my_webgl2.u8array_to_game_texture(data, width, height).unwrap();
    }

    pub fn update_u8array_to_overlay_texture(&self, data: &[u8], width: i32, height: i32) {
        // self.my_webgl2.u8array_to_overlay_texture(data, width, height).unwrap();
    }

    pub fn draw(&self) {
        self.my_webgl2.draw();
    }

    pub fn play_audio_sound(&mut self, sounds_states: &[bool]) {
        self.my_webaudio.play_sounds(sounds_states);
    }
}
