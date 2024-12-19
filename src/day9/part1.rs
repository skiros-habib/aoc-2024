use super::{memory_checksum, parse, MemoryBlock};

pub fn solve(input: &str) -> usize {
    let (mut data, _) = parse(input);
    for i in 0..data.len() {
        if data[i] == MemoryBlock::Free {
            let next_block = data
                .iter()
                .rposition(|&x| x != MemoryBlock::Free)
                .expect("There should be a next block");
            if next_block < i {
                break;
            }
            data.swap(i, next_block);
        }
    }
    memory_checksum(&data)
}
