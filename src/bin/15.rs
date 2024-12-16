use std::fmt::Display;

use advent_of_code::algebra_helpers::{Point2, Point2Direction, PointGrid};

advent_of_code::solution!(15);

#[derive(Debug, Default, Clone, Copy)]
enum Tile {
    #[default]
    Wall,
    Box,
    LeftBox,
    RightBox,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Wall => '#',
                Tile::Box => 'O',
                Tile::LeftBox => '[',
                Tile::RightBox => ']',
            }
        )
    }
}

#[derive(Debug)]
struct Warehouse {
    map: PointGrid<isize, 2, Tile>,
    robot_pos: Point2<isize>,
    robot_instruction_counter: usize,
    robot_instructions: Vec<Point2Direction>,
}

impl From<&str> for Warehouse {
    fn from(value: &str) -> Self {
        let mut map = PointGrid::default();
        let mut robot_pos = Point2::zero();
        let mut robot_instructions = vec![];

        let (map_str, instruction_str) = value.trim().split_once("\n\n").unwrap();
        for (y, row) in map_str.trim().lines().enumerate() {
            for (x, c) in row.chars().enumerate() {
                let position = Point2::new(x as isize, y as isize);
                match c {
                    '#' => map.insert(position, Tile::Wall),
                    'O' => map.insert(position, Tile::Box),
                    '@' => robot_pos = position,
                    _ => (),
                }
            }
        }

        for l in instruction_str.trim().lines() {
            for c in l.trim().chars() {
                robot_instructions.push(match c {
                    '^' => Point2Direction::North,
                    '>' => Point2Direction::East,
                    'v' => Point2Direction::South,
                    '<' => Point2Direction::West,
                    _ => unreachable!(),
                });
            }
        }

        Self {
            map,
            robot_pos,
            robot_instruction_counter: 0,
            robot_instructions,
        }
    }
}

impl Display for Warehouse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Warehouse ({:?}, {})",
            self.robot_pos, self.robot_instruction_counter
        )?;
        let (min, max) = self.map.dimensions();
        for y in min.0[1]..(max.0[1] + 1) {
            for x in min.0[0]..(max.0[0] + 1) {
                if let Some(t) = self.map.get(&Point2::new(x, y)) {
                    write!(f, "{}", t)?;
                } else if Point2::new(x, y) == self.robot_pos {
                    write!(f, "@")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        writeln!(f)
    }
}

impl Warehouse {
    fn double(&mut self) {
        let mut new_map = PointGrid::default();

        let (min, max) = self.map.dimensions();
        for y in min.0[1]..(max.0[1] + 1) {
            for x in min.0[0]..(max.0[0] + 1) {
                if let Some(t) = self.map.get(&Point2::new(x, y)) {
                    match t {
                        Tile::Wall => {
                            new_map.insert(Point2::new(2 * x, y), Tile::Wall);
                            new_map.insert(Point2::new(2 * x + 1, y), Tile::Wall);
                        }
                        Tile::Box => {
                            new_map.insert(Point2::new(2 * x, y), Tile::LeftBox);
                            new_map.insert(Point2::new(2 * x + 1, y), Tile::RightBox);
                        }
                        _ => (),
                    }
                }
            }
        }

        self.map = new_map;
        self.robot_pos = Point2::new(2 * self.robot_pos.0[0], self.robot_pos.0[1]);
    }

    // This is not great, because I wanted to keep using my PointGrid and Tiling.
    // TODO: Refactor everything by storing an (Obj->Pos)-Map and making true
    // "two-wide" objects.
    fn step(&mut self) -> bool {
        if self.robot_instruction_counter >= self.robot_instructions.len() {
            return false;
        }

        let direction = self
            .robot_instructions
            .get(self.robot_instruction_counter)
            .expect("Instruction counter should not overflow");
        self.robot_instruction_counter += 1;

        let new_robot_pos = self.robot_pos.get_point_in_direction(direction, 1);
        let mut new_map = self.map.clone();
        let mut box_stack = vec![];
        if let Some(t) = self.map.get(&new_robot_pos) {
            match (t, direction) {
                (Tile::Wall, _) => return true,
                (Tile::Box, _)
                | (Tile::LeftBox, Point2Direction::East)
                | (Tile::LeftBox, Point2Direction::North)
                | (Tile::LeftBox, Point2Direction::South) => box_stack.push((new_robot_pos, 0)),
                (Tile::RightBox, Point2Direction::West)
                | (Tile::RightBox, Point2Direction::North)
                | (Tile::RightBox, Point2Direction::South) => box_stack.push((
                    new_robot_pos.get_point_in_direction(&Point2Direction::West, 1),
                    0,
                )),
                _ => unreachable!("Impossible box push"),
            }
        }

        while let Some((box_position, count)) = box_stack.pop() {
            if let Some(t) = new_map.get(&box_position) {
                match (t, count) {
                    (Tile::Wall, _) => return true,
                    (Tile::Box, 0) => {
                        box_stack.push((box_position, 1));
                        box_stack.push((box_position.get_point_in_direction(direction, 1), 0));
                    }
                    (Tile::Box, _) => {
                        new_map.0.remove(&box_position);
                        new_map
                            .insert(box_position.get_point_in_direction(direction, 1), Tile::Box);
                    }
                    (Tile::RightBox, x) => box_stack.push((
                        box_position.get_point_in_direction(&Point2Direction::West, 1),
                        x,
                    )),
                    (Tile::LeftBox, 0) => match direction {
                        Point2Direction::North | Point2Direction::South => {
                            box_stack.push((box_position, 1));
                            box_stack.push((box_position.get_point_in_direction(direction, 1), 0));
                        }
                        Point2Direction::East => {
                            box_stack.push((box_position, 1));
                            box_stack.push((box_position.get_point_in_direction(direction, 2), 0));
                        }
                        Point2Direction::West => {
                            box_stack.push((box_position, 1));
                            box_stack.push((box_position.get_point_in_direction(direction, 1), 0));
                        }
                        _ => unreachable!(),
                    },
                    (Tile::LeftBox, 1) => match direction {
                        Point2Direction::North | Point2Direction::South => {
                            box_stack.push((box_position, 2));
                            box_stack.push((
                                box_position
                                    .get_point_in_direction(&Point2Direction::East, 1)
                                    .get_point_in_direction(direction, 1),
                                0,
                            ));
                        }
                        Point2Direction::East => {
                            new_map.0.remove(&box_position);
                            new_map.0.remove(
                                &box_position.get_point_in_direction(&Point2Direction::East, 1),
                            );
                            new_map.insert(
                                box_position.get_point_in_direction(direction, 1),
                                Tile::LeftBox,
                            );
                            new_map.insert(
                                box_position.get_point_in_direction(direction, 2),
                                Tile::RightBox,
                            );
                        }
                        Point2Direction::West => {
                            new_map.0.remove(&box_position);
                            new_map.0.remove(
                                &box_position.get_point_in_direction(&Point2Direction::East, 1),
                            );
                            new_map.insert(
                                box_position.get_point_in_direction(direction, 1),
                                Tile::LeftBox,
                            );
                            new_map.insert(box_position, Tile::RightBox);
                        }
                        _ => unreachable!(),
                    },
                    (Tile::LeftBox, 2) => match direction {
                        Point2Direction::North | Point2Direction::South => {
                            new_map.0.remove(&box_position);
                            new_map.0.remove(
                                &box_position.get_point_in_direction(&Point2Direction::East, 1),
                            );
                            new_map.insert(
                                box_position.get_point_in_direction(direction, 1),
                                Tile::LeftBox,
                            );
                            new_map.insert(
                                box_position
                                    .get_point_in_direction(&Point2Direction::East, 1)
                                    .get_point_in_direction(direction, 1),
                                Tile::RightBox,
                            );
                        }
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                }
            }
        }
        self.map = new_map;
        self.robot_pos = new_robot_pos;
        true
    }

    fn run(&mut self) {
        while self.step() {}
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut warehouse = Warehouse::from(input);
    warehouse.run();
    Some(
        warehouse
            .map
            .0
            .iter()
            .filter_map(|(p, t)| match t {
                Tile::Box => Some(100 * p.0[1] + p.0[0]),
                _ => None,
            })
            .sum::<isize>() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut warehouse = Warehouse::from(input);
    warehouse.double();
    warehouse.run();
    Some(
        warehouse
            .map
            .0
            .iter()
            .filter_map(|(p, t)| match t {
                Tile::LeftBox => Some(100 * p.0[1] + p.0[0]),
                _ => None,
            })
            .sum::<isize>() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
}
