use std::collections::{HashMap, HashSet};

use advent_of_code::{helpers::get_width, point::Point};

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u32> {
    let w = get_width(input);
    let (map, starts) = parse_map(input, w);

    starts
        .into_iter()
        .map(|start| dfs(start, &map, false))
        .sum::<i32>()
        .try_into()
        .ok()
}

pub fn part_two(input: &str) -> Option<u32> {
    let w = get_width(input);
    let (map, starts) = parse_map(input, w);

    starts
        .into_iter()
        .map(|start| dfs(start, &map, true))
        .sum::<i32>()
        .try_into()
        .ok()
}

fn dfs(pos: Point<i32>, map: &HashMap<Point<i32>, i32>, count_distinct: bool) -> i32 {
    let mut visited: HashSet<Point<i32>> = HashSet::new();
    let mut stack: Vec<(Point<i32>, i32)> = vec![(pos, 0)];
    let mut goal_count = 0;

    while let Some((pos, val)) = stack.pop() {
        vec![pos.right(), pos.left(), pos.up(), pos.down()]
            .iter()
            .filter(|p| map.get(p).is_some_and(|v| *v == val + 1))
            .for_each(|p| {
                if count_distinct || !visited.contains(p) {
                    visited.insert(p.clone());
                    if val < 8 {
                        stack.push((p.clone(), val + 1));
                    } else {
                        goal_count += 1;
                    }
                }
            });
    }

    goal_count
}

fn parse_map(input: &str, w: i32) -> (HashMap<Point<i32>, i32>, Vec<Point<i32>>) {
    let mut starts: Vec<Point<i32>> = vec![];
    (
        input
            .chars()
            .filter(|c| *c != '\n')
            .enumerate()
            .map(|(i, c)| {
                (
                    Point::new(i as i32 % w, i as i32 / w),
                    c.to_digit(10).unwrap() as i32,
                )
            })
            .inspect(|(pos, v)| {
                if *v == 0 {
                    starts.push(pos.clone());
                }
            })
            .collect(),
        starts,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
