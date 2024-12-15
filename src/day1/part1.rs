use super::parse_input;

pub fn solve(input: &str) -> i32 {
    let lines = parse_input(input);
    let mut first: Vec<i32> = Vec::new();
    let mut second: Vec<i32> = Vec::new();
    for line in lines {
        first.push(line[0]);
        second.push(line[1]);
    }
    first.sort();
    second.sort();
    let mut distance = 0;
    for i in 0..first.len() {
        distance += (first[i] - second[i]).abs();
    }
    distance
}
