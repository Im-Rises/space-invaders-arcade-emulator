use std::cell::RefCell;
use std::rc::Rc;

use crate::binary_lib::*;
use crate::my_webapi::{MyWebApi, SoundType};
pub use crate::si_arcade::inputs_outputs::GameInput;

mod cpu;
mod inputs_outputs;
mod mmu;
mod ppu;
mod spu;

const SCREEN_REFRESH_TIME: u128 = 16;
const INTERRUPT_VBLANK_COUNTER: usize = cpu::CLOCK_FREQUENCY / ppu::SCREEN_FREQUENCY;
const INTERRUPT_MIDDLE_VBLANK: usize = INTERRUPT_VBLANK_COUNTER / 2;

pub struct SpaceInvadersArcade {
    cpu: cpu::Cpu,
    ppu: ppu::Ppu,
    spu: spu::Spu,
    mmu: Rc<RefCell<mmu::Mmu>>,
    pub inputs_outputs: inputs_outputs::InputsOutputs,
    my_api: MyWebApi,
    frequency_counter: usize,
    last_frequency_counter: usize,
}

impl SpaceInvadersArcade {
    pub fn new(rom_h: &[u8; 0x800], rom_g: &[u8; 0x800], rom_f: &[u8; 0x800], rom_e: &[u8; 0x800]) -> Self {
        let mmu_init = Rc::new(RefCell::new(mmu::Mmu::new(rom_h, rom_g, rom_f, rom_e)));
        let mut sounds: Vec<(&[u8], SoundType)> = Vec::new();
        sounds.push((&spu::SOUND_0, SoundType::LoopSound));
        sounds.push((&spu::SOUND_1, SoundType::VariableLengthSound));
        sounds.push((&spu::SOUND_2, SoundType::UniqueSound));
        sounds.push((&spu::SOUND_3, SoundType::UniqueSound));
        sounds.push((&spu::SOUND_4, SoundType::UniqueSound));
        sounds.push((&spu::SOUND_5, SoundType::UniqueSound));
        sounds.push((&spu::SOUND_6, SoundType::UniqueSound));
        sounds.push((&spu::SOUND_7, SoundType::UniqueSound));
        sounds.push((&spu::SOUND_8, SoundType::UniqueSound));
        SpaceInvadersArcade {
            cpu: cpu::Cpu::new(&mmu_init, 0),
            ppu: ppu::Ppu::new(&mmu_init),
            spu: spu::Spu::new(),
            mmu: Rc::clone(&mmu_init),
            inputs_outputs: inputs_outputs::InputsOutputs::new(),
            my_api: MyWebApi::new(ppu::SCREEN_WIDTH as u32, ppu::SCREEN_HEIGHT as u32, sounds),
            frequency_counter: 0,
            last_frequency_counter: 0,
        }
    }

    pub fn emulate_cycle(&mut self) {
        // Handle CPU
        let mut do_loop = true;

        // Loop until we refresh the screen (16ms)
        while do_loop {
            if !self.cpu.get_halted() {
                if self.cpu.get_cycles() == 0 {
                    let opcode = self.cpu.fetch_opcode();
                    // println!("Opcode {}", opcode);
                    // web_sys::console::log_1(&format!("Opcode {}", opcode).into());
                    if opcode == 0xDB {
                        let port = self.cpu.fetch_byte();
                        let a = self.inputs(port, self.cpu.get_a());
                        self.cpu.set_a(a);
                        self.cpu.set_cycles(10);
                    } else if opcode == 0xd3 {
                        let port = self.cpu.fetch_byte();
                        self.outputs(port, self.cpu.get_a());
                        self.cpu.set_cycles(10);
                    } else {
                        let cycles = self.cpu.compute_opcode(opcode);
                        self.cpu.set_cycles(cycles);
                    }
                }
                self.cpu.set_cycles(self.cpu.get_cycles() - 1);
            }

            // Update frequency counter
            self.frequency_counter += 1;

            // Handle Interrupts and PPU
            if self.cpu.get_inte() {
                if self.frequency_counter > INTERRUPT_MIDDLE_VBLANK
                    && self.last_frequency_counter <= INTERRUPT_MIDDLE_VBLANK
                {
                    cpu::interrupts::interrupt(&mut self.cpu, 1);
                }
                if self.frequency_counter > INTERRUPT_VBLANK_COUNTER {
                    cpu::interrupts::interrupt(&mut self.cpu, 2);
                    self.frequency_counter = 0;
                    self.ppu.clock();
                    self.my_api.update_u8array_to_texture(
                        self.ppu.get_screen(),
                        ppu::SCREEN_WIDTH as i32,
                        ppu::SCREEN_HEIGHT as i32,
                    );
                    self.my_api.draw();
                    do_loop = false;
                }
            } else {
                self.frequency_counter = 0;
            }

            // Update last frequency counter
            self.last_frequency_counter = self.frequency_counter;
        }
    }

    fn inputs(&mut self, port: u8, mut data: u8) -> u8 {
        // Read inputs states to data
        match port {
            0 => {
                data = 0b0000_1110;
            } //INPUTS (Mapped in hardware but never used by the code)
            1 => {
                data = 0b0000_1000;
                data = set_reset_bit(data, 0, self.inputs_outputs.coin);
                data = set_reset_bit(data, 1, self.inputs_outputs.player2_start);
                data = set_reset_bit(data, 2, self.inputs_outputs.player1_start);
                data = set_reset_bit(data, 4, self.inputs_outputs.player.shot);
                data = set_reset_bit(data, 5, self.inputs_outputs.player.left);
                data = set_reset_bit(data, 6, self.inputs_outputs.player.right);
            }
            2 => {
                data = 0b0000_0000;
                data = set_reset_bit(data, 0, self.inputs_outputs.dip3);
                data = set_reset_bit(data, 1, self.inputs_outputs.dip5);
                data = set_reset_bit(data, 3, self.inputs_outputs.dip6);
                data = set_reset_bit(data, 4, self.inputs_outputs.player.shot); // player 2 shot
                data = set_reset_bit(data, 5, self.inputs_outputs.player.left); // player 2 left
                data = set_reset_bit(data, 6, self.inputs_outputs.player.right); // player 2 right
                data = set_reset_bit(data, 7, self.inputs_outputs.dip7);
            }
            3 => data = ((self.inputs_outputs.shift_register >> (8 - self.inputs_outputs.shift_offset)) & 0xFF) as u8,
            6 => (), //WATCHDOG
            _ => {
                panic!(
                    "Error: Writing to port not implemented at port {} with data {}",
                    port, data
                );
            }
        }

        data
    }

    fn outputs(&mut self, port: u8, data: u8) {
        match port {
            2 => self.inputs_outputs.shift_offset = data & 0b0000_0111,
            3 => {
                self.play_audio(port, data);
            }
            4 => self.inputs_outputs.shift_register = self.inputs_outputs.shift_register >> 8 | (data as u16) << 8,
            5 => {
                self.play_audio(port, data);
            }
            6 => (), //Watch dog
            _ => {
                panic!(
                    "Error: Reading from port not implemented at port {} with data {}",
                    port, data
                );
            }
        }
    }

    fn play_audio(&mut self, port: u8, data: u8) {
        self.spu.update(port, data);
        self.my_api.play_audio_sound(self.spu.get_sounds_states());
    }

    pub fn update_input(&mut self, game_input: inputs_outputs::GameInput, value: bool) {
        self.inputs_outputs.update_input(game_input, value);
    }

    pub fn get_screen(&self) -> &[u8; ppu::SCREEN_WIDTH * ppu::SCREEN_HEIGHT * 3] {
        self.ppu.get_screen()
    }

    pub fn get_si_arcade_screen_width_height(&self) -> (usize, usize) {
        (ppu::SCREEN_WIDTH, ppu::SCREEN_HEIGHT)
    }
}
