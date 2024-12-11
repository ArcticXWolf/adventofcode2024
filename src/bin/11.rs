use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(11);

fn count_numbers(number: u64, depth: usize, cache: &mut HashMap<(u64, usize), u64>) -> u64 {
    if depth == 0 {
        return 1;
    }

    if let Some(&res) = cache.get(&(number, depth)) {
        return res;
    }

    let mut result = 0;
    for new_number in apply_rules(number) {
        let new_count = count_numbers(new_number, depth - 1, cache);
        cache.insert((new_number, depth - 1), new_count);
        result += new_count;
    }
    result
}

fn apply_rules(number: u64) -> Vec<u64> {
    if number == 0 {
        return vec![1];
    }

    let digits = number.ilog10() + 1;
    if digits % 2 == 0 {
        let divider = 10_u64.pow(digits / 2);
        return vec![number / divider, number % divider];
    }

    vec![number * 2024]
}

pub fn part_one(input: &str) -> Option<u64> {
    let numbers = input
        .trim()
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect_vec();
    let mut cache = HashMap::new();
    let mut result = 0;
    for n in numbers {
        result += count_numbers(n, 25, &mut cache);
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let numbers = input
        .trim()
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect_vec();
    let mut cache = HashMap::new();
    let mut result = 0;
    for n in numbers {
        result += count_numbers(n, 75, &mut cache);
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
