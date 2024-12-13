use regex::Regex;

advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<i64> {
    Some(
        input
            .split("\n\n")
            .map(parse_lines)
            .map(|(a, b, prize)| calculate(a, b, prize, 0))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<i64> {
    Some(
        input
            .split("\n\n")
            .map(parse_lines)
            .map(|(a, b, prize)| calculate(a, b, prize, 10000000000000))
            .sum(),
    )
}

fn parse_lines(s: &str) -> ((i64, i64), (i64, i64), (i64, i64)) {
    let re_button = Regex::new(r"X\+(\d+), Y\+(\d+)$").unwrap();
    let re_prize = Regex::new(r"X=(\d+), Y=(\d+)$").unwrap();
    let mut split_iter = s.split('\n');
    let a: (i64, i64) = re_button
        .captures(split_iter.next().unwrap())
        .map(|caps| caps.extract().1.map(|cap| cap.parse::<i64>().unwrap()))
        .map(|slice: [i64; 2]| (slice[0], slice[1]))
        .unwrap();
    let b = re_button
        .captures(split_iter.next().unwrap())
        .map(|caps| caps.extract().1.map(|cap| cap.parse::<i64>().unwrap()))
        .map(|slice: [i64; 2]| (slice[0], slice[1]))
        .unwrap();
    let prize = re_prize
        .captures(split_iter.next().unwrap())
        .map(|caps| caps.extract().1.map(|cap| cap.parse::<i64>().unwrap()))
        .map(|slice: [i64; 2]| (slice[0], slice[1]))
        .unwrap();
    (a, b, prize)
}

fn calculate(a: (i64, i64), b: (i64, i64), prize: (i64, i64), offset: i64) -> i64 {
    let prize = (prize.0 + offset, prize.1 + offset);
    let det = a.0 * b.1 - a.1 * b.0;
    let x = (prize.0 * b.1 - prize.1 * b.0) / det;
    let y = (a.0 * prize.1 - a.1 * prize.0) / det;
    if (a.0 * x + b.0 * y, a.1 * x + b.1 * y) == (prize.0, prize.1) {
        x * 3 + y
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}
