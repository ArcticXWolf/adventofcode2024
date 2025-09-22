#![feature(trim_prefix_suffix)]
use std::fmt::{Display, Write};

use advent_of_code::algebra_helpers::{Point2, Point2Direction};
use itertools::Itertools;

advent_of_code::solution!(21);

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum NumericKeypadButton {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Accept,
}

impl TryFrom<char> for NumericKeypadButton {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '0' => Ok(Self::Zero),
            '1' => Ok(Self::One),
            '2' => Ok(Self::Two),
            '3' => Ok(Self::Three),
            '4' => Ok(Self::Four),
            '5' => Ok(Self::Five),
            '6' => Ok(Self::Six),
            '7' => Ok(Self::Seven),
            '8' => Ok(Self::Eight),
            '9' => Ok(Self::Nine),
            'A' => Ok(Self::Accept),
            _ => Err(()),
        }
    }
}

impl NumericKeypadButton {
    fn forbidden_positions() -> Vec<Point2<isize>> {
        vec![Point2::new(0, 0)]
    }

    fn button_position(&self) -> Point2<isize> {
        match self {
            NumericKeypadButton::Zero => Point2::new(1, 0),
            NumericKeypadButton::One => Point2::new(0, 1),
            NumericKeypadButton::Two => Point2::new(1, 1),
            NumericKeypadButton::Three => Point2::new(2, 1),
            NumericKeypadButton::Four => Point2::new(0, 2),
            NumericKeypadButton::Five => Point2::new(1, 2),
            NumericKeypadButton::Six => Point2::new(2, 2),
            NumericKeypadButton::Seven => Point2::new(0, 3),
            NumericKeypadButton::Eight => Point2::new(1, 3),
            NumericKeypadButton::Nine => Point2::new(2, 3),
            NumericKeypadButton::Accept => Point2::new(2, 0),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum DirectionalKeypadButton {
    Up,
    Right,
    Down,
    Left,
    Accept,
}

impl TryFrom<char> for DirectionalKeypadButton {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(Self::Up),
            '>' => Ok(Self::Right),
            'v' => Ok(Self::Down),
            '<' => Ok(Self::Left),
            'A' => Ok(Self::Accept),
            _ => Err(()),
        }
    }
}

impl Display for DirectionalKeypadButton {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DirectionalKeypadButton::Up => f.write_char('^'),
            DirectionalKeypadButton::Right => f.write_char('>'),
            DirectionalKeypadButton::Down => f.write_char('v'),
            DirectionalKeypadButton::Left => f.write_char('<'),
            DirectionalKeypadButton::Accept => f.write_char('A'),
        }
    }
}

impl DirectionalKeypadButton {
    fn forbidden_positions() -> Vec<Point2<isize>> {
        vec![Point2::new(0, 1)]
    }

    fn button_position(&self) -> Point2<isize> {
        match self {
            &DirectionalKeypadButton::Up => Point2::new(1, 1),
            &DirectionalKeypadButton::Right => Point2::new(2, 0),
            &DirectionalKeypadButton::Down => Point2::new(1, 0),
            &DirectionalKeypadButton::Left => Point2::new(0, 0),
            &DirectionalKeypadButton::Accept => Point2::new(2, 1),
        }
    }

    fn direction(&self) -> Point2Direction {
        match self {
            DirectionalKeypadButton::Up => Point2Direction::South,
            DirectionalKeypadButton::Right => Point2Direction::East,
            DirectionalKeypadButton::Down => Point2Direction::North,
            DirectionalKeypadButton::Left => Point2Direction::West,
            DirectionalKeypadButton::Accept => unreachable!(),
        }
    }
}

fn parse_code(code: &str) -> Vec<NumericKeypadButton> {
    code.trim()
        .chars()
        .map(|c| NumericKeypadButton::try_from(c).unwrap())
        .collect::<Vec<NumericKeypadButton>>()
}

fn navigate_path_numerical(buttons: Vec<NumericKeypadButton>) -> Vec<Vec<DirectionalKeypadButton>> {
    navigate_path(
        NumericKeypadButton::Accept.button_position(),
        &buttons
            .iter()
            .map(|b| b.button_position())
            .collect::<Vec<_>>(),
        &NumericKeypadButton::forbidden_positions(),
    )
}

fn navigate_paths_directional(
    paths: &[Vec<DirectionalKeypadButton>],
) -> Vec<Vec<DirectionalKeypadButton>> {
    paths
        .iter()
        .map(|path| navigate_path_directional(path))
        .flatten()
        .collect::<Vec<_>>()
}

fn navigate_path_directional(
    path: &[DirectionalKeypadButton],
) -> Vec<Vec<DirectionalKeypadButton>> {
    navigate_path(
        DirectionalKeypadButton::Accept.button_position(),
        &path.iter().map(|d| d.button_position()).collect::<Vec<_>>(),
        &DirectionalKeypadButton::forbidden_positions(),
    )
}

fn navigate_path(
    source: Point2<isize>,
    path: &[Point2<isize>],
    forbidden_positions: &[Point2<isize>],
) -> Vec<Vec<DirectionalKeypadButton>> {
    let mut possible_paths: Vec<Vec<DirectionalKeypadButton>> = vec![vec![]];
    let mut current: Point2<isize> = source;

    for destination in path {
        let new_paths = get_paths_to_position(current, *destination, forbidden_positions);
        possible_paths = possible_paths
            .into_iter()
            .map(|p| {
                new_paths
                    .iter()
                    .cloned()
                    .map(|np| {
                        p.iter()
                            .chain(&np)
                            .cloned()
                            .collect::<Vec<DirectionalKeypadButton>>()
                    })
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect::<Vec<_>>();
        current = *destination;
    }

    possible_paths
}

fn get_paths_to_position(
    source: Point2<isize>,
    destination: Point2<isize>,
    forbidden_positions: &[Point2<isize>],
) -> Vec<Vec<DirectionalKeypadButton>> {
    let mut directions: Vec<DirectionalKeypadButton> = vec![];
    let x_diff = destination.0[0] - source.0[0];
    if x_diff >= 0 {
        directions.extend_from_slice(&[DirectionalKeypadButton::Right].repeat(x_diff as usize));
    } else {
        directions
            .extend_from_slice(&[DirectionalKeypadButton::Left].repeat((x_diff * -1) as usize));
    }

    let y_diff = destination.0[1] - source.0[1];
    if y_diff >= 0 {
        directions.extend_from_slice(&[DirectionalKeypadButton::Up].repeat(y_diff as usize));
    } else {
        directions
            .extend_from_slice(&[DirectionalKeypadButton::Down].repeat((y_diff * -1) as usize));
    }

    let length = directions.len();
    directions
        .into_iter()
        .permutations(length)
        .unique()
        .filter(|path| {
            let mut pos = source;
            for dir in path {
                pos = pos.get_point_in_direction(&dir.direction(), 1);
                if forbidden_positions.contains(&pos) {
                    return false;
                }
            }
            true
        })
        .map(|d| {
            d.iter()
                .chain([&DirectionalKeypadButton::Accept])
                .cloned()
                .collect()
        })
        .collect()
}

fn calculate_min_path_length(
    buttons: Vec<NumericKeypadButton>,
    amount_of_intermediate_robots: u32,
) -> u32 {
    let mut paths = navigate_path_numerical(buttons);
    let mut current_min_length = paths.iter().map(|p| p.len()).min().unwrap();
    paths = paths
        .into_iter()
        .filter(|p| p.len() <= current_min_length)
        .collect();

    for _ in 0..amount_of_intermediate_robots {
        paths = navigate_paths_directional(&paths);
        current_min_length = paths.iter().map(|p| p.len()).min().unwrap();
        paths = paths
            .into_iter()
            .filter(|p| p.len() <= current_min_length)
            .collect();
    }

    paths = navigate_paths_directional(&paths);
    current_min_length = paths.iter().map(|p| p.len()).min().unwrap();
    paths = paths
        .into_iter()
        .filter(|p| p.len() <= current_min_length)
        .collect();
    current_min_length as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut result = 0;
    for code in input.lines() {
        let numerical = code.trim_suffix('A').parse::<u32>().unwrap();
        let length = calculate_min_path_length(parse_code(code), 1);
        result += numerical * length;
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut result = 0;
    for code in input.lines() {
        let numerical = code.trim_suffix('A').parse::<u32>().unwrap();
        let length = calculate_min_path_length(parse_code(code), 25);
        result += numerical * length;
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(126384));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_get_paths_to_position() {
        assert_eq!(
            get_paths_to_position(Point2::new(0, 0), Point2::new(1, 1), &vec![]).len(),
            2
        );
        assert_eq!(
            get_paths_to_position(Point2::new(0, 0), Point2::new(2, 1), &vec![]).len(),
            3
        );
        assert_eq!(
            get_paths_to_position(
                Point2::new(1, 0),
                Point2::new(0, 2),
                &vec![Point2::new(0, 0)]
            )
            .len(),
            2
        );
        assert_eq!(
            get_paths_to_position(
                Point2::new(0, 1),
                Point2::new(0, 3),
                &vec![Point2::new(0, 0)]
            )
            .len(),
            1
        );
    }

    #[test]
    fn test_parse_code() {
        assert_eq!(
            parse_code("0123456789A"),
            vec![
                NumericKeypadButton::Zero,
                NumericKeypadButton::One,
                NumericKeypadButton::Two,
                NumericKeypadButton::Three,
                NumericKeypadButton::Four,
                NumericKeypadButton::Five,
                NumericKeypadButton::Six,
                NumericKeypadButton::Seven,
                NumericKeypadButton::Eight,
                NumericKeypadButton::Nine,
                NumericKeypadButton::Accept,
            ]
        );
    }

    #[test]
    fn test_navigate_path() {
        let mut paths = navigate_path_numerical(parse_code("179A"));
        assert_eq!(paths.len(), 2);

        paths = navigate_path_numerical(parse_code("029A"));
        assert_eq!(paths.len(), 3);

        paths = navigate_paths_directional(&paths);
        assert_eq!(paths.len(), 128);
    }

    #[test]
    fn test_known_codes() {
        let mut code = "029A";
        println!("Code: {}", code);
        assert_eq!(calculate_min_path_length(parse_code(code), 2), 68);
        code = "980A";
        println!("Code: {}", code);
        assert_eq!(calculate_min_path_length(parse_code(code), 1), 60);
        code = "179A";
        println!("Code: {}", code);
        assert_eq!(calculate_min_path_length(parse_code(code), 1), 68);
        code = "456A";
        println!("Code: {}", code);
        assert_eq!(calculate_min_path_length(parse_code(code), 1), 64);
        code = "379A";
        println!("Code: {}", code);
        assert_eq!(calculate_min_path_length(parse_code(code), 1), 64);
    }
}
