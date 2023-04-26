#![allow(unused_variables)]

use js_sys::Error;

mod binary_lib;
mod my_webapi;
mod si_arcade;

// fn main() {
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::KeyboardEvent;

#[wasm_bindgen(start)]
pub fn initialize() -> Result<(), JsValue> {
    web_sys::console::log_1(&"Rust module loaded!".into());
    Ok(())
}

#[wasm_bindgen]
pub fn run(canvas_id: String) -> Result<(), JsValue> {
    /* Debug code */
    let array_h: [u8; 0x800] = include_bytes!("../game_roms/invaders.h").to_vec().try_into().unwrap();
    let array_g: [u8; 0x800] = include_bytes!("../game_roms/invaders.g").to_vec().try_into().unwrap();
    let array_f: [u8; 0x800] = include_bytes!("../game_roms/invaders.f").to_vec().try_into().unwrap();
    let array_e: [u8; 0x800] = include_bytes!("../game_roms/invaders.e").to_vec().try_into().unwrap();

    // let input_rom_h = get_input_element("input_rom_h");
    // let input_rom_g = get_input_element("input_rom_g");
    // let input_rom_f = get_input_element("input_rom_f");
    // let input_rom_e = get_input_element("input_rom_e");

    /* Debug code*/
    // let array_h: [u8; 0x800] = get_rom_from_input(&input_rom_h).unwrap();

    // If the four inputs are filled with the roms
    // if !input_rom_h.value().is_empty()
    //     && !input_rom_g.value().is_empty()
    //     && !input_rom_f.value().is_empty()
    //     && !input_rom_e.value().is_empty()
    {
        // let array_h: [u8; 0x800] = get_rom_from_input(&input_rom_h).unwrap();
        // let array_g: [u8; 0x800] = get_rom_from_input(&input_rom_g).unwrap();
        // let array_f: [u8; 0x800] = get_rom_from_input(&input_rom_f).unwrap();
        // let array_e: [u8; 0x800] = get_rom_from_input(&input_rom_e).unwrap();

        // If the four inputs are filled with the roms
        let space_invaders_arcade = Rc::new(RefCell::new(si_arcade::SpaceInvadersArcade::new(
            canvas_id, &array_h, &array_g, &array_f, &array_e,
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

        // let mut i = 0;
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

fn read_file(input: web_sys::HtmlInputElement) {}

fn get_rom_from_input(input: &web_sys::HtmlInputElement) -> Result<[u8; 0x800], Error> {
    let mut array: [u8; 0x800] = [0; 0x800];

    let files = input.files().expect("Error: No file selected.");
    let file = files.get(0).expect("Error: No file found.");

    web_sys::console::log_1(&format!("file.name = {}", file.name()).into());
    web_sys::console::log_1(&format!("file.size = {}", file.size()).into());

    let file_reader = web_sys::FileReader::new().unwrap();

    // let read_promise = wasm_bindgen_futures::JsFuture::from(file_reader.read_as_array_buffer(&file)).unwrap();
    //
    // read_promise.then(move |result| {
    //     let file_slice = js_sys::Uint8Array::new(&result);
    //     // Do something with the file_slice here...
    //     web_sys::console::log_1(&format!("file_slice.length = {}", file_slice.length()).into());
    //     Ok(())
    // })

    file_reader.read_as_array_buffer(&file).unwrap();

    // let file_slice = js_sys::Uint8Array::new_with_length(file.size() as u32);
    let file_slice =
        js_sys::Uint8Array::new_with_byte_offset_and_length(&file_reader.result().unwrap(), 0, file.size() as u32);

    web_sys::console::log_1(&format!("file_slice.length = {}", file_slice.length()).into());

    web_sys::console::log_1(&format!("file_reader_result = {:?}", file_reader.result().unwrap()).into());

    if file_slice.length() == 0x800 {
        file_slice.copy_to(&mut array[0..0x800]);

        for (i, byte) in array.iter().enumerate() {
            web_sys::console::log_1(&format!("array_h[{}] = {}", i, byte).into());
        }

        Ok(array)
    } else {
        web_sys::console::log_1(&format!("Error: Wrong file size").into());
        Err(Error::new("Wrong file size"))
    }
}
// }
