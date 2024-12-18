use super::parse;

use rayon::prelude::*;

fn calc(numbers: &[usize], operations: &Vec<usize>) -> usize {
    numbers.iter().enumerate().fold(0, |acc, (index, &value)| {
        if index == 0 {
            return value;
        }
        let operation = operations[index - 1];
        match operation {
            0 => acc + value,
            1 => acc * value,
            2 => format!("{}{}", acc, value)
                .parse()
                .expect("These should both be numbers anyway"),
            _ => panic!("Unknown operation"),
        }
    })
}

pub fn solve(input: &str) -> usize {
    let lines = parse(input);
    let mut total = 0;
    for (test_value, numbers) in &lines {
        let max = numbers.len() - 1;
        let total_combinations = 3usize.pow(max as u32);
        let comb_result = (0..total_combinations).into_par_iter().map(|num| {
            let mut current: usize = num;
            let mut combination: Vec<usize> = vec![];
            for _ in 0..max {
                combination.push(current % 3);
                current /= 3;
            }
            combination
        });
        let combinations = comb_result.collect::<Vec<Vec<usize>>>();
        let result = combinations
            .into_par_iter()
            .find_first(|combination| calc(numbers, combination) == *test_value);
        if result.is_some() {
            total += test_value;
        }
    }
    total
}
