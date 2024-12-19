use std::{collections::BTreeMap, fmt::Debug, iter};

mod part1;
mod part2;

const EXAMPLE: &str = "2333133121414131402";
const INPUT: &str = include_str!("input");

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MemoryBlock {
    Used(usize),
    Free,
}

impl Debug for MemoryBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MemoryBlock::Used(x) => write!(f, "{}", x),
            MemoryBlock::Free => write!(f, "."),
        }
    }
}

pub fn parse(input: &str) -> (Vec<MemoryBlock>, BTreeMap<usize, (usize, usize)>) {
    let mut blocks = Vec::new();
    let mut files: BTreeMap<usize, (usize, usize)> = BTreeMap::new();
    let mut length = 0;
    input.chars().enumerate().for_each(|(id, c)| {
        let size = c as usize - 48;
        let block = match id % 2 {
            0 => {
                files.insert(id / 2, (length, size));
                MemoryBlock::Used(id / 2)
            }
            1 => MemoryBlock::Free,
            _ => unreachable!(),
        };
        blocks.extend(iter::repeat_n(block, size));
        length += size;
    });
    (blocks, files)
}

pub fn memory_checksum(memory: &[MemoryBlock]) -> usize {
    memory.iter().enumerate().fold(0, |acc, (id, &c)| {
        acc + match c {
            MemoryBlock::Used(x) => x,
            MemoryBlock::Free => 0,
        } * id
    })
}

pub fn run_p1() {
    assert_eq!(part1::solve(EXAMPLE), 1928);
    let result = part1::solve(INPUT);
    println!("Day 9, Part 1: {}", result);
}

pub fn run_p2() {
    assert_eq!(part2::solve(EXAMPLE), 2858);
    let result = part2::solve(INPUT);
    println!("Day 9, Part 2: {}", result);
}
