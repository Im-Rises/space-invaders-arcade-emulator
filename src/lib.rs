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
        /* Debug code */
        // let array_h: [u8; 0x800] = include_bytes!("../game_roms/invaders.h").to_vec().try_into().unwrap();
        // let array_g: [u8; 0x800] = include_bytes!("../game_roms/invaders.g").to_vec().try_into().unwrap();
        // let array_f: [u8; 0x800] = include_bytes!("../game_roms/invaders.f").to_vec().try_into().unwrap();
        // let array_e: [u8; 0x800] = include_bytes!("../game_roms/invaders.e").to_vec().try_into().unwrap();

        /* Release code */
        let input_rom_h = get_input_element("input_rom_h");
        let input_rom_g = get_input_element("input_rom_g");
        let input_rom_f = get_input_element("input_rom_f");
        let input_rom_e = get_input_element("input_rom_e");

        /* Debug code*/
        let array_h: [u8; 0x800] = get_rom_from_input(&input_rom_h);
        for (i, byte) in array_h.iter().enumerate() {
            web_sys::console::log_1(&format!("array_h[{}] = {}", i, byte).into());
        }

        // If the four inputs are filled with the roms
        if !input_rom_h.value().is_empty()
            && !input_rom_g.value().is_empty()
            && !input_rom_f.value().is_empty()
            && !input_rom_e.value().is_empty()
        {
            let array_h: [u8; 0x800] = get_rom_from_input(&input_rom_h);
            let array_g: [u8; 0x800] = get_rom_from_input(&input_rom_g);
            let array_f: [u8; 0x800] = get_rom_from_input(&input_rom_f);
            let array_e: [u8; 0x800] = get_rom_from_input(&input_rom_e);

            // If the four inputs are filled with the roms
            let space_invaders_arcade = Rc::new(RefCell::new(si_arcade::SpaceInvadersArcade::new(
                &array_h, &array_g, &array_f, &array_e,
            )));

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
                        // "k" => space_invaders_arcade_ref
                        //     .borrow_mut()
                        //     .update_input(si_arcade::GameInput::Dip3, is_pressed),
                        // "l" => space_invaders_arcade_ref
                        //     .borrow_mut()
                        //     .update_input(si_arcade::GameInput::Dip5, is_pressed),
                        // "m" => space_invaders_arcade_ref
                        //     .borrow_mut()
                        //     .update_input(si_arcade::GameInput::Dip6, is_pressed),
                        // "o" => space_invaders_arcade_ref
                        //     .borrow_mut()
                        //     .update_input(si_arcade::GameInput::Dip7, is_pressed),
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
        }

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

    fn get_input_element(id: &str) -> web_sys::HtmlInputElement {
        window()
            .document()
            .unwrap()
            .get_element_by_id(id)
            .unwrap()
            .dyn_into::<web_sys::HtmlInputElement>()
            .unwrap()
    }

    fn get_rom_from_input(input: &web_sys::HtmlInputElement) -> [u8; 0x800] {
        let mut array: [u8; 0x800] = [0; 0x800];
        let files = input.files().expect("Error: No file selected.");
        let file = files.get(0).expect("Error: No file found.");

        let file_reader = web_sys::FileReader::new().unwrap();
        file_reader.read_as_array_buffer(&file);

        let file_slice = js_sys::Uint8Array::new(&file_reader.result().unwrap());
        file_slice.copy_to(&mut array[0..0x800]);

        // let files = input.files().expect("Error: No file selected.");
        // let file = files.get(0).expect("Error: No file found.");
        // .slice()
        // .expect("Error: Failed to slice file.");

        // match file {
        //     None => {
        //         let file_blob = file.slice().expect("Error: Failed to slice file.");
        //         let u8jsarray = js_sys::Uint8Array::new(&file_slice);
        //         u8jsarray.copy_to(&mut array[0..0x800]);
        //     }
        //     _ => web_sys::console::log_1(&format!("Error: Failed to get file.").into()),
        // }

        // web_sys::console::log_1(&format!("file_slice.size = {}", file_slice.size()).into());
        // for i in file_slice {
        //     web_sys::console::log_1(&format!("i = {}", i).into());
        // }
        // let u8jsarray = js_sys::Uint8Array::new(&file_slice);
        // web_sys::console::log_1(&format!("u8jsarray.length = {}", u8jsarray.length()).into());
        // u8jsarray.copy_to(&mut array[0..0x800]);
        array
    }
}
