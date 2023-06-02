#![allow(unused_variables)]

use js_sys::Error;

mod binary_lib;
mod my_webapi;
mod si_arcade;

use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::KeyboardEvent;

const UPDATE_INTERVAL_MS: f64 = 1000.0 / 60.0;

#[wasm_bindgen(start)]
pub fn initialize() -> Result<(), JsValue> {
    web_sys::console::log_1(&"Rust module loaded!".into());
    Ok(())
}

#[wasm_bindgen]
pub fn run(
    canvas_id: String,
    display_mode: String,
    // rom_h: js_sys::Uint8Array,
    // rom_g: js_sys::Uint8Array,
    // rom_f: js_sys::Uint8Array,
    // rom_e: js_sys::Uint8Array,
) -> Result<(), JsValue> {
    // /* Debug code */
    let array_h: [u8; 0x800] = include_bytes!("../game_roms/invaders.h").to_vec().try_into().unwrap();
    let array_g: [u8; 0x800] = include_bytes!("../game_roms/invaders.g").to_vec().try_into().unwrap();
    let array_f: [u8; 0x800] = include_bytes!("../game_roms/invaders.f").to_vec().try_into().unwrap();
    let array_e: [u8; 0x800] = include_bytes!("../game_roms/invaders.e").to_vec().try_into().unwrap();

    // let array_h: [u8; 0x800] = rom_h.to_vec().try_into().unwrap();
    // let array_g: [u8; 0x800] = rom_g.to_vec().try_into().unwrap();
    // let array_f: [u8; 0x800] = rom_f.to_vec().try_into().unwrap();
    // let array_e: [u8; 0x800] = rom_e.to_vec().try_into().unwrap();

    // If the four inputs are filled with the roms
    let space_invaders_arcade = Rc::new(RefCell::new(si_arcade::SpaceInvadersArcade::new(
        canvas_id,
        display_mode,
        &array_h,
        &array_g,
        &array_f,
        &array_e,
    )));
    web_sys::console::log_1(&"SpaceInvadersArcade created!".into());

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

    // // VSync animation loop
    // let f = Rc::new(RefCell::new(None));
    // let g = f.clone();
    // *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
    //     space_invaders_arcade.borrow_mut().emulate_cycle();
    //     request_animation_frame(f.borrow().as_ref().unwrap());
    // }) as Box<dyn FnMut()>));
    //
    // request_animation_frame(g.borrow().as_ref().unwrap());

    // // Animation loop with fixed time step
    // let f = Rc::new(RefCell::new(None));
    // let g = f.clone();
    // let mut previous_time = window().performance().unwrap().now();
    // *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
    //     let current_time = window().performance().unwrap().now();
    //     let elapsed_time = current_time - previous_time;
    //
    //     if elapsed_time >= UPDATE_INTERVAL_MS {
    //         space_invaders_arcade.borrow_mut().emulate_cycle();
    //         previous_time = current_time;
    //     }
    //
    //     request_animation_frame(f.borrow().as_ref().unwrap());
    // }) as Box<dyn FnMut()>));
    //
    // request_animation_frame(g.borrow().as_ref().unwrap());

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
