use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

use advent_of_code::algebra_helpers::{Point2, Point2Direction, Rectangle};

advent_of_code::solution!(18);

// TODO: I think my A* implementation is broken here, it is way to slow. But it
//       still works.

struct Map {
    points: Vec<Point2<isize>>,
    bounds: Rectangle<isize>,
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let mut result = vec![];
        let (mut min_x, mut max_x) = (100, 0);
        let (mut min_y, mut max_y) = (100, 0);
        for l in value.trim().lines() {
            let (x_str, y_str) = l.split_once(',').expect("invalid coordinates in input");
            let (x, y) = (
                x_str
                    .parse::<isize>()
                    .expect("invalid x coordinate in input"),
                y_str
                    .parse::<isize>()
                    .expect("invalid x coordinate in input"),
            );
            (min_x, max_x) = (min_x.min(x), max_x.max(x));
            (min_y, max_y) = (min_y.min(y), max_y.max(y));
            result.push(Point2::new(x, y));
        }
        Self {
            points: result,
            bounds: Rectangle::new(Point2::new(min_x, min_y), Point2::new(max_x + 1, max_y + 1)),
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
struct State {
    cost: u32,
    position: Point2<isize>,
    time: u32,
    path: Vec<Point2<isize>>,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // we flip the Ord here, so the max-heap becomes a min-heap
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Map {
    fn passable_at_time(&self, position: &Point2<isize>, time: u32) -> bool {
        if !self.bounds.contains(position) {
            return false;
        }
        if let Some(time_position_is_corrupted) = self.points.iter().position(|p| p == position) {
            time <= time_position_is_corrupted as u32
        } else {
            true
        }
    }

    fn _print_map_at_time(&self, time: u32, other_positions: &[Point2<isize>]) {
        for y in self.bounds.min.0[1]..self.bounds.max.0[1] {
            for x in self.bounds.min.0[0]..self.bounds.max.0[0] {
                let position = Point2::new(x, y);
                if !self.passable_at_time(&position, time) {
                    print!("#");
                } else if other_positions.contains(&position) {
                    print!("O");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    fn find_shortest_path(
        &self,
        start_pos: &Point2<isize>,
        end_pos: &Point2<isize>,
        fixed_time: Option<u32>,
    ) -> u32 {
        let mut queue = BinaryHeap::new();
        let mut closed_nodes = HashMap::new();
        queue.push(State {
            cost: 0,
            position: *start_pos,
            time: 0,
            path: vec![*start_pos],
        });

        while let Some(State {
            cost: _,
            position,
            time,
            path,
        }) = queue.pop()
        {
            closed_nodes.insert(position, time);
            if &position == end_pos {
                // let map_time = fixed_time.unwrap_or(time);
                // self._print_map_at_time(map_time, &path);
                return time;
            }

            for pd in Point2Direction::all() {
                let next_position = position.get_point_in_direction(pd, 1);
                let next_cost = time + 1;
                let map_time = fixed_time.unwrap_or(next_cost);
                if self.passable_at_time(&next_position, map_time)
                    && closed_nodes
                        .get(&next_position)
                        .is_none_or(|c| next_cost < *c)
                {
                    let heuristic = (end_pos.0[0] - next_position.0[0]).abs()
                        + (end_pos.0[1] - next_position.0[1]).abs();
                    let mut new_path = path.clone();
                    new_path.push(next_position);
                    queue.push(State {
                        cost: next_cost + heuristic as u32,
                        position: next_position,
                        time: next_cost,
                        path: new_path,
                    });
                }
            }
        }

        0
    }
}

pub fn _part_one(input: &str, fixed_time: Option<u32>, bounds: Rectangle<isize>) -> Option<u32> {
    let mut map = Map::from(input);
    map.bounds = bounds;

    Some(map.find_shortest_path(
        &map.bounds.min,
        &(map.bounds.max - Point2::one()),
        fixed_time,
    ))
}

pub fn part_one(input: &str) -> Option<u32> {
    _part_one(
        input,
        Some(1024),
        Rectangle::new(Point2::zero(), Point2::new(71, 71)),
    )
}

pub fn _part_two(input: &str, bounds: Rectangle<isize>) -> Option<String> {
    let mut map = Map::from(input);
    map.bounds = bounds;

    // Binary search for the correct value because my A* implementation is slow.
    let (mut lower_bound, mut upper_bound) = (0, map.points.len());
    while lower_bound < upper_bound {
        let mid = lower_bound + (upper_bound - lower_bound) / 2;
        let result = map.find_shortest_path(
            &map.bounds.min,
            &(map.bounds.max - Point2::one()),
            Some((mid + 1) as u32),
        );

        if result == 0 {
            upper_bound = mid;
        } else {
            lower_bound = mid + 1;
        }

        if upper_bound == lower_bound {
            let p = map.points.get(lower_bound).unwrap();
            return Some(format!("{},{}", p.0[0], p.0[1]));
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<String> {
    _part_two(input, Rectangle::new(Point2::zero(), Point2::new(71, 71)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = _part_one(
            &advent_of_code::template::read_file("examples", DAY),
            Some(12),
            Rectangle::new(Point2::zero(), Point2::new(7, 7)),
        );
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = _part_two(
            &advent_of_code::template::read_file("examples", DAY),
            Rectangle::new(Point2::zero(), Point2::new(7, 7)),
        );
        assert_eq!(result, Some("6,1".to_string()));
    }
}
