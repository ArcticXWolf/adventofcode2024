use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use advent_of_code::algebra_helpers::{Point2, PointGrid, Rectangle};
use itertools::Itertools;

advent_of_code::solution!(8);

#[derive(Debug, Default)]
struct FrequencyMap {
    grid: PointGrid<isize, 2, char>,
    antennas_per_type: HashMap<char, Vec<Point2<isize>>>,
}

impl From<&str> for FrequencyMap {
    fn from(value: &str) -> Self {
        let mut result = Self::default();
        for (y, row) in value.trim().lines().enumerate() {
            for (x, c) in row.chars().enumerate() {
                let position = Point2::new(x as isize, y as isize);
                match c {
                    'A'..='Z' | 'a'..='z' | '0'..='9' => {
                        result.grid.insert(position, c);
                        result
                            .antennas_per_type
                            .entry(c)
                            .or_default()
                            .push(position);
                    }
                    '.' => (),
                    _ => unreachable!("unknown character in map"),
                }
            }
        }

        result
    }
}

impl Display for FrequencyMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Frequency Map:\n{}\nAntennas: {:?}",
            self.grid, self.antennas_per_type
        )
    }
}

impl FrequencyMap {
    fn find_antinodes(
        &self,
        bounds: &Rectangle<isize>,
        max_repetitions: usize,
    ) -> Vec<Point2<isize>> {
        let mut result: HashSet<Point2<isize>> = HashSet::new();
        for antenna_positions in self.antennas_per_type.values() {
            for (&antenna1, &antenna2) in antenna_positions.iter().tuple_combinations() {
                let distance_vector = antenna2 - antenna1;
                for i in 0..max_repetitions {
                    if bounds
                        .contains(&(antenna2 + distance_vector - (distance_vector * i as isize)))
                    {
                        result.insert(antenna2 + distance_vector - (distance_vector * i as isize));
                    }
                    if bounds
                        .contains(&(antenna1 - distance_vector + (distance_vector * i as isize)))
                    {
                        result.insert(antenna1 - distance_vector + (distance_vector * i as isize));
                    }
                }
            }
        }

        result.into_iter().collect::<Vec<_>>()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let max_x = input.lines().next().unwrap().chars().count();
    let max_y = input.lines().count();
    let bounds = Rectangle::new(Point2::zero(), Point2::new(max_x as isize, max_y as isize));
    let fm = FrequencyMap::from(input);

    Some(fm.find_antinodes(&bounds, 1).len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let max_x = input.lines().next().unwrap().chars().count();
    let max_y = input.lines().count();
    let bounds = Rectangle::new(Point2::zero(), Point2::new(max_x as isize, max_y as isize));
    let fm = FrequencyMap::from(input);

    Some(fm.find_antinodes(&bounds, max_x.max(max_y) + 3).len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
