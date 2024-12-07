advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(parse_line)
            .filter_map(|(goal, vals)| find_ops_rec(0, &vals, 0, goal, false))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(parse_line)
            .filter_map(|(goal, vals)| find_ops_rec(0, &vals, 0, goal, true))
            .sum(),
    )
}

fn parse_line(line: &str) -> (u64, Vec<u64>) {
    let (goal, vals) = line.split_once(": ").unwrap();
    let goal: u64 = goal.parse().unwrap();
    let vals = vals
        .split(' ')
        .map(|val| val.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    (goal, vals)
}

fn find_ops_rec(
    sum: u64,
    vals: &Vec<u64>,
    i: usize,
    goal: u64,
    concat_enabled: bool,
) -> Option<u64> {
    if i >= vals.len() {
        return (sum == goal).then(|| goal);
    } else if sum > goal {
        return None;
    }

    if let Some(res) = find_ops_rec(sum + vals[i], &vals, i + 1, goal, concat_enabled) {
        return Some(res);
    } else if let Some(res) = find_ops_rec(sum * vals[i], &vals, i + 1, goal, concat_enabled) {
        return Some(res);
    } else if concat_enabled {
        let concatted = format!("{}{}", sum, vals[i]).parse::<u64>().unwrap();
        return find_ops_rec(concatted, &vals, i + 1, goal, concat_enabled);
    }
    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
