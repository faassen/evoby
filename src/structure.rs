use crate::blockid::BlockIdentifier;

struct Strand<'a> {
    blocks: Vec<Block<'a>>,
}

#[derive(Debug)]
struct Block<'a> {
    identifier: BlockIdentifier,
    slice: &'a [u8],
    // instructions: Vec<Instruction>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct RegisterId(u8);

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    Call(BlockIdentifier),
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
                    1 => Instruction::Call(BlockIdentifier::decode_backward(slice, index)),
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
                let r0 = RegisterId(bytecode & 0b0000_1100 >> 2);
                let r1 = RegisterId(bytecode & 0b0000_0011);
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
    // fn decode(slice: &[u8], index: usize) -> Instruction) {
    //     let opcode = slice[index] >> 4;
    //     match opcode {
    //         0 => {todo!()}
    //         1 => {todo!()}
    //         2..=14 => {
    //             todo!();
    //     }
    //     }
    // }
}

impl<'a> Strand<'a> {
    pub(crate) fn from_bytes(data: &'a [u8]) -> Strand<'a> {
        // split data by zero byte into slices
        // use the slice to construct the BlockIdentifier
        let slices = data.split(|&byte| byte == 0);
        let mut blocks = Vec::new();
        for slice in slices {
            let identifier = BlockIdentifier::decode_forward(slice, 0);
            let block = Block { identifier, slice };
            blocks.push(block);
        }
        Strand { blocks }
    }
}
