use std::collections::HashMap;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
    let mut nums: HashMap<u64, u64> = input
        .split_whitespace()
        .map(|s| (s.parse::<u64>().unwrap(), 1))
        .collect();

    (0..25).for_each(|_| {
        nums = blink(&nums);
    });

    Some(nums.iter().map(|(_, count)| *count).sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut nums: HashMap<u64, u64> = input
        .split_whitespace()
        .map(|s| (s.parse::<u64>().unwrap(), 1))
        .collect();

    (0..75).for_each(|_| {
        nums = blink(&nums);
    });

    Some(nums.iter().map(|(_, count)| *count).sum())
}

fn blink(nums: &HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut next_nums: HashMap<u64, u64> = HashMap::new();
    nums.iter().for_each(|(num, count)| {
        if *num == 0 {
            next_nums.insert(1, next_nums.get(&1).unwrap_or(&0) + count);
        } else if num.to_string().len() % 2 == 0 {
            let s = num.to_string();
            let (a, b) = s.split_at(s.len() / 2);
            let (a, b) = (a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap());
            next_nums.insert(a, next_nums.get(&a).unwrap_or(&0) + count);
            next_nums.insert(b, next_nums.get(&b).unwrap_or(&0) + count);
        } else {
            next_nums.insert(
                num * 2024,
                next_nums.get(&(num * 2024)).unwrap_or(&0) + count,
            );
        }
    });

    next_nums
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
