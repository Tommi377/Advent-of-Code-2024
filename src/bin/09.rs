advent_of_code::solution!(9);

// Absolutely horrible but O(n) since solved algorithmically
pub fn part_one(input: &str) -> Option<u64> {
    let nums: Vec<u64> = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .collect();

    let mut left = 0usize;
    let mut right = nums.len() - 1;
    let mut left_count = nums[0];
    let mut right_count = nums[right];

    let mut min = 0;
    let mut max = (input.len() as u64 - 1) / 2;

    let mut mult = 0u64;
    let mut sum = 0u64;

    let mut normal_mode = true;

    while left < right {
        while left_count > 0 {
            if normal_mode {
                sum += mult * min;
            } else {
                if right_count == 0 {
                    max -= 1;
                    right -= 2;
                    right_count = nums[right] - 1;
                } else {
                    right_count -= 1;
                }
                sum += mult * max;
            }
            left_count -= 1;
            mult += 1;
        }
        left += 1;
        left_count = nums[left];
        if normal_mode {
            min += 1;
        }
        normal_mode = !normal_mode;
    }

    while right_count > 0 {
        right_count -= 1;
        sum += mult * max;
        mult += 1;
    }

    Some(sum)
}

// Ok this one's algorithm is kinda elegant but unreadable
pub fn part_two(input: &str) -> Option<u64> {
    let nums: Vec<(u64, u64)> = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .scan(0u64, |state, x| {
            *state = *state + x;
            Some((*state - x, x))
        })
        .collect();
    let occupied: Vec<(usize, (u64, u64))> =
        nums.clone().into_iter().step_by(2).enumerate().collect();
    let mut empty: Vec<(usize, (u64, u64))> = nums
        .clone()
        .into_iter()
        .skip(1)
        .step_by(2)
        .enumerate()
        .collect();

    Some(
        occupied
            .iter()
            .rev()
            .map(|(i, (offset, n))| {
                match empty
                    .iter_mut()
                    .find(|(j, (_, spaces))| j < i && spaces >= n)
                {
                    Some((_, (start, spaces))) => {
                        *start += n;
                        *spaces -= n;
                        ((0..*n).map(|k| (*start + k - n) * (*i as u64))).sum::<u64>()
                    }
                    None => ((0..*n).map(|k| (*offset + k) * (*i as u64))).sum::<u64>(),
                }
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
