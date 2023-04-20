use std::cell::RefCell;
use std::rc::Rc;

use crate::binary_lib::*;
use crate::my_webgl2::MyWebGl2;

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
    mmu: Rc<RefCell<mmu::Mmu>>,
    pub inputs_outputs: inputs_outputs::InputsOutputs,
    my_webgl2: MyWebGl2,
    frequency_counter: usize,
    last_frequency_counter: usize,
}

impl SpaceInvadersArcade {
    pub fn new() -> SpaceInvadersArcade {
        let mmu_init = Rc::new(RefCell::new(mmu::Mmu::new()));
        SpaceInvadersArcade {
            cpu: cpu::Cpu::new(&mmu_init, 0),
            ppu: ppu::Ppu::new(&mmu_init),
            mmu: Rc::clone(&mmu_init),
            inputs_outputs: inputs_outputs::InputsOutputs::new(),
            my_webgl2: MyWebGl2::new(ppu::SCREEN_WIDTH as u32, ppu::SCREEN_HEIGHT as u32).unwrap(),
            frequency_counter: 0,
            last_frequency_counter: 0,
        }

        // let mut time = Instant::now();
        // let mut sdl2_video: my_sdl2::MySdl2 = my_sdl2::MySdl2::new(
        //     spu::SOUND_0,
        //     spu::SOUND_1,
        //     spu::SOUND_2,
        //     spu::SOUND_3,
        //     spu::SOUND_4,
        //     spu::SOUND_5,
        //     spu::SOUND_6,
        //     spu::SOUND_7,
        //     spu::SOUND_8,
        // );
    }

    pub fn emulate_cycle(&mut self) {
        // Handle CPU
        // while sdl2_video.get_window_active(self) {
        if !self.cpu.get_halted() {
            if self.cpu.get_cycles() == 0 {
                let opcode = self.cpu.fetch_opcode();
                // println!("Opcode {}", opcode);
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
                self.my_webgl2
                    .u8array_to_texture(
                        self.ppu.get_screen(),
                        ppu::SCREEN_WIDTH as i32,
                        ppu::SCREEN_WIDTH as i32,
                    )
                    .expect("Error cannot update texture");
                self.my_webgl2.draw();
                // if time.elapsed().as_millis() < SCREEN_REFRESH_TIME {
                //     std::thread::sleep(std::time::Duration::from_millis(SCREEN_REFRESH_TIME as u64 - time.elapsed().as_millis() as u64));
                // }
                // time = Instant::now();
            }
        } else {
            self.frequency_counter = 0;
        }

        self.last_frequency_counter = self.frequency_counter;
    }

    fn inputs(&mut self, port: u8, mut data: u8) -> u8 {
        match port {
            0 => {
                data = 0b0000_1110;
            } //INPUTS (Mapped in hardware but never used by the code)
            1 => {
                data = 0b0000_1000;
                data = set_reset_bit(data, 0, self.inputs_outputs.coin);
                data = set_reset_bit(data, 1, self.inputs_outputs.player2.start);
                data = set_reset_bit(data, 2, self.inputs_outputs.player1.start);
                data = set_reset_bit(data, 4, self.inputs_outputs.player1.shot);
                data = set_reset_bit(data, 5, self.inputs_outputs.player1.left);
                data = set_reset_bit(data, 6, self.inputs_outputs.player1.right);
            }
            2 => {
                data = 0b0000_0000;
                data = set_reset_bit(data, 0, self.inputs_outputs.dip3);
                data = set_reset_bit(data, 1, self.inputs_outputs.dip5);
                data = set_reset_bit(data, 3, self.inputs_outputs.dip6);
                data = set_reset_bit(data, 4, self.inputs_outputs.player2.shot);
                data = set_reset_bit(data, 5, self.inputs_outputs.player2.left);
                data = set_reset_bit(data, 6, self.inputs_outputs.player2.right);
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
                // sdl2_video.play_audio_sound(port, data);
            }
            4 => self.inputs_outputs.shift_register = self.inputs_outputs.shift_register >> 8 | (data as u16) << 8,
            5 => {
                // sdl2_video.play_audio_sound(port, data);
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

    pub fn get_screen(&self) -> &[u8; ppu::SCREEN_WIDTH * ppu::SCREEN_HEIGHT * 3] {
        self.ppu.get_screen()
    }

    pub fn get_si_arcade_screen_width_height(&self) -> (usize, usize) {
        (ppu::SCREEN_WIDTH, ppu::SCREEN_HEIGHT)
    }
}
