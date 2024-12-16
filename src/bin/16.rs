use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
    fmt::Display,
};

use advent_of_code::algebra_helpers::{Point2, Point2Direction, PointGrid};

advent_of_code::solution!(16);

#[derive(Debug, Default)]
enum Tile {
    #[default]
    Wall,
    Start,
    End,
}

impl TryFrom<char> for Tile {
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Tile::Wall),
            'S' => Ok(Tile::Start),
            'E' => Ok(Tile::End),
            _ => Err("unknown symbol"),
        }
    }

    type Error = &'static str;
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Wall => write!(f, "#"),
            Self::Start => write!(f, "S"),
            Self::End => write!(f, "E"),
        }
    }
}

#[derive(Debug, Default)]
struct Maze(PointGrid<isize, 2, Tile>);

impl From<&str> for Maze {
    fn from(value: &str) -> Self {
        let mut maze = Self::default();
        for (y, row) in value.trim().lines().enumerate() {
            for (x, c) in row.chars().enumerate() {
                let position = Point2::new(x as isize, y as isize);
                if let Ok(tile) = Tile::try_from(c) {
                    maze.0.insert(position, tile);
                }
            }
        }
        maze
    }
}

impl Display for Maze {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.0)
    }
}

#[derive(Clone, Eq, PartialEq)]
struct State {
    cost: u32,
    position: Point2<isize>,
    direction: Point2Direction,
    visited_points: HashSet<Point2<isize>>,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // we flip the Ord here, so the max-heap becomes a min-heap
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| other.direction.cmp(&self.direction))
            .then_with(|| self.position.0[0].cmp(&other.position.0[0]))
            .then_with(|| self.position.0[1].cmp(&other.position.0[1]))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Maze {
    fn get_start_position(&self) -> Point2<isize> {
        *self
            .0
             .0
            .iter()
            .filter_map(|(pos, tile)| match tile {
                Tile::Start => Some(pos),
                _ => None,
            })
            .next()
            .unwrap()
    }

    fn find_paths(&self) -> (u32, HashSet<Point2<isize>>) {
        let mut queue = BinaryHeap::new();
        let mut best_cost = u32::MAX;
        let mut best_cost_map = HashMap::new();
        let mut best_paths_points = HashSet::new();
        queue.push(State {
            cost: 0,
            position: self.get_start_position(),
            direction: Point2Direction::East,
            visited_points: HashSet::from([self.get_start_position()]),
        });

        while let Some(State {
            cost,
            position,
            direction,
            visited_points,
        }) = queue.pop()
        {
            let mut new_visited_points = visited_points.clone();
            while let Some(merge_state) = queue.pop() {
                if merge_state.cost == cost
                    && merge_state.position == position
                    && merge_state.direction == direction
                {
                    new_visited_points.extend(merge_state.visited_points);
                } else {
                    queue.push(merge_state);
                    break;
                }
            }
            best_cost_map.insert((position, direction), cost);

            match self.0.get(&position) {
                Some(Tile::Wall) => continue,
                Some(Tile::End) => {
                    if cost <= best_cost {
                        best_paths_points.extend(new_visited_points);
                        best_cost = cost;
                    }
                    continue;
                }
                _ => (),
            }

            if best_cost_map
                .get(&(position, direction.direction_left()))
                .is_none_or(|&c| c == cost + 1000)
            {
                queue.push(State {
                    cost: cost + 1000,
                    position,
                    direction: direction.direction_left(),
                    visited_points: new_visited_points.clone(),
                });
            }
            if best_cost_map
                .get(&(position, direction.direction_right()))
                .is_none_or(|&c| c == cost + 1000)
            {
                queue.push(State {
                    cost: cost + 1000,
                    position,
                    direction: direction.direction_right(),
                    visited_points: new_visited_points.clone(),
                });
            }
            if best_cost_map
                .get(&(position.get_point_in_direction(&direction, 1), direction))
                .is_none_or(|&c| c == cost + 1)
            {
                new_visited_points.insert(position.get_point_in_direction(&direction, 1));
                queue.push(State {
                    cost: cost + 1,
                    position: position.get_point_in_direction(&direction, 1),
                    direction,
                    visited_points: new_visited_points,
                });
            }
        }

        (best_cost, best_paths_points)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let maze = Maze::from(input);
    let (cost, _) = maze.find_paths();
    Some(cost)
}

pub fn part_two(input: &str) -> Option<u32> {
    let maze = Maze::from(input);
    let (_, paths) = maze.find_paths();
    Some(paths.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(45));
    }
}
