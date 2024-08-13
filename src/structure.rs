use rand::Rng;

use crate::blockid::BlockId;
use crate::blockpattern::BlockPattern;
use crate::fuzzy::FuzzyBitMap;

pub(crate) struct Strand<'a> {
    blocks: Vec<Block<'a>>,
}

pub(crate) struct Blocks {
    fuzzy_bit_map: FuzzyBitMap<BlockId>,
}

#[derive(Debug)]
struct Block<'a> {
    pattern: BlockPattern,
    slice: &'a [u8],
}

impl<'a> Strand<'a> {
    pub(crate) fn from_bytes(data: &'a [u8]) -> Strand<'a> {
        // split data by zero byte into slices
        // use the slice to construct the BlockIdentifier
        let slices = data.split(|&byte| byte == 0);
        let mut blocks = Vec::new();
        for slice in slices {
            let identifier = BlockPattern::decode_forward(slice, 0);
            let block = Block {
                pattern: identifier,
                slice,
            };
            blocks.push(block);
        }
        Strand { blocks }
    }
}

impl Blocks {
    pub(crate) fn new(max_distance: u32, match_chance: f64, strands: &[Strand]) -> Blocks {
        let mut fuzzy_bit_map = FuzzyBitMap::new(max_distance, match_chance);
        // walk through each strand block by block, and insert them at the same level in fuzzy map
        let mut block_index = 0;
        loop {
            let mut exhausted = true;
            for (strand_id, strand) in strands.iter().enumerate() {
                if block_index >= strand.blocks.len() {
                    continue;
                }
                exhausted = false;
                let block = &strand.blocks[block_index];
                fuzzy_bit_map.insert(block.pattern.get(), BlockId::new(strand_id, block_index));
            }
            if exhausted {
                break;
            }
            block_index += 1;
        }
        Self { fuzzy_bit_map }
    }

    pub(crate) fn lookup(
        &self,
        pattern: BlockPattern,
        block_index: usize,
        rng: &mut impl Rng,
    ) -> Option<&BlockId> {
        self.fuzzy_bit_map.get(pattern.get(), block_index, rng)
    }
}

#[cfg(test)]
mod tests {
    use rand::{rngs::SmallRng, SeedableRng};

    use super::*;

    #[test]
    fn test_new() {
        let strands = vec![Strand::from_bytes(&[1, 0, 1])];
        let blocks = Blocks::new(0, 0.5, &strands);
        let mut rng = SmallRng::from_seed([0; 32]);
        assert_eq!(blocks.lookup(BlockPattern::new(0b1111), 0, &mut rng), None);
    }
}
