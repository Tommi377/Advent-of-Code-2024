use std::collections::{HashMap, HashSet};

use advent_of_code::point::Point;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let w = get_width(input);
    let mut antinodes: HashSet<Point<i32>> = HashSet::new();

    parse_map(input, w).iter().for_each(|(_, positions)| {
        positions.iter().for_each(|pos| {
            positions
                .iter()
                .filter(move |p| pos != *p)
                .map(|p| pos.clone() + pos.clone() - p.clone())
                .filter(|pos| pos.in_bound(0, w))
                .for_each(|antinode_pos| {
                    antinodes.insert(antinode_pos);
                });
        })
    });

    Some(antinodes.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let w = get_width(input);
    let mut antinodes: HashSet<Point<i32>> = HashSet::new();

    parse_map(input, w).iter().for_each(|(_, positions)| {
        positions.iter().for_each(|pos| {
            positions
                .iter()
                .filter(move |p| pos != *p)
                .flat_map(|p| {
                    let mut new_antinodes: Vec<Point<i32>> = vec![];
                    let mut i = 0;
                    loop {
                        let new_point = pos.clone() + ((pos.clone() - p.clone()) * i);
                        if new_point.in_bound(0, w) {
                            new_antinodes.push(new_point);
                        } else {
                            break;
                        }
                        i += 1;
                    }
                    new_antinodes
                })
                .for_each(|antinode_pos| {
                    antinodes.insert(antinode_pos);
                });
        })
    });

    Some(antinodes.len() as u32)
}

fn get_width(input: &str) -> i32 {
    input
        .lines()
        .next()
        .unwrap()
        .chars()
        .count()
        .try_into()
        .unwrap()
}

fn parse_map(input: &str, w: i32) -> HashMap<char, Vec<Point<i32>>> {
    let mut satellites: HashMap<char, Vec<Point<i32>>> = HashMap::new();
    input
        .chars()
        .filter(|c| *c != '\n')
        .enumerate()
        .map(|(i, c)| (Point::new(i as i32 % w, i as i32 / w), c))
        .for_each(|(pos, c)| match satellites.entry(c) {
            std::collections::hash_map::Entry::Occupied(vec) => vec.into_mut().push(pos.clone()),
            std::collections::hash_map::Entry::Vacant(hm) => {
                if c != '.' {
                    hm.insert(vec![pos.clone()]);
                }
            }
        });

    satellites
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
