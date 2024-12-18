use super::parse;

fn calc(numbers: &[usize], operations: usize) -> usize {
    numbers.iter().enumerate().fold(0, |acc, (index, &value)| {
        if index == 0 {
            return value;
        }
        let operation = (operations & (1 << index - 1)) >> index - 1;
        // print!("{:?}", operation);
        match operation {
            0 => acc + value,
            1 => acc * value,
            _ => panic!("Unknown operation"),
        }
    })
}

pub fn solve(input: &str) -> usize {
    let lines = parse(input);
    let mut total = 0;

    for (test_value, numbers) in &lines {
        for operation in 0..usize::pow(2, (numbers.len() - 1) as u32) {
            let result = calc(numbers, operation);
            if result == *test_value {
                total += *test_value;
                break;
            }
        }
    }
    total
}
