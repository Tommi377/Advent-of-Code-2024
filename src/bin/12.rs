use std::collections::{HashMap, HashSet};

use advent_of_code::{helpers::get_width, point::Point};

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u64> {
    let map = parse_map(input);
    let mut visited = HashSet::<Point<i32>>::new();

    Some(
        map.iter()
            .map(|(pos, c)| dfs(*c, pos, &map, &mut visited))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let map = parse_map(input);
    let mut visited = HashSet::<Point<i32>>::new();

    Some(
        map.iter()
            .map(|(pos, c)| dfs2(*c, pos, &map, &mut visited))
            .sum(),
    )
}

fn dfs(
    symbol: char,
    pos: &Point<i32>,
    map: &HashMap<Point<i32>, char>,
    visited: &mut HashSet<Point<i32>>,
) -> u64 {
    let mut stack: Vec<Point<i32>> = vec![pos.clone()];
    let mut area: u64 = 0;
    let mut perim: u64 = 0;

    while let Some(pos) = stack.pop() {
        if !visited.contains(&pos) {
            visited.insert(pos.clone());
            pos.neighbors()
                .iter()
                .filter(|p| map.get(p).is_some_and(|v| *v == symbol))
                .for_each(|p| {
                    stack.push(p.clone());
                });

            area += 1;
            perim += pos
                .neighbors()
                .iter()
                .filter(|p| map.get(p).is_none_or(|v| *v != symbol))
                .count() as u64
        }
    }

    area * perim
}

fn dfs2(
    symbol: char,
    pos: &Point<i32>,
    map: &HashMap<Point<i32>, char>,
    visited: &mut HashSet<Point<i32>>,
) -> u64 {
    if visited.contains(&pos) {
        return 0;
    }

    let mut stack: Vec<Point<i32>> = vec![pos.clone()];
    let mut visited_now: HashSet<Point<i32>> = HashSet::new();
    let mut visited_corners: HashSet<Point<i32>> = HashSet::new();
    let mut area: u64 = 0;
    let mut sides: u64 = 0;

    while let Some(pos) = stack.pop() {
        if !visited.contains(&pos) {
            visited.insert(pos.clone());
            visited_now.insert(pos.clone());
            pos.neighbors()
                .iter()
                .filter(|p| map.get(p).is_some_and(|v| *v == symbol))
                .for_each(|p| {
                    stack.push(p.clone());
                });

            area += 1;
        }
    }

    visited_now.iter().for_each(|pos| {
        vec![pos.clone(), pos.right(), pos.down(), pos.down_right()]
            .iter()
            .for_each(|corner_pos| {
                if !visited_corners.contains(corner_pos) {
                    visited_corners.insert(corner_pos.clone());

                    sides += corner_count(
                        map.contains_key(&corner_pos.up_left())
                            && visited_now.contains(&corner_pos.up_left()),
                        map.contains_key(&corner_pos.up())
                            && visited_now.contains(&corner_pos.up()),
                        map.contains_key(&corner_pos.left())
                            && visited_now.contains(&corner_pos.left()),
                        map.contains_key(&corner_pos.clone())
                            && visited_now.contains(&corner_pos.clone()),
                    );
                }
            });
    });

    area * sides
}

fn corner_count(tl: bool, tr: bool, bl: bool, br: bool) -> u64 {
    match (tl, tr, bl, br) {
        (true, true, true, true)
        | (true, true, false, false)
        | (true, false, true, false)
        | (false, true, false, true)
        | (false, false, true, true)
        | (false, false, false, false) => 0,
        (true, true, true, false)
        | (true, true, false, true)
        | (true, false, true, true)
        | (true, false, false, false)
        | (false, true, true, true)
        | (false, true, false, false)
        | (false, false, true, false)
        | (false, false, false, true) => 1,
        (true, false, false, true) | (false, true, true, false) => 2,
    }
}

fn parse_map(input: &str) -> HashMap<Point<i32>, char> {
    let w = get_width(input);
    input
        .chars()
        .filter(|c| *c != '\n')
        .enumerate()
        .map(|(i, c)| (Point::new(i as i32 % w, i as i32 / w), c))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
