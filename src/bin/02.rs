advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let mut values = line
                    .split_whitespace()
                    .map(|str| str.parse::<i32>().unwrap())
                    .peekable();

                let prev = values.next().unwrap();
                let diff = prev - *values.peek().unwrap();
                let dir = if diff < 0 { 1 } else { -1 };
                if diff.abs() == 0 || diff.abs() >= 4 {
                    return 0;
                }

                if values.is_sorted_by(|a, b| dir * (b - a) > 0 && dir * (b - a) < 4) {
                    1
                } else {
                    0
                }
            })
            .sum::<u32>(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let values = line
                    .split_whitespace()
                    .map(|str| str.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();

                if (0..values.len()).any(|i| {
                    let mut removed = values.to_vec();
                    removed.remove(i);
                    let mut removed = removed.iter().peekable();

                    let prev = removed.next().unwrap();
                    let diff = prev - *removed.peek().unwrap();
                    let dir = if diff < 0 { 1 } else { -1 };
                    if diff.abs() == 0 || diff.abs() >= 4 {
                        return false;
                    }

                    removed.is_sorted_by(|a, b| dir * (*b - *a) > 0 && dir * (*b - *a) < 4)
                }) {
                    1
                } else {
                    0
                }
            })
            .sum::<u32>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
