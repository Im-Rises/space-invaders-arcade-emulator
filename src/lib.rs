#![allow(unused_variables)]

mod binary_lib;
mod my_webapi;
mod si_arcade;

fn main() {
    use std::cell::RefCell;
    use std::rc::Rc;
    use wasm_bindgen::prelude::*;
    use wasm_bindgen::JsCast;
    use web_sys::{console, KeyboardEvent, Window};

    #[wasm_bindgen(start)]
    pub fn start() -> Result<(), JsValue> {
        let mut space_invaders_arcade = si_arcade::SpaceInvadersArcade::new();
        // web_sys::console::log_1(&format!("Started webassembly run").into());
        let f = Rc::new(RefCell::new(None));
        let g = f.clone();

        let mut i = 0;
        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            space_invaders_arcade.emulate_cycle();
            request_animation_frame(f.borrow().as_ref().unwrap());
        }) as Box<dyn FnMut()>));

        request_animation_frame(g.borrow().as_ref().unwrap());
        Ok(())
    }

    fn window() -> web_sys::Window {
        web_sys::window().expect("no global `window` exists")
    }

    fn request_animation_frame(f: &Closure<dyn FnMut()>) {
        window()
            .request_animation_frame(f.as_ref().unchecked_ref())
            .expect("should register `requestAnimationFrame` OK");
    }

    #[wasm_bindgen]
    pub fn handle_keydown(event: KeyboardEvent) {
        match event.key().as_ref() {
            "ArrowLeft" => web_sys::console::log_1(&"Left arrow key pressed".into()),
            "ArrowUp" => web_sys::console::log_1(&"Up arrow key pressed".into()),
            "ArrowRight" => web_sys::console::log_1(&"Right arrow key pressed".into()),
            // "ArrowDown" => web_sys::console::log_1(&"Down arrow key pressed".into()),
            "c" => web_sys::console::log_1(&"C key pressed".into()),
            "k" => web_sys::console::log_1(&"K key pressed".into()),
            "l" => web_sys::console::log_1(&"L key pressed".into()),
            "m" => web_sys::console::log_1(&"M key pressed".into()),
            "1" => web_sys::console::log_1(&"1 key pressed".into()),
            "2" => web_sys::console::log_1(&"2 key pressed".into()),
            _ => {}
        }
    }

    #[wasm_bindgen]
    pub fn handle_keyup(event: KeyboardEvent) {
        match event.key().as_ref() {
            "ArrowLeft" => web_sys::console::log_1(&"Left arrow key released".into()),
            "ArrowUp" => web_sys::console::log_1(&"Up arrow key released".into()),
            "ArrowRight" => web_sys::console::log_1(&"Right arrow key released".into()),
            // "ArrowDown" => web_sys::console::log_1(&"Down arrow key released".into()),
            "c" => web_sys::console::log_1(&"C key released".into()),
            "k" => web_sys::console::log_1(&"K key released".into()),
            "l" => web_sys::console::log_1(&"L key released".into()),
            "m" => web_sys::console::log_1(&"M key released".into()),
            "1" => web_sys::console::log_1(&"1 key released".into()),
            "2" => web_sys::console::log_1(&"2 key released".into()),
            _ => {}
        }
    }
}
