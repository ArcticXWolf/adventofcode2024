use std::fmt::Display;

use advent_of_code::algebra_helpers::{Point2, Point2Direction, PointGrid};

advent_of_code::solution!(6);

#[derive(Debug, Clone, Copy)]
enum MapTile {
    Wall,
}

impl Display for MapTile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MapTile::Wall => write!(f, "#"),
        }
    }
}

#[derive(Debug, Clone)]
struct Map {
    grid: PointGrid<isize, 2, MapTile>,
    guard: (Point2<isize>, Point2Direction),
    guard_path: Vec<(Point2<isize>, Point2Direction)>,
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let mut grid = PointGrid::default();
        let mut guard = (Point2::new(0, 0), Point2Direction::North);
        for (y, row) in value.trim().lines().enumerate() {
            for (x, c) in row.chars().enumerate() {
                let position = Point2::new(x as isize, y as isize);
                match c {
                    '.' => (),
                    '#' => grid.insert(position, MapTile::Wall),
                    '^' => guard = (position, Point2Direction::North),
                    '>' => guard = (position, Point2Direction::East),
                    'v' => guard = (position, Point2Direction::South),
                    '<' => guard = (position, Point2Direction::West),
                    _ => unimplemented!("Caught unknown symbol {}!", c),
                }
            }
        }
        Self {
            grid,
            guard,
            guard_path: vec![],
        }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (min, max) = self.grid.dimensions();
        writeln!(f, "Map ({}, {}):", min, max)?;
        for y in min.0[1]..(max.0[1] + 1) {
            for x in min.0[0]..(max.0[0] + 1) {
                let position = Point2::new(x, y);
                if let Some(u) = self.grid.get(&position) {
                    write!(f, "{}", u)?;
                } else if position == self.guard.0 {
                    match self.guard.1 {
                        Point2Direction::North => write!(f, "^")?,
                        Point2Direction::East => write!(f, ">")?,
                        Point2Direction::South => write!(f, "v")?,
                        Point2Direction::West => write!(f, "<")?,
                        _ => write!(f, "*")?,
                    };
                } else if self.guard_path.iter().any(|(p, _)| p == &position) {
                    write!(f, "X")?;
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f)?;
        }
        write!(f, "")
    }
}

impl Map {
    fn move_guard(&mut self) {
        let new_guard_position = self.guard.0.get_point_in_direction(&self.guard.1, 1);
        if self.grid.get(&new_guard_position).is_none() {
            self.guard_path.push(self.guard);
            self.guard = (new_guard_position, self.guard.1);
        } else {
            self.guard_path.push(self.guard);
            self.guard = (self.guard.0, self.guard.1.direction_right());
        }
    }

    // Returns true if path loops, false otherwise
    fn run_guard_until_loop_or_exit(&mut self) -> bool {
        let bounds = self.grid.dimensions_as_range();
        while bounds.contains(&self.guard.0) {
            self.move_guard();
            if self.guard_path.contains(&self.guard) {
                return true;
            }
        }
        false
    }

    fn guard_path_length(&self) -> usize {
        self.guard_path
            .iter()
            .map(|(p, _)| p)
            .collect::<::std::collections::HashSet<_>>()
            .len()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut map = Map::from(input);
    assert!(!map.run_guard_until_loop_or_exit());
    Some(map.guard_path_length() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let base_map = Map::from(input);
    let mut count_loop_paths = 0;

    let mut find_path_map = base_map.clone();
    find_path_map.run_guard_until_loop_or_exit();
    let possible_obstacle_positions = find_path_map
        .guard_path
        .iter()
        .map(|(p, _)| p)
        .collect::<::std::collections::HashSet<_>>();

    for possible_obstacle_position in possible_obstacle_positions {
        if possible_obstacle_position == &base_map.guard.0 {
            continue;
        }

        let mut modified_map = base_map.clone();
        modified_map
            .grid
            .insert(*possible_obstacle_position, MapTile::Wall);

        if modified_map.run_guard_until_loop_or_exit() {
            count_loop_paths += 1;
        }
    }

    Some(count_loop_paths)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
