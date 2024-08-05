use crate::blockid::BlockIdentifier;

struct Strand<'a> {
    blocks: Vec<Block<'a>>,
}

#[derive(Debug)]
struct Block<'a> {
    identifier: BlockIdentifier,
    slice: &'a [u8],
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
