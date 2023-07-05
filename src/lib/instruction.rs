use crate::emulator::EmulatorError;

fn extract_address(instruction: (u8, u8)) -> usize {
    let first = ((instruction.0 & 0xF) as u16) << 8;
    let second = instruction.1 as u16;
    (first | second) as usize
}

fn extract_registers(instruction: (u8, u8)) -> (usize, usize) {
    let first = instruction.0 & 0x0F;
    let second = instruction.1 >> 4;
    (first as usize, second as usize)
}

fn extract_second_nibble(byte: u8) -> u8 {
    byte & 0x0F
}

#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    // screen
    ClearScreen,               // 00E0
    Draw(usize, usize, usize), // DXYN
    // control flow
    Jump(usize),                                   // 1NNN
    JumpWithOffset(usize),                         // BNNN
    Call(usize),                                   // 2NNN
    Return,                                        // 00EE
    SkipIfRegisterEqualsConstant(usize, u8),       // 3XNN
    SkipIfRegisterNotEqualsConstant(usize, u8),    // 4XNN
    SkipIfRegisterEqualsRegister(usize, usize),    // 5XY0
    SkipIfRegisterNotEqualsRegister(usize, usize), // 9XY0
    // assignment
    SetRegisterToValue(usize, u8),              // 6XNN
    SetRegisterToValueOfRegister(usize, usize), // 8XY0
    // logical and arithmetic
    BinaryOR(usize, usize),              // 8XY1
    BinaryAND(usize, usize),             // 8XY2
    BinaryXOR(usize, usize),             // 8XY3
    AddValueToRegister(usize, u8),       // 7XNN
    AddRegisterToRegister(usize, usize), // 8XY4
    SubstractXMinusY(usize, usize),      // 8XY5
    SubstractYMinusX(usize, usize),      // 8XY7
    ShiftRight(usize, usize),            // 8XY6
    ShiftLeft(usize, usize),             // 8XYE
    // keys
    SkipIfKeyIsPressed(usize),    // EX9E
    SkipIfKeyIsNotPressed(usize), // EXA1
    GetKey(usize),                // FX0A
    // timer and sound
    GetDelayTimerValue(usize), // FX07
    SetDelayTimer(usize),      // FX15
    SetSoundTimer(usize),      // FX18
    // memory
    StoreRegistersToMemory(usize),     // FX55
    LoadRegistersFromMemory(usize),    // FX65
    SetIndexRegister(usize),           // ANNN
    AddRegisterToIndexRegister(usize), // FX1E
    LoadSprite(usize),                 // FX29
    // misc
    BCD(usize),        // FX33
    Random(usize, u8), // CXNN
}

impl Instruction {
    pub fn parse(instruction: (u8, u8)) -> Result<Self, EmulatorError> {
        //let instruction_bytes = instruction.to_be_bytes();
        //println!("inst {:#02x}{:#02x}", instruction.0, instruction.1);
        let first_nibble = instruction.0 >> 4;
        let i = match first_nibble {
            0x0 => {
                if instruction.0 != 0 {
                    return Err(EmulatorError::Instruction());
                }
                match instruction.1 {
                    0xE0 => Self::ClearScreen,
                    0xEE => Self::Return,
                    _ => return Err(EmulatorError::Instruction()),
                }
            }
            0x1 => Self::Jump(extract_address(instruction)),
            0x2 => Self::Call(extract_address(instruction)),
            0x3 => Self::SkipIfRegisterEqualsConstant(
                extract_second_nibble(instruction.0) as usize,
                instruction.1,
            ),
            0x4 => Self::SkipIfRegisterNotEqualsConstant(
                extract_second_nibble(instruction.0) as usize,
                instruction.1,
            ),
            0x5 => {
                if extract_second_nibble(instruction.1) == 0 {
                    let (x, y) = extract_registers(instruction);
                    Self::SkipIfRegisterEqualsRegister(x, y)
                } else {
                    return Err(EmulatorError::Instruction());
                }
            }
            0x6 => Self::SetRegisterToValue(
                extract_second_nibble(instruction.0) as usize,
                instruction.1,
            ),
            0x7 => Self::AddValueToRegister(
                extract_second_nibble(instruction.0) as usize,
                instruction.1,
            ),
            0x8 => {
                let fourth_nibble = extract_second_nibble(instruction.1);
                let (x, y) = extract_registers(instruction);
                match fourth_nibble {
                    0x0 => Self::SetRegisterToValueOfRegister(x, y),
                    0x1 => Self::BinaryOR(x, y),
                    0x2 => Self::BinaryAND(x, y),
                    0x3 => Self::BinaryXOR(x, y),
                    0x4 => Self::AddRegisterToRegister(x, y),
                    0x5 => Self::SubstractXMinusY(x, y),
                    0x6 => Self::ShiftRight(x, y),
                    0x7 => Self::SubstractYMinusX(x, y),
                    0xE => Self::ShiftLeft(x, y),
                    _ => return Err(EmulatorError::Instruction()),
                }
            }
            0x9 => {
                if extract_second_nibble(instruction.1) == 0 {
                    let (x, y) = extract_registers(instruction);
                    Self::SkipIfRegisterNotEqualsRegister(x, y)
                } else {
                    return Err(EmulatorError::Instruction());
                }
            }
            0xA => Self::SetIndexRegister(extract_address(instruction)),
            0xB => Self::JumpWithOffset(extract_address(instruction)),
            0xC => Self::Random(extract_second_nibble(instruction.0) as usize, instruction.1),
            0xD => {
                let (x, y) = extract_registers(instruction);
                Self::Draw(x, y, extract_second_nibble(instruction.1) as usize)
            }
            0xE => {
                let x = extract_second_nibble(instruction.0) as usize;
                match instruction.1 {
                    0x9E => Self::SkipIfKeyIsPressed(x),
                    0xA1 => Self::SkipIfKeyIsNotPressed(x),
                    _ => return Err(EmulatorError::Instruction()),
                }
            }
            0xF => {
                let x = extract_second_nibble(instruction.0) as usize;
                match instruction.1 {
                    0x07 => Self::GetDelayTimerValue(x),
                    0x0A => Self::GetKey(x),
                    0x15 => Self::SetDelayTimer(x),
                    0x18 => Self::SetSoundTimer(x),
                    0x1E => Self::AddRegisterToIndexRegister(x),
                    0x29 => Self::LoadSprite(x),
                    0x33 => Self::BCD(x),
                    0x55 => Self::StoreRegistersToMemory(x),
                    0x65 => Self::LoadRegistersFromMemory(x),
                    _ => return Err(EmulatorError::Instruction()),
                }
            }
            _ => unreachable!(),
        };
        Ok(i)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() {
        let tests = [
            ((0x00, 0xE0), Instruction::ClearScreen),
            ((0x00, 0xEE), Instruction::Return),
            ((0x12, 0x34), Instruction::Jump(0x234)),
            ((0x29, 0x32), Instruction::Call(0x932)),
            (
                (0x36, 0x22),
                Instruction::SkipIfRegisterEqualsConstant(6, 0x22),
            ),
            (
                (0x4A, 0x99),
                Instruction::SkipIfRegisterNotEqualsConstant(0xA, 0x99),
            ),
            (
                (0x52, 0x30),
                Instruction::SkipIfRegisterEqualsRegister(2, 3),
            ),
            ((0x62, 0x10), Instruction::SetRegisterToValue(0x2, 0x10)),
            ((0x7E, 0x69), Instruction::AddValueToRegister(0xE, 0x69)),
            (
                (0x8B, 0x30),
                Instruction::SetRegisterToValueOfRegister(0xB, 0x3),
            ),
            ((0x8E, 0x41), Instruction::BinaryOR(0xE, 0x4)),
            ((0x8D, 0x52), Instruction::BinaryAND(0xD, 0x5)),
            ((0x8C, 0x63), Instruction::BinaryXOR(0xC, 0x6)),
            ((0x8B, 0x74), Instruction::AddRegisterToRegister(0xB, 0x7)),
            ((0x8A, 0x85), Instruction::SubstractXMinusY(0xA, 0x8)),
            ((0x89, 0x96), Instruction::ShiftRight(0x9, 0x9)),
            ((0x88, 0xA7), Instruction::SubstractYMinusX(0x8, 0xA)),
            ((0x87, 0xBE), Instruction::ShiftLeft(0x7, 0xB)),
            (
                (0x9B, 0xC0),
                Instruction::SkipIfRegisterNotEqualsRegister(0xB, 0xC),
            ),
            ((0xA3, 0x15), Instruction::SetIndexRegister(0x315)),
            ((0xB5, 0x17), Instruction::JumpWithOffset(0x517)),
            ((0xC4, 0xA0), Instruction::Random(0x4, 0xA0)),
            ((0xD2, 0x91), Instruction::Draw(0x2, 0x9, 0x1)),
            ((0xE1, 0x9E), Instruction::SkipIfKeyIsPressed(0x1)),
            ((0xE2, 0xA1), Instruction::SkipIfKeyIsNotPressed(0x2)),
            ((0xFF, 0x07), Instruction::GetDelayTimerValue(0xF)),
            ((0xFA, 0x0A), Instruction::GetKey(0xA)),
            ((0xF7, 0x15), Instruction::SetDelayTimer(0x7)),
            ((0xF6, 0x18), Instruction::SetSoundTimer(0x6)),
            ((0xF5, 0x1E), Instruction::AddRegisterToIndexRegister(0x5)),
            ((0xF4, 0x29), Instruction::LoadSprite(0x4)),
            ((0xF3, 0x33), Instruction::BCD(0x3)),
            ((0xF2, 0x55), Instruction::StoreRegistersToMemory(0x2)),
            ((0xF1, 0x65), Instruction::LoadRegistersFromMemory(0x1)),
        ];

        for (i, expected) in tests {
            let actual = Instruction::parse(i).unwrap();
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_invalid() {
        let tests = [
            (0x00, 0xE1),
            (0x5A, 0xA2),
            (0x81, 0xFF),
            (0x94, 0x5F),
            (0xEA, 0xAA),
            (0xF8, 0x66),
        ];

        for i in tests {
            let result = Instruction::parse(i);
            assert_eq!(result, Err(EmulatorError::Instruction()));
        }
    }
}
