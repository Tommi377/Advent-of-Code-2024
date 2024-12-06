use std::collections::{HashMap, HashSet};

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let mut input_iter = input.split("\n\n");

    let (rules, _) = parse_rules(&input_iter.next().unwrap());

    Some(
        input_iter
            .next()
            .unwrap()
            .lines()
            .map(|line| {
                line.split(',')
                    .map(|s| s.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>()
            })
            .map(|vals: Vec<u32>| (vals.clone(), vals[vals.len() / 2]))
            .filter(|(vals, _)| check_correctness(vals, &rules).is_none())
            .map(|(_, mid)| mid)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut input_iter = input.split("\n\n");

    let (rules, rules_rev) = parse_rules(&input_iter.next().unwrap());

    let incorrect = input_iter
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            line.split(',')
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .filter(|vals| check_correctness(vals, &rules).is_some())
        .collect::<Vec<Vec<u32>>>();

    Some(
        incorrect
            .iter()
            .map(|vals| {
                let mut prev_vals = vals.clone();
                while let Some(i) = check_correctness(&prev_vals, &rules) {
                    let val = vals[i];
                    let mut temp = prev_vals.clone();
                    temp.remove(i);
                    let found = temp
                        .iter()
                        .position(|v| rules_rev.get(&val).unwrap().contains(v))
                        .unwrap();
                    temp.insert(found, val);
                    prev_vals = temp;
                }
                prev_vals[prev_vals.len() / 2]
            })
            .sum::<u32>(),
    )
}

fn parse_rules(input: &str) -> (HashMap<u32, HashSet<u32>>, HashMap<u32, HashSet<u32>>) {
    let mut rules: HashMap<u32, HashSet<u32>> = HashMap::new();
    let mut rules_rev: HashMap<u32, HashSet<u32>> = HashMap::new();
    input
        .split("\n")
        .map(|rule| {
            rule.split('|')
                .map(|r| r.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .map(|v| (v[0], v[1]))
        .for_each(|(a, b)| {
            if !rules_rev.contains_key(&a) {
                rules_rev.insert(a, HashSet::new());
            }
            if !rules.contains_key(&b) {
                rules.insert(b, HashSet::new());
            }
            rules_rev.get_mut(&a).unwrap().insert(b);
            rules.get_mut(&b).unwrap().insert(a);
        });
    (rules, rules_rev)
}

fn check_correctness(vals: &Vec<u32>, rules: &HashMap<u32, HashSet<u32>>) -> Option<usize> {
    let mut bad: HashSet<u32> = HashSet::new();
    let mut res: Option<usize> = None;
    vals.iter().enumerate().all(|(i, v)| {
        if rules.contains_key(v) {
            bad.extend(rules.get(v).unwrap());
        }
        if bad.contains(v) {
            res = Some(i);
        }
        !bad.contains(v)
    });
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
