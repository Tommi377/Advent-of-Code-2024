use strum::IntoEnumIterator;
use strum_macros::EnumIter;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let w: i32 = get_width(input);
    let mut starts: Vec<i32> = vec![];
    let map: Vec<char> = parse_map(input, 'X', &mut starts);

    Some(
        starts
            .iter()
            .map(|i| {
                Dir::iter()
                    .filter(|dir| {
                        let char_x = get_cell(*i, w, &map, 0, &dir);
                        let char_m = get_cell(*i, w, &map, 1, &dir);
                        let char_a = get_cell(*i, w, &map, 2, &dir);
                        let char_s = get_cell(*i, w, &map, 3, &dir);

                        char_x.is_some_and(|c| *c == 'X')
                            && char_m.is_some_and(|c| *c == 'M')
                            && char_a.is_some_and(|c| *c == 'A')
                            && char_s.is_some_and(|c| *c == 'S')
                    })
                    .count()
            })
            .map(|x| u32::try_from(x).unwrap())
            .sum::<u32>(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let w: i32 = get_width(input);
    let mut starts: Vec<i32> = vec![];
    let map: Vec<char> = parse_map(input, 'A', &mut starts);

    Some(
        starts
            .iter()
            .filter(|i| {
                let char_ul = get_cell(**i, w, &map, 1, &Dir::UpLeft);
                let char_dl = get_cell(**i, w, &map, 1, &Dir::DownLeft);
                let char_ur = get_cell(**i, w, &map, 1, &Dir::UpRight);
                let char_dr = get_cell(**i, w, &map, 1, &Dir::DownRight);

                (char_dr.is_some()
                    && char_ul.is_some_and(|c| {
                        *c == 'M' && *char_dr.unwrap() == 'S'
                            || *c == 'S' && *char_dr.unwrap() == 'M'
                    }))
                    && char_ur.is_some()
                    && char_dl.is_some_and(|c| {
                        *c == 'M' && *char_ur.unwrap() == 'S'
                            || *c == 'S' && *char_ur.unwrap() == 'M'
                    })
            })
            .count() as u32,
    )
}

#[derive(Debug, EnumIter)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
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

fn parse_map(input: &str, start_char: char, starts: &mut Vec<i32>) -> Vec<char> {
    input
        .chars()
        .filter(|c| *c != '\r' && *c != '\n')
        .enumerate()
        .inspect(|(i, c)| {
            if *c == start_char {
                starts.push((*i).try_into().unwrap());
            }
        })
        .map(|(_, c)| c)
        .collect()
}

fn get_cell<'a>(pos: i32, w: i32, map: &'a Vec<char>, d: i32, dir: &Dir) -> Option<&'a char> {
    match match dir {
        Dir::Up => get_coord(pos, w, 0, -d),
        Dir::Right => get_coord(pos, w, d, 0),
        Dir::Down => get_coord(pos, w, 0, d),
        Dir::Left => get_coord(pos, w, -d, 0),
        Dir::UpRight => get_coord(pos, w, d, -d),
        Dir::UpLeft => get_coord(pos, w, -d, -d),
        Dir::DownRight => get_coord(pos, w, d, d),
        Dir::DownLeft => get_coord(pos, w, -d, d),
    } {
        Some(new_pos) => map.get(usize::try_from(new_pos).unwrap()),
        None => None,
    }
}

fn get_coord(i: i32, w: i32, x: i32, y: i32) -> Option<i32> {
    if (i % w) + x < 0 || (i % w) + x >= w {
        return None;
    }

    let coord = ((i + x) % w) + ((i + x) / w + y) * w;
    if coord < 0 {
        None
    } else {
        Some(coord)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
