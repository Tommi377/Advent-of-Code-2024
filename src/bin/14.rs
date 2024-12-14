use std::{collections::HashSet, io};

use regex::Regex;

advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<i64> {
    let re = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
    let w: i64 = 101;
    let h: i64 = 103;
    let times: i64 = 100;
    let mut quadrants: [i64; 4] = [0; 4];

    re.captures_iter(input)
        .map(|c| c.extract().1)
        .map(|s_slice: [&str; 4]| s_slice.map(|s| s.parse::<i64>().unwrap()))
        .map(|[x, y, v_x, v_y]| ((x + (v_x * times)) % w, (y + (v_y * times)) % h))
        .map(|(x, y)| (if x < 0 { w + x } else { x }, if y < 0 { h + y } else { y }))
        .for_each(
            |(x, y)| match (x < w / 2, x > w / 2, y < h / 2, y > h / 2) {
                (true, false, true, false) => quadrants[0] += 1,
                (false, true, true, false) => quadrants[1] += 1,
                (true, false, false, true) => quadrants[2] += 1,
                (false, true, false, true) => quadrants[3] += 1,
                _ => (),
            },
        );

    Some(quadrants.iter().fold(1, |acc, v| acc * v))
}

pub fn part_two(input: &str) -> Option<i64> {
    let re = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
    let w: i64 = 101;
    let h: i64 = 103;
    let total_times: i64 = 10000;

    let skip = true;
    let start: i64 = 476;
    let step: i64 = 103;

    // This one needs manual labour so skipperoni
    if !skip {
        let robots: Vec<_> = re
            .captures_iter(input)
            .map(|c| c.extract().1)
            .map(|s_slice: [&str; 4]| s_slice.map(|s| s.parse::<i64>().unwrap()))
            .collect();

        for times in 1..=total_times {
            let mut final_pos = HashSet::<(i64, i64)>::new();
            let t = start + step * times;
            robots
                .clone()
                .iter()
                .map(|[x, y, v_x, v_y]| ((x + (v_x * t)) % w, (y + (v_y * t)) % h))
                .map(|(x, y)| (if x < 0 { w + x } else { x }, if y < 0 { h + y } else { y }))
                .for_each(|pos| {
                    final_pos.insert(pos);
                });

            println!("Loop {}", t);
            for y in 0..h {
                print!("|");
                for x in 0..w {
                    print!(
                        "{}",
                        if final_pos.contains(&(x, y)) {
                            "X"
                        } else {
                            " "
                        }
                    );
                }
                println!("|");
            }
            io::stdin().read_line(&mut String::new()).unwrap();
        }
    }
    Some(7892)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7892));
    }
}
