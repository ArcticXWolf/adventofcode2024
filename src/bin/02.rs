use itertools::Itertools;

advent_of_code::solution!(2);

struct Report(Vec<u8>);

impl From<&str> for Report {
    fn from(value: &str) -> Self {
        Self {
            0: value
                .split_whitespace()
                .filter_map(|s| s.parse::<u8>().ok())
                .collect_vec(),
        }
    }
}

impl Report {
    fn is_increasing(&self) -> bool {
        self.0.iter().skip(1).zip(&self.0).all(|(l, r)| *r < *l)
    }

    fn is_decreasing(&self) -> bool {
        self.0.iter().skip(1).zip(&self.0).all(|(l, r)| *r > *l)
    }

    fn is_distance_valid(&self) -> bool {
        self.0.iter().skip(1).zip(&self.0).all(|(l, r)| {
            let distance = (*r as i32 - *l as i32).abs();
            distance >= 1 && distance <= 3
        })
    }

    fn is_safe(&self) -> bool {
        (self.is_increasing() || self.is_decreasing()) && self.is_distance_valid()
    }

    fn is_safe_with_problem_dampener(&self) -> bool {
        if self.is_safe() {
            return true;
        }

        for n in 0..self.0.len() {
            let numbers = self
                .0
                .iter()
                .enumerate()
                .filter(|&(i, _)| i != n)
                .map(|(_, v)| v)
                .cloned()
                .collect_vec();
            if (Report { 0: numbers }).is_safe() {
                return true;
            }
        }

        false
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .trim()
            .lines()
            .map(|l| Report::from(l))
            .filter(|r| r.is_safe())
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .trim()
            .lines()
            .map(|l| Report::from(l))
            .filter(|r| r.is_safe_with_problem_dampener())
            .count() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
