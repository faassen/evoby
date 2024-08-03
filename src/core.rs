#[derive(Debug, PartialEq, Eq)]
enum OpCode {
    Special0 = 0,
    Special1,
    Store,
    Load,
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Gt,
    And,
    Or,
    Xor,
    Unknown0,
    Unknown1,
    Pattern,
}

#[derive(Debug, PartialEq, Eq)]
enum Special0 {
    Block = 0,
    Call,
    Return,
    Value,
    IfR0,
    IfR1,
    IfR2,
    IfR3,
    RepeatR0,
    RepeatR1,
    RepeatR2,
    RepeatR3,
    NotR0,
    NotR1,
    NotR2,
    NotR3,
}

#[derive(Debug, PartialEq, Eq)]
enum Special1 {
    PushR0 = 0,
    PushR1,
    PushR2,
    PushR3,
    PopR0,
    PopR1,
    PopR2,
    PopR3,
    IncR0,
    IncR1,
    IncR2,
    IncR3,
    DecR0,
    DecR1,
    DecR2,
    DecR3,
}

#[derive(Debug, PartialEq, Eq)]
struct OneRegisterOperand {
    reg: u8,
}

#[derive(Debug, PartialEq, Eq)]
struct TwoRegisterOperand {
    reg0: u8,
    reg1: u8,
}

#[derive(Debug, PartialEq, Eq)]
struct BitPattern {
    pattern: u8,
}

// take the high nibble of the instruction and decode the opcode
fn decode_opcode(instruction: u8) -> OpCode {
    let opcode = instruction >> 4;

    match opcode {
        0 => OpCode::Special0,
        1 => OpCode::Special1,
        2 => OpCode::Store,
        3 => OpCode::Load,
        4 => OpCode::Add,
        5 => OpCode::Sub,
        6 => OpCode::Mul,
        7 => OpCode::Div,
        8 => OpCode::Eq,
        9 => OpCode::Gt,
        10 => OpCode::And,
        11 => OpCode::Or,
        12 => OpCode::Xor,
        13 => OpCode::Unknown0,
        14 => OpCode::Unknown1,
        15 => OpCode::Pattern,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // test decode opcode function
    #[test]
    fn test_decode_opcode_special0() {
        let instruction = 0b0000_0000;
        let opcode = decode_opcode(instruction);
        assert_eq!(opcode, OpCode::Special0);
    }

    #[test]
    fn test_decode_opcode_special1() {
        let instruction = 0b0001_0000;
        let opcode = decode_opcode(instruction);
        assert_eq!(opcode, OpCode::Special1);
    }

    #[test]
    fn test_decode_opcode_store() {
        let instruction = 0b0010_0000;
        let opcode = decode_opcode(instruction);
        assert_eq!(opcode, OpCode::Store);
    }

    #[test]
    fn test_decode_opcode_load() {
        let instruction = 0b0011_0000;
        let opcode = decode_opcode(instruction);
        assert_eq!(opcode, OpCode::Load);
    }
}
