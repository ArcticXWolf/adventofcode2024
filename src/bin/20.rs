use std::cmp::Ordering;

use advent_of_code::algebra_helpers::{Point2, Point2Direction, PointGrid};

advent_of_code::solution!(20);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum Tile {
    #[default]
    Wall,
    Path,
}

#[derive(Debug)]
struct Map {
    grid: PointGrid<isize, 2, Tile>,
    path: Vec<Point2<isize>>,
    start_pos: Point2<isize>,
    end_pos: Point2<isize>,
}

#[derive(Debug, PartialEq, Eq)]
struct Cheat {
    start: Point2<isize>,
    end: Point2<isize>,
    saved_cost: isize,
}

impl Ord for Cheat {
    fn cmp(&self, other: &Self) -> Ordering {
        self.saved_cost
            .cmp(&other.saved_cost)
            .then_with(|| self.start.0[0].cmp(&other.start.0[0]))
            .then_with(|| self.start.0[1].cmp(&other.start.0[1]))
            .then_with(|| self.end.0[0].cmp(&other.end.0[0]))
            .then_with(|| self.end.0[1].cmp(&other.end.0[1]))
    }
}

impl PartialOrd for Cheat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let mut res = Self {
            grid: Default::default(),
            path: Default::default(),
            start_pos: Point2::zero(),
            end_pos: Point2::zero(),
        };

        for (y, row) in value.trim().lines().enumerate() {
            for (x, c) in row.chars().enumerate() {
                let position = Point2::new(x as isize, y as isize);
                match c {
                    '#' => res.grid.insert(position, Tile::Wall),
                    'S' => {
                        res.grid.insert(position, Tile::Path);
                        res.start_pos = position;
                    }
                    'E' => {
                        res.grid.insert(position, Tile::Path);
                        res.end_pos = position;
                    }
                    '.' => res.grid.insert(position, Tile::Path),
                    _ => unimplemented!("unknown symbol in map"),
                }
            }
        }

        res.populate_path();
        res
    }
}

impl Map {
    fn populate_path(&mut self) {
        let mut current_pos = self.start_pos;
        let mut previous_pos = current_pos;
        let mut path = vec![];

        while current_pos != self.end_pos {
            for pd in Point2Direction::all() {
                let next = current_pos.get_point_in_direction(pd, 1);
                if next != previous_pos && self.grid.get(&next).is_some_and(|&t| t == Tile::Path) {
                    path.push(current_pos);
                    previous_pos = current_pos;
                    current_pos = next;
                    break;
                }
            }
        }
        path.push(current_pos);

        self.path = path;
    }

    fn find_cheats(&self, cheat_length: usize) -> Vec<Cheat> {
        let mut res = vec![];

        for (from_idx, from_pos) in self.path.iter().enumerate() {
            for (_, to_pos) in self.path.iter().enumerate().skip(from_idx + 1) {
                let cheat_distance = from_pos.distance_manhattan_from(*to_pos) as usize;
                if cheat_distance <= cheat_length {
                    let cheat = self.create_cheat(from_pos, to_pos, cheat_distance);
                    if cheat.saved_cost > 0 {
                        res.push(cheat);
                    }
                }
            }
        }

        res
    }

    fn create_cheat(
        &self,
        start_pos: &Point2<isize>,
        end_pos: &Point2<isize>,
        cost: usize,
    ) -> Cheat {
        let start_index = self
            .path
            .iter()
            .position(|p| p == start_pos)
            .expect("cheat called for nonexistent path");
        let end_index = self
            .path
            .iter()
            .position(|p| p == end_pos)
            .expect("cheat called for nonexistent path");

        Cheat {
            start: *start_pos,
            end: *end_pos,
            saved_cost: end_index as isize - start_index as isize - cost as isize,
        }
    }
}

fn solve(input: &str, cheat_length: usize, cost_saved: usize) -> Option<u32> {
    let map = Map::from(input);
    let mut cheats = map.find_cheats(cheat_length);
    cheats.sort();
    cheats.dedup();
    Some(
        cheats
            .iter()
            .filter(|c| c.saved_cost >= cost_saved as isize)
            .count() as u32,
    )
}

pub fn part_one(input: &str) -> Option<u32> {
    solve(input, 2, 100)
}

pub fn part_two(input: &str) -> Option<u32> {
    solve(input, 20, 100)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = solve(&advent_of_code::template::read_file("examples", DAY), 2, 50);
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_part_two() {
        let result = solve(
            &advent_of_code::template::read_file("examples", DAY),
            20,
            50,
        );
        assert_eq!(result, Some(285));
    }
}
