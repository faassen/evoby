struct Strand<'a> {
    blocks: Vec<Block<'a>>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct BlockIdentifier(u32);

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

impl BlockIdentifier {
    fn decode(data: &[u8], index: usize) -> BlockIdentifier {
        todo!();
    }

    fn decode_byte_backward(data: &[u8], index: usize) -> (u8, usize) {
        if index == 0 {
            return (0, 0);
        }
        let mut index = index - 1;
        let byte = data[index];
        if byte >> 4 == 0b1111 {
            let low_nibble = byte & 0b0000_1111;
            if index > 0 {
                index -= 1;
                let byte = data[index];
                let high_nibble = byte & 0b0000_1111;
                return (high_nibble << 4 | low_nibble, index);
            } else {
                // high nibble is considered 0000
                return (low_nibble, index);
            }
        }
        if byte == 0 {
            return (0, index + 1);
        }
        (byte, index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_byte_backward_simple() {
        let data = [0b0000_0001, 0b0000_0010, 0b0000_0100, 0b0000_1000];
        // index starts at the end
        let index = data.len();
        // now we get a single byte from the instruction stream, going backwards
        let (byte, index) = BlockIdentifier::decode_byte_backward(&data, index);
        assert_eq!(byte, 0b0000_1000);
        assert_eq!(index, data.len() - 1);
    }

    #[test]
    fn test_decode_byte_backward_at_start_of_array() {
        let data = [0b0000_0001, 0b0000_0010, 0b0000_0100, 0b0000_1000];
        let index = 0;
        let (byte, index) = BlockIdentifier::decode_byte_backward(&data, index);
        // we're at the start of the array, so we read 0
        assert_eq!(byte, 0b0000_0000);
        assert_eq!(index, 0);
    }

    #[test]
    fn test_decode_byte_backward_at_start_of_block() {
        let data = [0b0000_0000, 0b0000_0010, 0b0000_0100, 0b0000_1000];
        let index = 1;
        let (byte, index) = BlockIdentifier::decode_byte_backward(&data, index);
        // at start of block we also read 0 and don't move index
        assert_eq!(byte, 0b0000_0000);
        assert_eq!(index, 1);
    }

    #[test]
    fn test_decode_byte_backward_on_start_of_block() {
        // shouldn't be possible to reach this state, but let's test it anyway
        let data = [0b0000_0001, 0b0000_0000, 0b0000_0100, 0b0000_1000];
        let index = 1;
        let (byte, index) = BlockIdentifier::decode_byte_backward(&data, index);
        assert_eq!(byte, 0b0000_0001);
        assert_eq!(index, 0);
    }

    #[test]
    fn test_decode_byte_backward_with_full_pattern_byte() {
        let data = [0b1111_0001, 0b1111_0010];
        let index = data.len();
        let (byte, index) = BlockIdentifier::decode_byte_backward(&data, index);
        assert_eq!(byte, 0b0001_0010);
        assert_eq!(index, 0);
    }

    #[test]
    fn test_decode_byte_backward_with_pattern_byte_at_start_of_block() {
        let data = [0b1111_0010];
        let index = data.len();
        let (byte, index) = BlockIdentifier::decode_byte_backward(&data, index);
        assert_eq!(byte, 0b0000_0010);
        assert_eq!(index, 0);
    }

    // #[test]
    // fn test_decode_byte_backward_with_full_pattern_byte_at_start_of_block() {
    //     let data = [0b1111_0001, 0b0000_0010];
    //     let index = 1;
    //     let (byte, index) = BlockIdentifier::decode_byte_backward(&data, index);
    //     assert_eq!(byte, 0b0001_0010);
    //     assert_eq!(index, 0);
    // }

    // #[test]
    // fn test_decode_byte_backward_with_full_pattern_byte_at_start_of_array() {
    //     let data = [0b1111_0001, 0b0000_0010];
    //     let index = 0;
    //     let (byte, index) = BlockIdentifier::decode_byte_backward(&data, index);
    //     assert_eq!(byte, 0b0001_0010);
    //     assert_eq!(index, 0);
    // }

    // #[test]
    // fn test_decode_block_identifier_4_bytes() {
    //     let data = [0b0000_0001, 0b0000_0010, 0b0000_0100, 0b0000_1000];
    //     // index starts at the end
    //     let index = data.len();
    //     // we go backwards from index to construct u32
    //     // little-endian architecture, so the first byte we is the most significant,
    //     // as it comes last
    //     let byte = BlockIdentifier::decode_byte(data, index);
    // }
}
