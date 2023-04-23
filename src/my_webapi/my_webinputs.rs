use wasm_bindgen::prelude::*;
use web_sys::{console, window, KeyboardEvent};

pub struct MyWebInputs {
    one_additional_life_last_state: bool,
    two_additional_lives_last_state: bool,
    extraship_btn_last_state: bool,
}

impl MyWebInputs {
    pub fn new() -> MyWebInputs {
        MyWebInputs {
            one_additional_life_last_state: false,
            two_additional_lives_last_state: false,
            extraship_btn_last_state: false,
        }
    }

    // pub fn get_inputs(&self) -> u8 {
    //     // let mut inputs: u8 = 0;
    //     // if self.up_btn_last_state {
    //     //     inputs |= 0b0000_0001;
    //     // }
    //     // if self.down_btn_last_state {
    //     //     inputs |= 0b0000_0010;
    //     // }
    //     // if self.left_btn_last_state {
    //     //     inputs |= 0b0000_0100;
    //     // }
    //     // if self.right_btn_last_state {
    //     //     inputs |= 0b0000_1000;
    //     // }
    //     // if self.coin_btn_last_state {
    //     //     inputs |= 0b0001_0000;
    //     // }
    //     // if self.one_additional_life_last_state {
    //     //     inputs |= 0b0010_0000;
    //     // }
    //     // if self.two_additional_lives_last_state {
    //     //     inputs |= 0b0100_0000;
    //     // }
    //     // if self.extraship_btn_last_state {
    //     //     inputs |= 0b1000_0000;
    //     // }
    //     // inputs
    //
    //     0
    // }
}
