advent_of_code::solution!(1);

use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    let mut left = vec![];
    let mut right = vec![];
    for line in input.lines() {
        let split: Vec<u32> = line
            .split_whitespace()
            .map(|str| str.parse::<u32>().unwrap())
            .collect();
        left.push(split[0]);
        right.push(split[1]);
    }
    left.sort();
    right.sort();

    Some(
        left.iter()
            .zip(right.iter())
            .fold(0, |acc, (a, b)| acc + a.abs_diff(*b)),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut values = vec![];
    let mut map: HashMap<u32, u32> = HashMap::new();
    for line in input.lines() {
        let split: Vec<u32> = line
            .split_whitespace()
            .map(|str| str.parse::<u32>().unwrap())
            .collect();
        values.push(split[0]);
        map.insert(split[1], map.get(&split[1]).unwrap_or(&0) + 1);
    }
    Some(
        values
            .iter()
            .fold(0, |acc, x| acc + x * map.get(x).unwrap_or(&0)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
