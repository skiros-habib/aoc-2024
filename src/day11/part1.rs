fn observe(stones: &[usize]) -> Vec<usize> {
    let mut result = Vec::new();
    for &stone in stones {
        if let Some(digits) = stone.checked_ilog10() {
            if digits % 2 == 0 {
                result.push(stone * 2024);
            } else {
                let pow = 10_usize.pow((digits / 2) + 1);
                result.push(stone / pow);
                result.push(stone % pow);
            }
        } else {
            result.push(1);
        }
    }
    result
}

pub fn solve(input: &str) -> usize {
    let mut input = input
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    for _ in 0..25 {
        input = observe(&input);
    }
    input.len()
}
