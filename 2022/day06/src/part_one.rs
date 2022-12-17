fn unique(block: &[u8]) -> bool {
    block[0] != block[1]
        && block[0] != block[2]
        && block[0] != block[3]
        && block[1] != block[2]
        && block[1] != block[3]
        && block[2] != block[3]
}

pub fn process(input: &[u8]) -> usize {
    let block_size = 4;
    for (index, block) in input.windows(block_size).enumerate() {
        if unique(block) {
            return index + block_size;
        }
    }
    unreachable!()
}
