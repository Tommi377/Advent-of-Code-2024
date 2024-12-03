use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    Some(
        input
            .lines()
            .map(|line| {
                let mut sum = 0;
                for (_, [a, b]) in re.captures_iter(line).map(|c| c.extract()) {
                    sum += a.parse::<u32>().unwrap() * b.parse::<u32>().unwrap();
                }
                sum
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let do_re = Regex::new(r"(do\(\)|^)(.|\n)*?(don't\(\)|$)").unwrap();
    let mul_re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    Some(
        do_re
            .find_iter(input)
            .map(|segment| {
                let mut sum = 0;
                for (_, [a, b]) in mul_re.captures_iter(segment.as_str()).map(|c| c.extract()) {
                    sum += a.parse::<u32>().unwrap() * b.parse::<u32>().unwrap();
                }
                sum
            })
            .sum::<u32>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result: Option<u32> = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
