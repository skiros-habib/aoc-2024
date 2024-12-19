fn observe(stones: &[usize]) -> Vec<usize> {
    stones
        .iter()
        .map(|&stone| {
            if let Some(digits) = stone.checked_ilog10() {
                match digits % 2 == 0 {
                    true => [stone * 2024].iter().map(|&x| x).collect::<Vec<_>>(),
                    false => {
                        let pow = 10_usize.pow((digits / 2) + 1);
                        [stone / pow, stone % pow].iter().map(|&x| x).collect()
                    }
                }
            } else {
                [0].iter().map(|&x| x).collect()
            }
        })
        .flatten()
        .collect()
}

pub fn solve(input: &str) -> usize {
    let mut input = input
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    for _ in 0..75 {
        input = observe(&input);
    }
    input.len()
}
