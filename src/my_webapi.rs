use wasm_bindgen::{JsCast, JsValue};

mod my_webaudio;
mod my_webgl2;

pub struct MyWebApi {
    my_webgl2: my_webgl2::MyWebGl2,
    my_webaudio: my_webaudio::MyWebAudio,
}

impl MyWebApi {
    pub fn new(canvas_width: u32, canvas_height: u32) -> MyWebApi {
        MyWebApi {
            my_webgl2: my_webgl2::MyWebGl2::new(canvas_width, canvas_height).unwrap(),
            my_webaudio: my_webaudio::MyWebAudio::new().unwrap(),
        }
    }

    pub fn update_u8array_to_texture(&self, data: &[u8], width: i32, height: i32) {
        self.my_webgl2.u8array_to_texture(data, width, height).unwrap();
    }

    pub fn draw(&self) {
        self.my_webgl2.draw();
    }
}
