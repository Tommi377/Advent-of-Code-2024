use std::collections::{HashMap, HashSet};

use itertools::iproduct;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let w = get_width(input);
    let (map, start_pos) = parse_map(input, w);

    simulate_path(&start_pos, &map, None)
}

pub fn part_two(input: &str) -> Option<u32> {
    let h = input.lines().count() as i32;
    let w = get_width(input);
    let (map, start_pos) = parse_map(input, w);

    Some(
        iproduct!(0..w, 0..h)
            .filter(|barricade| simulate_path(&start_pos, &map, Some(barricade.clone())).is_none())
            .count()
            .try_into()
            .unwrap(),
    )
}

fn simulate_path(
    start: &(i32, i32),
    map: &HashMap<(i32, i32), char>,
    barricade: Option<(i32, i32)>,
) -> Option<u32> {
    let mut visited: HashSet<((i32, i32), Dir)> = HashSet::new();
    let mut pos = start.clone();
    let mut dir = Dir::Up;
    while let (new_pos, Some(c)) = get_cell(&pos, &map, &dir) {
        if visited.insert((pos, dir)) {
            if c == '#' || barricade.is_some_and(|p| p == new_pos) {
                dir = rotate(dir);
                continue;
            }
            pos = new_pos;
        } else {
            return None;
        }
    }
    visited.insert((pos, dir));

    Some(
        visited
            .into_iter()
            .map(|(pos, _)| pos)
            .collect::<HashSet<(i32, i32)>>()
            .len() as u32,
    )
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

fn parse_map(input: &str, w: i32) -> (HashMap<(i32, i32), char>, (i32, i32)) {
    let map = input
        .chars()
        .filter(|c| *c != '\n')
        .enumerate()
        .map(|(i, c)| ((i as i32 % w, i as i32 / w), c))
        .collect::<Vec<((i32, i32), char)>>();

    let start_pos = map.iter().position(|(_, c)| *c == '^').unwrap();
    let start_pos = (start_pos as i32 % w, start_pos as i32 / w);

    (map.into_iter().collect(), start_pos)
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

fn rotate(dir: Dir) -> Dir {
    match dir {
        Dir::Up => Dir::Right,
        Dir::Right => Dir::Down,
        Dir::Down => Dir::Left,
        Dir::Left => Dir::Up,
    }
}

fn get_cell(
    pos: &(i32, i32),
    map: &HashMap<(i32, i32), char>,
    dir: &Dir,
) -> ((i32, i32), Option<char>) {
    let new_pos = match dir {
        Dir::Up => (pos.0, pos.1 - 1),
        Dir::Right => (pos.0 + 1, pos.1),
        Dir::Down => (pos.0, pos.1 + 1),
        Dir::Left => (pos.0 - 1, pos.1),
    };

    (new_pos, map.get(&new_pos).map(|c| *c))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
