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
        let space_invaders_arcade = Rc::new(RefCell::new(si_arcade::SpaceInvadersArcade::new()));

        // Set up the keyboard event listener to handle key events
        {
            let space_invaders_arcade_ref = Rc::clone(&space_invaders_arcade);
            let closure = Closure::wrap(Box::new(move |event: KeyboardEvent| {
                let is_pressed = event.type_() == "keydown";
                match event.key().as_ref() {
                    "ArrowLeft" => space_invaders_arcade_ref
                        .borrow_mut()
                        .update_input(si_arcade::GameInput::Left, is_pressed),
                    "ArrowRight" => space_invaders_arcade_ref
                        .borrow_mut()
                        .update_input(si_arcade::GameInput::Right, is_pressed),
                    "ArrowUp" => space_invaders_arcade_ref
                        .borrow_mut()
                        .update_input(si_arcade::GameInput::Shot, is_pressed),
                    "c" => space_invaders_arcade_ref
                        .borrow_mut()
                        .update_input(si_arcade::GameInput::Coin, is_pressed),
                    "1" => space_invaders_arcade_ref
                        .borrow_mut()
                        .update_input(si_arcade::GameInput::Player1Start, is_pressed),
                    "2" => space_invaders_arcade_ref
                        .borrow_mut()
                        .update_input(si_arcade::GameInput::Player2Start, is_pressed),
                    "k" => space_invaders_arcade_ref
                        .borrow_mut()
                        .update_input(si_arcade::GameInput::Dip3, is_pressed),
                    "l" => space_invaders_arcade_ref
                        .borrow_mut()
                        .update_input(si_arcade::GameInput::Dip5, is_pressed),
                    "m" => space_invaders_arcade_ref
                        .borrow_mut()
                        .update_input(si_arcade::GameInput::Dip6, is_pressed),
                    "j" => space_invaders_arcade_ref
                        .borrow_mut()
                        .update_input(si_arcade::GameInput::Dip7, is_pressed),
                    _ => {}
                }
            }) as Box<dyn FnMut(_)>);

            window().add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())?;
            window().add_event_listener_with_callback("keyup", closure.as_ref().unchecked_ref())?;

            closure.forget();
        }

        let f = Rc::new(RefCell::new(None));
        let g = f.clone();

        let mut i = 0;
        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            // space_invaders_arcade.emulate_cycle();
            space_invaders_arcade.borrow_mut().emulate_cycle();
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
}
