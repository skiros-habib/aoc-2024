use std::collections::HashMap;

mod part1;
mod part2;

const EXAMPLE: &str = "125 17";
const INPUT: &str = include_str!("input");

// O(horrible) solution:
// fn observe(stones: &[usize]) -> Vec<usize> {
//     let mut result = Vec::new();
//     for &stone in stones {
//         if let Some(digits) = stone.checked_ilog10() {
//             if digits % 2 == 0 {
//                 result.push(stone * 2024);
//             } else {
//                 let pow = 10_usize.pow((digits / 2) + 1);
//                 result.push(stone / pow);
//                 result.push(stone % pow);
//             }
//         } else {
//             result.push(1);
//         }
//     }
//     result
// }

fn observe(stones: HashMap<usize, usize>) -> HashMap<usize, usize> {
    let mut new_stones = HashMap::new();
    stones.into_iter().for_each(|(stone, count)| {
        if let Some(digits) = stone.checked_ilog10() {
            if digits % 2 == 0 {
                new_stones
                    .entry(stone * 2024)
                    .and_modify(|e| *e += count)
                    .or_insert(count);
            } else {
                let pow = 10_usize.pow((digits / 2) + 1);
                new_stones
                    .entry(stone / pow)
                    .and_modify(|e| *e += count)
                    .or_insert(count);
                new_stones
                    .entry(stone % pow)
                    .and_modify(|e| *e += count)
                    .or_insert(count);
            }
        } else {
            new_stones
                .entry(1)
                .and_modify(|e| *e += count)
                .or_insert(count);
        }
    });
    new_stones
}

pub fn run_p1() {
    assert_eq!(part1::solve(EXAMPLE), 55312);
    let result = part1::solve(INPUT);
    println!("Day 11, Part 1: {}", result);
}

pub fn run_p2() {
    // assert_eq!(part2::solve(EXAMPLE), 81);
    let result = part2::solve(INPUT);
    println!("Day 11, Part 2: {}", result);
}
