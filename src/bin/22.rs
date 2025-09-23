use std::collections::{HashMap, HashSet};

advent_of_code::solution!(22);

const PRUNE_MASK: u32 = (1 << 24) - 1;

fn calculate_next_number(secret: u32) -> u32 {
    let step1 = ((secret << 6) ^ secret) & PRUNE_MASK;
    let step2 = ((step1 >> 5) ^ step1) & PRUNE_MASK;
    let step3 = ((step2 << 11) ^ step2) & PRUNE_MASK;
    step3
}

fn calculate_number_at_time(seed: u32, timestep: usize) -> u32 {
    let mut result = seed;
    for _ in 0..timestep {
        result = calculate_next_number(result);
    }
    result
}

fn count_changes(seed: u32, timestep: usize, counter: &mut HashMap<(i8, i8, i8, i8), u32>) {
    let mut current = seed;
    let mut changes = (0, 0, 0, 0);
    let mut lock: HashSet<(i8, i8, i8, i8)> = HashSet::new();
    for i in 0..timestep {
        let last_suffix = (current % 10) as i8;
        current = calculate_next_number(current);
        let change = (current % 10) as i8 - last_suffix;
        changes = (changes.1, changes.2, changes.3, change);
        if i < 3 {
            continue;
        }
        if !lock.contains(&changes) {
            *counter.entry(changes).or_default() += current % 10;
            lock.insert(changes);
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut result = 0;
    for seed in input.trim().lines().map(|l| l.parse::<u32>().unwrap()) {
        result += calculate_number_at_time(seed, 2000) as u64;
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut counter = HashMap::new();
    for seed in input.trim().lines().map(|l| l.parse::<u32>().unwrap()) {
        count_changes(seed, 2000, &mut counter);
    }
    counter.values().max().cloned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(23));
    }

    #[test]
    fn test_known_numbers() {
        assert_eq!(calculate_next_number(123), 15887950);
    }

    #[test]
    fn test_multi_step() {
        assert_eq!(calculate_number_at_time(1, 2000), 8685429);
        assert_eq!(calculate_number_at_time(10, 2000), 4700978);
        assert_eq!(calculate_number_at_time(100, 2000), 15273692);
        assert_eq!(calculate_number_at_time(2024, 2000), 8667524);
    }

    #[test]
    fn test_count_known_number() {
        count_changes(123, 40, &mut HashMap::new());
    }
}
