use std::{collections::HashMap, ops::Index};

use itertools::Itertools;

advent_of_code::solution!(19);

fn check_design_possibilities<'a>(
    design: &'a str,
    patterns: &[&str],
    cache: &mut HashMap<&'a str, usize>,
) -> usize {
    if design.is_empty() {
        cache.insert(design, 1);
        return 1;
    }

    if let Some(amount) = cache.get(design) {
        return *amount;
    }

    let mut sum = 0;
    for p in patterns {
        if design.starts_with(p) {
            sum += check_design_possibilities(design.index(p.len()..), patterns, cache);
        }
    }
    cache.insert(design, sum);
    sum
}

pub fn part_one(input: &str) -> Option<u32> {
    let (pattern_str, design_str) = input.trim().split_once("\n\n").unwrap();
    let mut cache = HashMap::new();
    let patterns = pattern_str.trim().split(", ").collect::<Vec<_>>();
    let designs = design_str.trim().lines().collect::<Vec<_>>();
    let design_possibilities = designs
        .into_iter()
        .map(|d| check_design_possibilities(d, &patterns, &mut cache))
        .collect_vec();
    Some(design_possibilities.iter().filter(|dp| **dp > 0).count() as u32)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (pattern_str, design_str) = input.trim().split_once("\n\n").unwrap();
    let mut cache = HashMap::new();
    let patterns = pattern_str.trim().split(", ").collect::<Vec<_>>();
    let designs = design_str.trim().lines().collect::<Vec<_>>();
    let design_possibilities = designs
        .into_iter()
        .map(|d| check_design_possibilities(d, &patterns, &mut cache))
        .collect_vec();
    Some(design_possibilities.iter().sum::<usize>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
