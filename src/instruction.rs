use rand::Rng;

use crate::{
    blockid::{BlockId, BlockPattern},
    structure::Blocks,
};

#[derive(Debug, PartialEq, Eq, Hash)]
struct RegisterId(u8);

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    Call(BlockPattern),
    Return,
    If(RegisterId),
    Repeat(RegisterId),
    Not(RegisterId),
    Push(RegisterId),
    Pop(RegisterId),
    Inc(RegisterId),
    Dec(RegisterId),
    Store(RegisterId, RegisterId),
    Load(RegisterId, RegisterId),
    Add(RegisterId, RegisterId),
    Sub(RegisterId, RegisterId),
    Mul(RegisterId, RegisterId),
    Div(RegisterId, RegisterId),
    Eq(RegisterId, RegisterId),
    Gt(RegisterId, RegisterId),
    And(RegisterId, RegisterId),
    Or(RegisterId, RegisterId),
    Xor(RegisterId, RegisterId),
    Unknown0(RegisterId, RegisterId),
    Unknown1(RegisterId, RegisterId),
}

impl Instruction {
    fn decode(slice: &[u8], index: usize) -> Instruction {
        let bytecode = slice[index];
        let opcode = bytecode >> 4;
        match opcode {
            0 => {
                let operand = bytecode & 0b0000_1111;
                match operand {
                    0 => {
                        unreachable!()
                    }
                    1 => {
                        let block_pattern = BlockPattern::decode_backward(slice, index);
                        Instruction::Call(block_pattern)
                    }
                    2 => Instruction::Return,
                    3 => todo!(),
                    4..=7 => {
                        let register = RegisterId(operand & 0b0000_0011);
                        Instruction::If(register)
                    }
                    8..=11 => {
                        let register = RegisterId(operand & 0b0000_0011);
                        Instruction::Repeat(register)
                    }
                    12..=15 => {
                        let register = RegisterId(operand & 0b0000_0011);
                        Instruction::Not(register)
                    }
                    _ => unreachable!(),
                }
            }
            1 => {
                let register = RegisterId(bytecode & 0b0000_0011);
                match opcode {
                    0..=3 => Instruction::Push(register),
                    4..=7 => Instruction::Pop(register),
                    8..=11 => Instruction::Inc(register),
                    12..=15 => Instruction::Dec(register),
                    _ => unreachable!(),
                }
            }
            2..=14 => {
                let operand = bytecode & 0b0000_1111;
                let r0 = RegisterId((operand & 0b0000_1100) >> 2);
                let r1 = RegisterId(operand & 0b0000_0011);
                match opcode {
                    2 => Instruction::Store(r0, r1),
                    3 => Instruction::Load(r0, r1),
                    4 => Instruction::Add(r0, r1),
                    5 => Instruction::Sub(r0, r1),
                    6 => Instruction::Mul(r0, r1),
                    7 => Instruction::Div(r0, r1),
                    8 => Instruction::Eq(r0, r1),
                    9 => Instruction::Gt(r0, r1),
                    10 => Instruction::And(r0, r1),
                    11 => Instruction::Or(r0, r1),
                    12 => Instruction::Xor(r0, r1),
                    13 => Instruction::Unknown0(r0, r1),
                    14 => Instruction::Unknown1(r0, r1),
                    _ => unreachable!(),
                }
            }
            15 => {
                todo!()
            }
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::blockid::BlockPattern;

    use super::*;

    #[test]
    fn test_decode_call() {
        let data = [
            // byte 1
            0b0101_1000,
            // byte 2
            0b1010_1010,
            // byte 3
            0b0101_0101,
            // byte 4
            0b1111_1000,
            0b1111_0001,
            // call
            0b0000_0001,
        ];
        let index = data.len() - 1;
        let instruction = Instruction::decode(&data, index);
        assert_eq!(
            instruction,
            Instruction::Call(BlockPattern::new(0b0101_1000_1010_1010_0101_0101_1000_0001))
        );
    }

    #[test]
    fn test_decode_if_0() {
        let data = [0b0000_0100];
        let instruction = Instruction::decode(&data, 0);
        assert_eq!(instruction, Instruction::If(RegisterId(0)));
    }

    #[test]
    fn test_decode_if_3() {
        let data = [0b0000_0111];
        let instruction = Instruction::decode(&data, 0);
        assert_eq!(instruction, Instruction::If(RegisterId(3)));
    }

    #[test]
    fn test_add_r0_r1() {
        let data = [0b0100_0001];
        let instruction = Instruction::decode(&data, 0);
        assert_eq!(instruction, Instruction::Add(RegisterId(0), RegisterId(1)));
    }
}
