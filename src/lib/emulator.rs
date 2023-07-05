use std::cmp;
use std::collections::HashSet;

use rand::prelude::*;
use thiserror::Error;

use crate::beeper::Beeper;
use crate::instruction::Instruction;

pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;
pub const FPS: u32 = 60;

const MEMORY_SIZE: usize = 4096;
const PROGRAM_START_ADDRESS: usize = 512;
const FONT_START_ADDRESS: usize = 80;

#[derive(PartialEq, Eq, Error, Debug)]
pub enum EmulatorError {
    #[error("unknown instruction")]
    Instruction(),
    #[error("invalid memory access")]
    MemoryAccess,
    #[error("stack underflow")]
    StackUnderflow,
}

static FONT: [[u8; 5]; 16] = [
    [0xF0, 0x90, 0x90, 0x90, 0xF0], // 0
    [0x20, 0x60, 0x20, 0x20, 0x70], // 1
    [0xF0, 0x10, 0xF0, 0x80, 0xF0], // 2
    [0xF0, 0x10, 0xF0, 0x10, 0xF0], // 3
    [0x90, 0x90, 0xF0, 0x10, 0x10], // 4
    [0xF0, 0x80, 0xF0, 0x10, 0xF0], // 5
    [0xF0, 0x80, 0xF0, 0x90, 0xF0], // 6
    [0xF0, 0x10, 0x20, 0x40, 0x40], // 7
    [0xF0, 0x90, 0xF0, 0x90, 0xF0], // 8
    [0xF0, 0x90, 0xF0, 0x10, 0xF0], // 9
    [0xF0, 0x90, 0xF0, 0x90, 0x90], // A
    [0xE0, 0x90, 0xE0, 0x90, 0xE0], // B
    [0xF0, 0x80, 0x80, 0x80, 0xF0], // C
    [0xE0, 0x90, 0x90, 0x90, 0xE0], // D
    [0xF0, 0x80, 0xF0, 0x80, 0xF0], // E
    [0xF0, 0x80, 0xF0, 0x80, 0x80], // F
];

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Key {
    Key0,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    KeyA,
    KeyB,
    KeyC,
    KeyD,
    KeyE,
    KeyF,
}

impl Key {
    pub fn to_num(&self) -> u8 {
        match self {
            Key::Key0 => 0x0,
            Key::Key1 => 0x1,
            Key::Key2 => 0x2,
            Key::Key3 => 0x3,
            Key::Key4 => 0x4,
            Key::Key5 => 0x5,
            Key::Key6 => 0x6,
            Key::Key7 => 0x7,
            Key::Key8 => 0x8,
            Key::Key9 => 0x9,
            Key::KeyA => 0xA,
            Key::KeyB => 0xB,
            Key::KeyC => 0xC,
            Key::KeyD => 0xD,
            Key::KeyE => 0xE,
            Key::KeyF => 0xF,
        }
    }

    pub fn from_num(n: u8) -> Self {
        match n {
            0x0 => Key::Key0,
            0x1 => Key::Key1,
            0x2 => Key::Key2,
            0x3 => Key::Key3,
            0x4 => Key::Key4,
            0x5 => Key::Key5,
            0x6 => Key::Key6,
            0x7 => Key::Key7,
            0x8 => Key::Key8,
            0x9 => Key::Key9,
            0xA => Key::KeyA,
            0xB => Key::KeyB,
            0xC => Key::KeyC,
            0xD => Key::KeyD,
            0xE => Key::KeyE,
            0xF => Key::KeyF,
            _ => panic!(),
        }
    }
}

pub struct Emulator {
    memory: [u8; MEMORY_SIZE],
    stack: Vec<usize>,
    registers: [u8; 16],
    i: usize,
    program_counter: usize,
    delay_timer: u8,
    sound_timer: u8,
    frame_buf: [[bool; 64]; 32],

    keys_pressed: HashSet<Key>,

    inst_count: u8,
    ticks_per_frame: u8,
    timers_update_interval: u8,

    rand_num_gen: ThreadRng,
    beeper: Beeper,

    redraw: bool,
}

impl Emulator {
    pub fn new(clock_speed: u16, program: Vec<u8>) -> Result<Emulator, EmulatorError> {
        let ticks_per_frame = (clock_speed as f64 / FPS as f64).round() as u8;
        let timers_update_interval = (clock_speed as f64 / 60_f64).round() as u8;

        let mut e = Self {
            memory: [0; MEMORY_SIZE],
            stack: vec![],
            registers: [0; 16],
            i: 0,
            program_counter: PROGRAM_START_ADDRESS,
            delay_timer: 0,
            sound_timer: 0,
            frame_buf: [[false; WIDTH]; HEIGHT],

            keys_pressed: HashSet::new(),

            inst_count: 0,
            ticks_per_frame,
            timers_update_interval,

            rand_num_gen: thread_rng(),
            beeper: Beeper::new(),

            redraw: false,
        };
        e.write_to_memory(PROGRAM_START_ADDRESS, &program)?;
        e.write_to_memory(FONT_START_ADDRESS, &FONT.concat())?;

        Ok(e)
    }

    fn write_to_memory(&mut self, start_address: usize, buf: &[u8]) -> Result<(), EmulatorError> {
        if start_address + buf.len() > MEMORY_SIZE {
            return Err(EmulatorError::MemoryAccess);
        }
        for (mem, data) in self.memory[start_address..].iter_mut().zip(buf.iter()) {
            *mem = *data
        }
        Ok(())
    }

    fn draw_to_fb(&mut self, x: usize, y: usize, sprite: &[u8]) -> bool {
        let x = x & (WIDTH - 1);
        let y = y & (HEIGHT - 1);

        let row_iter = cmp::min(HEIGHT - y, sprite.len());
        let col_iter = cmp::min(WIDTH - x, 8);

        let mut any_px_erased = false;
        for row in 0..row_iter {
            let mut sprite_row = sprite[row];
            for col in 0..col_iter {
                let sprite_px_on = (sprite_row & 128) != 0;
                sprite_row <<= 1;

                if sprite_px_on {
                    let x_coord = x + col;
                    let y_coord = y + row;

                    if self.frame_buf[y_coord][x_coord] {
                        self.frame_buf[y_coord][x_coord] = false;
                        any_px_erased = true;
                    } else {
                        self.frame_buf[y_coord][x_coord] = true;
                    }
                }
            }
        }
        any_px_erased
    }

    fn clear_screen(&mut self) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                self.frame_buf[y][x] = false;
            }
        }
    }

    pub fn set_keys_pressed(&mut self, keys_pressed: HashSet<Key>) {
        self.keys_pressed = keys_pressed;
    }

    pub fn should_redraw(&self) -> bool {
        self.redraw
    }

    pub fn get_framebuffer(&self) -> &[[bool; WIDTH]; HEIGHT] {
        &self.frame_buf
    }

    pub fn run_frame(&mut self) -> Result<(), EmulatorError> {
        let mut redraw = false;
        for _ in 0..self.ticks_per_frame {
            redraw = self.tick()? || redraw;

            self.inst_count += 1;
            if self.inst_count == self.timers_update_interval {
                self.update_timers();
                self.inst_count = 0;
            }
        }
        self.redraw = redraw;
        Ok(())
    }

    /// returns true if a redraw is necessary
    pub fn tick(&mut self) -> Result<bool, EmulatorError> {
        let instruction_bytes = (
            self.memory[self.program_counter],
            self.memory[self.program_counter + 1],
        );
        self.program_counter += 2;

        let instruction = Instruction::parse(instruction_bytes)?;

        let mut redraw = false;
        match instruction {
            Instruction::ClearScreen => {
                self.clear_screen();
                redraw = true;
            }
            Instruction::Draw(x, y, n) => {
                let x_coord = self.registers[x] as usize;
                let y_coord = self.registers[y] as usize;
                let any_px_erased = self.draw_to_fb(
                    x_coord,
                    y_coord,
                    &self.memory[self.i..self.i + n].to_owned(),
                );
                if any_px_erased {
                    self.registers[0xF] = 1;
                } else {
                    self.registers[0xF] = 0;
                }
                redraw = true;
            }
            Instruction::Jump(adr) => {
                self.program_counter = adr;
            }
            Instruction::JumpWithOffset(adr) => {
                self.program_counter = adr + self.registers[0] as usize;
            }
            Instruction::Call(adr) => {
                self.stack.push(self.program_counter);
                self.program_counter = adr;
            }
            Instruction::Return => {
                let adr = self.stack.pop().ok_or(EmulatorError::StackUnderflow)?;
                self.program_counter = adr;
            }
            Instruction::SkipIfRegisterEqualsConstant(x, c) => {
                if self.registers[x] == c {
                    self.program_counter += 2;
                }
            }
            Instruction::SkipIfRegisterNotEqualsConstant(x, c) => {
                if self.registers[x] != c {
                    self.program_counter += 2;
                }
            }
            Instruction::SkipIfRegisterEqualsRegister(x, y) => {
                if self.registers[x] == self.registers[y] {
                    self.program_counter += 2;
                }
            }
            Instruction::SkipIfRegisterNotEqualsRegister(x, y) => {
                if self.registers[x] != self.registers[y] {
                    self.program_counter += 2;
                }
            }
            Instruction::SetRegisterToValue(x, value) => {
                self.registers[x] = value;
            }
            Instruction::SetRegisterToValueOfRegister(x, y) => {
                self.registers[x] = self.registers[y];
            }
            Instruction::BinaryOR(x, y) => {
                self.registers[x] |= self.registers[y];
            }
            Instruction::BinaryAND(x, y) => {
                self.registers[x] &= self.registers[y];
            }
            Instruction::BinaryXOR(x, y) => {
                self.registers[x] ^= self.registers[y];
            }
            Instruction::AddValueToRegister(x, value) => {
                self.registers[x] = self.registers[x].wrapping_add(value);
            }
            Instruction::AddRegisterToRegister(x, y) => {
                let sum = self.registers[x].wrapping_add(self.registers[y]);
                self.registers[0xF] = (sum < self.registers[x]) as u8;
                self.registers[x] = sum;
            }
            Instruction::SubstractXMinusY(x, y) => {
                let flag = (self.registers[x] > self.registers[y]) as u8;
                self.registers[x] = self.registers[x].wrapping_sub(self.registers[y]);
                self.registers[0xF] = flag;
            }
            Instruction::SubstractYMinusX(x, y) => {
                let flag = (self.registers[y] > self.registers[x]) as u8;
                self.registers[x] = self.registers[y].wrapping_sub(self.registers[x]);
                self.registers[0xF] = flag;
            }
            Instruction::ShiftRight(x, y) => {
                let flag = self.registers[y] & 1; // shifted out bit
                self.registers[x] = self.registers[y] >> 1;
                self.registers[0xF] = flag;
            }
            Instruction::ShiftLeft(x, y) => {
                let flag = (self.registers[y] & 128 != 0) as u8; // shifted out bit
                self.registers[x] = self.registers[y] << 1;
                self.registers[0xF] = flag;
            }
            Instruction::SkipIfKeyIsPressed(x) => {
                let key = Key::from_num(self.registers[x]);
                if self.keys_pressed.contains(&key) {
                    self.program_counter += 2;
                }
            }
            Instruction::SkipIfKeyIsNotPressed(x) => {
                let key = Key::from_num(self.registers[x]);
                if !self.keys_pressed.contains(&key) {
                    self.program_counter += 2;
                }
            }
            Instruction::GetKey(x) => {
                if self.keys_pressed.len() == 1 {
                    self.registers[x] = self.keys_pressed.iter().next().unwrap().to_num();
                } else {
                    self.program_counter -= 2;
                }
            }
            Instruction::GetDelayTimerValue(x) => {
                self.registers[x] = self.delay_timer;
            }
            Instruction::SetDelayTimer(x) => {
                self.delay_timer = self.registers[x];
            }
            Instruction::SetSoundTimer(x) => {
                self.sound_timer = self.registers[x];
                if self.sound_timer > 0 {
                    self.beeper.start();
                }
            }
            Instruction::StoreRegistersToMemory(end_index) => {
                self.write_to_memory(self.i.clone(), &self.registers[0..=end_index].to_owned())?;
            }
            Instruction::LoadRegistersFromMemory(end_index) => {
                for (mem, data) in self.memory[self.i..]
                    .iter()
                    .zip(self.registers[0..=end_index].iter_mut())
                {
                    *data = *mem;
                }
            }
            Instruction::SetIndexRegister(value) => {
                self.i = value;
            }
            Instruction::AddRegisterToIndexRegister(x) => {
                self.i += self.registers[x] as usize;
            }
            Instruction::LoadSprite(x) => {
                self.i = FONT_START_ADDRESS + self.registers[x] as usize * 5;
            }
            Instruction::BCD(x) => {
                let n = [
                    self.registers[x] / 100,
                    (self.registers[x] % 100) / 10,
                    self.registers[x] % 10,
                ];
                self.write_to_memory(self.i, &n)?;
            }
            Instruction::Random(x, c) => {
                self.registers[x] = self.rand_num_gen.gen::<u8>() & c;
            }
        }

        Ok(redraw)
    }

    fn update_timers(&mut self) {
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        } else {
            self.beeper.stop();
        }

        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
    }
}
