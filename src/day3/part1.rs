pub fn solve(input: &str) -> i32 {
    let mut sum = 0;
    input.split("mul(").for_each(|chunk| {
        let mut nums = chunk.split(",");
        let a = nums
            .next()
            .unwrap_or_default()
            .parse::<i32>()
            .unwrap_or_default();
        let b = nums
            .next()
            .unwrap_or_default()
            .split(")")
            .next()
            .unwrap_or_default()
            .parse::<i32>()
            .unwrap_or_default();
        sum += a * b
    });
    sum
}
