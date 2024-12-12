pub fn get_width(input: &str) -> i32 {
    input
        .lines()
        .next()
        .unwrap()
        .chars()
        .count()
        .try_into()
        .unwrap()
}
