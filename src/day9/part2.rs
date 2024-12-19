use crate::day9::MemoryBlock;
use itertools::Itertools;

use super::{memory_checksum, parse};

fn _pm(memory: &[MemoryBlock]) {
    for i in memory {
        print!("{:?}", i);
    }
    println!();
}

pub fn solve(input: &str) -> usize {
    let (mut memory, files) = parse(input);
    for (&file, &(start, size)) in files.iter().rev() {
        let free_blocks: Vec<usize> = memory[..start]
            .iter()
            .positions(|block| *block == MemoryBlock::Free)
            .collect();
        let allowed = free_blocks
            .windows(size)
            .find(|window| window.windows(2).all(|w| w[1] - w[0] == 1));
        if let Some(allowed) = allowed {
            for i in allowed {
                memory[*i] = MemoryBlock::Used(file);
            }
            for i in start..start + size {
                memory[i] = MemoryBlock::Free;
            }
        }
    }
    memory_checksum(&memory)
}
