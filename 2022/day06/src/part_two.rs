use std::collections::HashSet;

fn unique(block: &[u8], size: usize) -> bool {
    block.iter().collect::<HashSet<_>>().len() == size
}

pub fn process(input: &[u8]) -> usize {
    let block_size: usize = 14;
    for (index, block) in input.windows(block_size).enumerate() {
        if unique(block, block_size) {
            return index + block_size;
        }
    }
    unreachable!()
}
