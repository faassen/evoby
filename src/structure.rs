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

    fn decode_byte_forward(data: &[u8], index: usize) -> (u8, usize) {
        if index >= data.len() {
            return (0, index);
        }
        let byte = data[index];
        if byte == 0 {
            return (0, index);
        }
        if is_pattern_byte(byte) {
            let high_nibble = byte & 0b0000_1111;
            if index < data.len() - 1 {
                let next_byte = data[index + 1];
                if is_pattern_byte(next_byte) {
                    // combine it with the current byte as the low nibble
                    let low_nibble = next_byte & 0b0000_1111;
                    return (high_nibble << 4 | low_nibble, index + 2);
                } else {
                    todo!();
                }
            } else {
                todo!();
            }
        }
        (byte, index + 1)
    }

    fn decode_byte_backward(data: &[u8], index: usize) -> (u8, usize) {
        if index == 0 {
            return (0, 0);
        }
        let mut index = index - 1;
        let byte = data[index];
        if is_pattern_byte(byte) {
            let low_nibble = byte & 0b0000_1111;
            if index > 0 {
                index -= 1;
                let byte = data[index];
                let high_nibble = if byte == 0 {
                    index += 1;
                    0
                } else {
                    byte & 0b0000_1111
                };
                return (high_nibble << 4 | low_nibble, index);
            } else {
                // high nibble is considered 0000
                return (low_nibble, index);
            }
        }
        if byte == 0 {
            return (0, index + 1);
        }
        // we have another byte before the index that is a pattern byte
        if index > 0 {
            let previous_byte = data[index - 1];
            if is_pattern_byte(previous_byte) {
                // combine it with the current byte as the high nibble
                let low_nibble = byte & 0b0000_1111;
                let high_nibble = previous_byte & 0b0000_1111;
                return (high_nibble << 4 | low_nibble, index - 1);
            }
        }
        (byte, index)
    }
}

fn is_pattern_byte(byte: u8) -> bool {
    byte >> 4 == 0b1111
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_byte_backward_simple() {
        let data = [0b0000_0001, 0b0000_0010, 0b0000_0100, 0b0000_1000];
        let index = data.len();
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
    fn test_decode_byte_backward_with_pattern_byte_at_start_of_array() {
        let data = [0b1111_0010];
        let index = data.len();
        let (byte, index) = BlockIdentifier::decode_byte_backward(&data, index);
        assert_eq!(byte, 0b0000_0010);
        assert_eq!(index, 0);
    }

    #[test]
    fn test_decode_byte_backward_with_pattern_byte_at_start_of_block() {
        let data = [0b0000_0000, 0b1111_0010];
        let index = data.len();
        let (byte, index) = BlockIdentifier::decode_byte_backward(&data, index);
        assert_eq!(byte, 0b0000_0010);
        assert_eq!(index, 1);
    }

    #[test]
    fn test_decode_byte_backward_with_lower_half_pattern_byte() {
        let data = [0b0000_0001, 0b1111_0010];
        let index = data.len();
        let (byte, index) = BlockIdentifier::decode_byte_backward(&data, index);
        assert_eq!(byte, 0b0001_0010);
        assert_eq!(index, 0);
    }

    #[test]
    fn test_decode_byte_backward_with_upper_half_pattern_byte() {
        let data = [0b1111_0001, 0b0000_0010];
        let index = data.len();
        let (byte, index) = BlockIdentifier::decode_byte_backward(&data, index);
        assert_eq!(byte, 0b0001_0010);
        assert_eq!(index, 0);
    }

    #[test]
    fn test_decode_byte_forward_simple() {
        let data = [0b0000_0001, 0b0000_0010, 0b0000_0100, 0b0000_1000];
        let index = 0;
        let (byte, index) = BlockIdentifier::decode_byte_forward(&data, index);
        assert_eq!(byte, 0b0000_0001);
        assert_eq!(index, 1);
    }

    #[test]
    fn test_decode_byte_forward_at_end_of_array() {
        let data = [0b0000_0001, 0b0000_0010, 0b0000_0100, 0b0000_1000];
        let index = data.len();
        let (byte, index) = BlockIdentifier::decode_byte_forward(&data, index);
        assert_eq!(byte, 0b0000_0000);
        assert_eq!(index, data.len());
    }

    #[test]
    fn test_decode_byte_forward_at_end_of_block() {
        let data = [0b0000_0001, 0b0000_0010, 0b0000_0100, 0b0000_0000];
        let index = data.len() - 1;
        let (byte, index) = BlockIdentifier::decode_byte_forward(&data, index);
        assert_eq!(byte, 0b0000_0000);
        assert_eq!(index, data.len() - 1);
    }

    #[test]
    fn test_decode_byte_forward_with_full_pattern_byte() {
        let data = [0b1111_0001, 0b1111_0010];
        let index = 0;
        let (byte, index) = BlockIdentifier::decode_byte_forward(&data, index);
        assert_eq!(byte, 0b0001_0010);
        assert_eq!(index, 2);
    }
}
