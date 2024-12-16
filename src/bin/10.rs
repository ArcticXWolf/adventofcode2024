use std::fmt::Display;

use advent_of_code::algebra_helpers::{Point2, Point2Direction, PointGrid};
use itertools::Itertools;

advent_of_code::solution!(10);

#[derive(Debug, Default)]
struct TopoMap(PointGrid<isize, 2, u8>);

impl From<&str> for TopoMap {
    fn from(value: &str) -> Self {
        let mut result = TopoMap::default();
        for (y, row) in value.trim().lines().enumerate() {
            for (x, c) in row.chars().enumerate() {
                let position = Point2::new(x as isize, y as isize);
                match c {
                    '0'..='9' => result.0.insert(position, c.to_digit(10).unwrap() as u8),
                    _ => unreachable!("Map should only contain digits"),
                }
            }
        }
        result
    }
}

impl Display for TopoMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TopoMap {
    fn find_trailends_from_position(&self, start_pos: &Point2<isize>) -> Vec<Point2<isize>> {
        let mut queue = vec![];
        let mut results = vec![];

        if let Some(start_level) = self.0.get(start_pos) {
            queue.push((*start_pos, *start_level));
        }

        while let Some((current_pos, current_level)) = queue.pop() {
            if current_level >= 9 {
                results.push(current_pos);
                continue;
            }

            for pd in Point2Direction::all() {
                let new_pos = current_pos.get_point_in_direction(pd, 1);
                if let Some(new_level) = self.0.get(&new_pos) {
                    if new_level.saturating_sub(current_level) == 1 {
                        queue.push((new_pos, *new_level));
                    }
                }
            }
        }

        results
    }

    fn get_trailheads(&self) -> Vec<Point2<isize>> {
        self.0
            .iter_full_bounds()
            .filter(|p| self.0.get(p) == Some(&0))
            .collect_vec()
    }

    fn score(&self) -> u32 {
        self.get_trailheads()
            .iter()
            .flat_map(|p| {
                self.find_trailends_from_position(p)
                    .into_iter()
                    .collect::<::std::collections::HashSet<_>>()
            })
            .count() as u32
    }

    fn rating(&self) -> u32 {
        self.get_trailheads()
            .iter()
            .flat_map(|p| self.find_trailends_from_position(p))
            .count() as u32
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let topomap = TopoMap::from(input);
    Some(topomap.score())
}

pub fn part_two(input: &str) -> Option<u32> {
    let topomap = TopoMap::from(input);
    Some(topomap.rating())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
