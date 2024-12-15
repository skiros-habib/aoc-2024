use super::parse_input;

pub fn solve(input: &str) -> i32 {
    let lines = parse_input(input);
    let mut first: Vec<i32> = Vec::new();
    let mut second: Vec<i32> = Vec::new();
    for line in lines {
        first.push(line[0]);
        second.push(line[1]);
    }
    let mut similarity = 0;
    for each in first {
        for other in second.iter() {
            if each == *other {
                similarity += each;
            }
        }
    }
    similarity
}
