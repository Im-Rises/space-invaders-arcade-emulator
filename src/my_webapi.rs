mod my_webaudio;
mod my_webgl2;
mod my_webinputs;

// pub use crate::my_webapi::my_webinputs::GameInput;
// use crate::si_arcade::InputsOutputs;
// use crate::si_arcade::SpaceInvadersArcade;

// pub enum WebApiKey {
//     ArrowLeft,
//     ArrowUp,
//     ArrowRight,
//     // ArrowDown,
//     C,
//     K,
//     L,
//     M,
//     One,
//     Two,
// }

pub struct MyWebApi {
    my_webgl2: my_webgl2::MyWebGl2,
    my_webaudio: my_webaudio::MyWebAudio,
    // my_webinputs: my_webinputs::MyWebInputs,
}

impl MyWebApi {
    pub fn new(canvas_width: u32, canvas_height: u32) -> MyWebApi {
        MyWebApi {
            my_webgl2: my_webgl2::MyWebGl2::new(canvas_width, canvas_height).unwrap(),
            my_webaudio: my_webaudio::MyWebAudio::new(),
            // my_webinputs: my_webinputs::MyWebInputs::new(),
        }
    }

    pub fn update_u8array_to_texture(&self, data: &[u8], width: i32, height: i32) {
        self.my_webgl2.u8array_to_texture(data, width, height).unwrap();
    }

    pub fn draw(&self) {
        self.my_webgl2.draw();
    }

    pub fn play_audio_sound(&self, port: u8, data: u8) {
        // self.my_webaudio.play_sound(port, data);
    }

    // pub fn update_input(&mut self, inputs_outputs: &mut InputsOutputs, input_index: GameInput, value: bool) {
    // my_webinputs::update_input(inputs_outputs, input_index, value);
    // }
}
