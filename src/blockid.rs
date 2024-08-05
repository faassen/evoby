#[derive(Debug, PartialEq, Eq, Hash)]
pub(crate) struct BlockIdentifier(u32);

impl BlockIdentifier {
    pub(crate) fn decode_forward(data: &[u8], index: usize) -> BlockIdentifier {
        let (first_byte, index) = Self::decode_byte_forward(data, index);
        let (second_byte, index) = Self::decode_byte_forward(data, index);
        let (third_byte, index) = Self::decode_byte_forward(data, index);
        let (fourth_byte, _index) = Self::decode_byte_forward(data, index);
        BlockIdentifier(
            (first_byte as u32) << 24
                | (second_byte as u32) << 16
                | (third_byte as u32) << 8
                | fourth_byte as u32,
        )
    }

    pub(crate) fn decode_backward(data: &[u8], index: usize) -> BlockIdentifier {
        let (fourth_byte, index) = Self::decode_byte_backward(data, index);
        let (third_byte, index) = Self::decode_byte_backward(data, index);
        let (second_byte, index) = Self::decode_byte_backward(data, index);
        let (first_byte, _index) = Self::decode_byte_backward(data, index);
        BlockIdentifier(
            (first_byte as u32) << 24
                | (second_byte as u32) << 16
                | (third_byte as u32) << 8
                | fourth_byte as u32,
        )
    }

    fn decode_byte_forward(data: &[u8], index: usize) -> (u8, usize) {
        if index >= data.len() {
            return (0, index);
        }
        let byte = data[index];
        if is_pattern_byte(byte) {
            let high_nibble = byte & 0b0000_1111;
            if index < data.len() - 1 {
                let next_byte = data[index + 1];
                // combine it with the current byte as the low nibble
                let low_nibble = next_byte & 0b0000_1111;
                return (high_nibble << 4 | low_nibble, index + 2);
            } else {
                return (high_nibble << 4, index + 1);
            }
        }

        // we have another byte before the end that is a pattern byte
        if index < data.len() - 1 {
            let next_byte = data[index + 1];
            if is_pattern_byte(next_byte) {
                // combine it with the current byte as the low nibble
                let high_nibble = byte & 0b0000_1111;
                let low_nibble = next_byte & 0b0000_1111;
                return (high_nibble << 4 | low_nibble, index + 2);
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
                let high_nibble = byte & 0b0000_1111;
                return (high_nibble << 4 | low_nibble, index);
            } else {
                // high nibble is considered 0000
                return (low_nibble, index);
            }
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
    fn test_decode_byte_forward_with_full_pattern_byte() {
        let data = [0b1111_0001, 0b1111_0010];
        let index = 0;
        let (byte, index) = BlockIdentifier::decode_byte_forward(&data, index);
        assert_eq!(byte, 0b0001_0010);
        assert_eq!(index, 2);
    }

    #[test]
    fn test_decode_byte_forward_with_pattern_byte_at_end_of_array() {
        let data = [0b1111_0001];
        let index = 0;
        let (byte, index) = BlockIdentifier::decode_byte_forward(&data, index);
        assert_eq!(byte, 0b0001_0000);
        assert_eq!(index, 1);
    }

    #[test]
    fn test_decode_byte_forward_with_upper_half_pattern_byte() {
        let data = [0b1111_0001, 0b0000_0010];
        let index = 0;
        let (byte, index) = BlockIdentifier::decode_byte_forward(&data, index);
        assert_eq!(byte, 0b0001_0010);
        assert_eq!(index, 2);
    }

    #[test]
    fn test_decode_byte_forward_with_lower_half_pattern_byte() {
        let data = [0b0000_0001, 0b1111_0010];
        let index = 0;
        let (byte, index) = BlockIdentifier::decode_byte_forward(&data, index);
        assert_eq!(byte, 0b0001_0010);
        assert_eq!(index, 2);
    }

    #[test]
    fn test_decode_block_identifier_forward() {
        let data = [0b0000_0001, 0b0000_0010, 0b0000_0100, 0b0000_1000];
        let index = 0;
        let identifier = BlockIdentifier::decode_forward(&data, index);
        assert_eq!(
            identifier,
            BlockIdentifier(0b0000_0001_0000_0010_0000_0100_0000_1000)
        );
    }

    #[test]
    fn test_decode_block_identifier_backward() {
        let data = [0b0000_0001, 0b0000_0010, 0b0000_0100, 0b0000_1000];
        let index = data.len();
        let identifier = BlockIdentifier::decode_backward(&data, index);
        assert_eq!(
            identifier,
            BlockIdentifier(0b0000_0001_0000_0010_0000_0100_0000_1000)
        );
    }
}
