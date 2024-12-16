pub fn solve(input: &str) -> i32 {
    let mut sum = 0;
    let mut mul = true;
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
        if mul {
            sum += a * b;
        }
        let dont = chunk.split("don't()").count() - 1;
        let dof = chunk.split("do()").count() - 1;
        if dont > dof {
            mul = false;
        } else if dont < dof {
            mul = true;
        }
    });
    sum
}
