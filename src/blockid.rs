#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct BlockId {
    strand_id: usize,
    block_index: usize,
}

impl BlockId {
    pub(crate) fn new(strand_id: usize, block_index: usize) -> BlockId {
        BlockId {
            strand_id,
            block_index,
        }
    }
}
