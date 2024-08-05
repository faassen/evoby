use crate::blockid::BlockIdentifier;

struct Strand<'a> {
    blocks: Vec<Block<'a>>,
}

#[derive(Debug)]
struct Block<'a> {
    identifier: BlockIdentifier,
    instruction_data: &'a [u8],
    // instructions: Vec<Instruction>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct RegisterId(u8);

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    Call(BlockIdentifier),
    Return,
    If_(RegisterId),
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

fn decode_instruction(data: &[u8], index: usize) -> Instruction {
    // let identifier = decode_block_identifier(data, index);
    // // first decode the opcode
    // let opcode = instruction >> 4;
    todo!();
}
