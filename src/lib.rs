#![allow(unused_variables)]

mod binary_lib;
mod my_webapi;
mod si_arcade;

use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::KeyboardEvent;

const UPDATE_INTERVAL_MS: i32 = 16; // 60Hz

#[wasm_bindgen(start)]
fn init() -> Result<(), JsValue> {
    web_sys::console::log_1(&"Hello from Space Invaders Emulator!".into());

    Ok(())
}

#[wasm_bindgen]
pub fn run(
    canvas_id: String,
    display_mode: String,
    one_extra_life: bool,
    two_extra_lives: bool,
    extra_ship_enabled_early: bool,
    coin_info_demo: bool,
    // rom_h: js_sys::Uint8Array,
    // rom_g: js_sys::Uint8Array,
    // rom_f: js_sys::Uint8Array,
    // rom_e: js_sys::Uint8Array,
) -> Result<(), JsValue> {
    /* Debug code */
    let array_h: [u8; 0x800] = include_bytes!("../game_roms/invaders.h").to_vec().try_into().unwrap();
    let array_g: [u8; 0x800] = include_bytes!("../game_roms/invaders.g").to_vec().try_into().unwrap();
    let array_f: [u8; 0x800] = include_bytes!("../game_roms/invaders.f").to_vec().try_into().unwrap();
    let array_e: [u8; 0x800] = include_bytes!("../game_roms/invaders.e").to_vec().try_into().unwrap();

    // let array_h: [u8; 0x800] = rom_h.to_vec().try_into().unwrap();
    // let array_g: [u8; 0x800] = rom_g.to_vec().try_into().unwrap();
    // let array_f: [u8; 0x800] = rom_f.to_vec().try_into().unwrap();
    // let array_e: [u8; 0x800] = rom_e.to_vec().try_into().unwrap();

    let space_invaders_arcade = Rc::new(RefCell::new(si_arcade::SpaceInvadersArcade::new(
        canvas_id,
        display_mode,
        &array_h,
        &array_g,
        &array_f,
        &array_e,
    )));

    // Set up the keyboard event listener to handle key events
    {
        let space_invaders_arcade_ref = Rc::clone(&space_invaders_arcade);
        let closure = Closure::wrap(Box::new(move |event: KeyboardEvent| {
            // Set the player inputs
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
                _ => {}
            }

            // Set the dip switches
            space_invaders_arcade_ref
                .borrow_mut()
                .update_input(si_arcade::GameInput::Dip3, one_extra_life);
            space_invaders_arcade_ref
                .borrow_mut()
                .update_input(si_arcade::GameInput::Dip5, two_extra_lives);
            space_invaders_arcade_ref
                .borrow_mut()
                .update_input(si_arcade::GameInput::Dip6, extra_ship_enabled_early);
            space_invaders_arcade_ref
                .borrow_mut()
                .update_input(si_arcade::GameInput::Dip7, coin_info_demo);
        }) as Box<dyn FnMut(_)>);

        // Add the event listener to the window
        window().add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())?;
        window().add_event_listener_with_callback("keyup", closure.as_ref().unchecked_ref())?;

        closure.forget();
    }

    // Set up the HTML buttons callback
    {
        let space_invaders_arcade_ref = Rc::clone(&space_invaders_arcade);

        let button_up = document().get_element_by_id("si-button-up").unwrap();
        let callback_up = Closure::wrap(Box::new(move || {
            space_invaders_arcade_ref
                .borrow_mut()
                .update_input(si_arcade::GameInput::Shot, true);
        }) as Box<dyn Fn()>);
        button_up.add_event_listener_with_callback("click", callback_up.as_ref().unchecked_ref())?;

        // let button_left = document().get_element_by_id("si-button-left").unwrap();
        // let button_right = document().get_element_by_id("si-button-right").unwrap();
        // let button_coin = document().get_element_by_id("si-button-coin").unwrap();
        // let button_1p = document().get_element_by_id("si-button-1p").unwrap();
        // let button_2p = document().get_element_by_id("si-button-2p").unwrap();

        callback_up.forget();
    }

    setup_clock(Rc::clone(&space_invaders_arcade))?;
    Ok(())
}

fn setup_clock(space_invaders_arcade: Rc<RefCell<si_arcade::SpaceInvadersArcade>>) -> Result<(), JsValue> {
    update_time(space_invaders_arcade.clone());
    let a = Closure::<dyn Fn()>::new(move || update_time(space_invaders_arcade.clone()));
    window().set_interval_with_callback_and_timeout_and_arguments_0(a.as_ref().unchecked_ref(), UPDATE_INTERVAL_MS)?;
    fn update_time(space_invaders_arcade: Rc<RefCell<si_arcade::SpaceInvadersArcade>>) {
        space_invaders_arcade.borrow_mut().emulate_cycle();
    }

    a.forget();

    Ok(())
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn document() -> web_sys::Document {
    window().document().expect("no global `document` exists")
}
