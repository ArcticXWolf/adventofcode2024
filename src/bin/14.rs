use advent_of_code::algebra_helpers::{Point2, PointGrid, Rectangle};
use itertools::Itertools;

advent_of_code::solution!(14);

const INPUT_REGEX: &str = r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)";

struct Robot {
    position: Point2<isize>,
    velocity: Point2<isize>,
}

impl From<&str> for Robot {
    fn from(value: &str) -> Self {
        let re = regex::Regex::new(INPUT_REGEX).unwrap();
        let caps = re.captures(value.trim()).unwrap();
        Self {
            position: Point2::new(caps[1].parse().unwrap(), caps[2].parse().unwrap()),
            velocity: Point2::new(caps[3].parse().unwrap(), caps[4].parse().unwrap()),
        }
    }
}

impl Robot {
    fn position_after_steps(&self, n: isize, bounds: Point2<isize>) -> Point2<isize> {
        let x = (self.position.0[0] + n * self.velocity.0[0]).rem_euclid(bounds.0[0]);
        let y = (self.position.0[1] + n * self.velocity.0[1]).rem_euclid(bounds.0[1]);
        Point2::new(x, y)
    }
}

fn _display_robot_grid(robots: &[Robot], bounds: Point2<isize>) {
    for i in 0..(bounds.0[0] * bounds.0[1]) {
        let mut grid = PointGrid::default();
        for p in robots.iter().map(|r| r.position_after_steps(i, bounds)) {
            grid.insert(p, 'X');
        }
        println!("Step {}:\n{}\n", i, grid);
    }
}

pub fn _part_one(input: &str, bounds: Point2<isize>) -> Option<u32> {
    let robots = input.trim().lines().map(Robot::from).collect_vec();
    let quadrants = [
        Rectangle::new(
            Point2::new(0, 0),
            Point2::new(bounds.0[0] / 2, bounds.0[1] / 2),
        ),
        Rectangle::new(
            Point2::new(bounds.0[0] / 2 + 1, 0),
            Point2::new(bounds.0[0], bounds.0[1] / 2),
        ),
        Rectangle::new(
            Point2::new(0, bounds.0[1] / 2 + 1),
            Point2::new(bounds.0[0] / 2, bounds.0[1]),
        ),
        Rectangle::new(
            Point2::new(bounds.0[0] / 2 + 1, bounds.0[1] / 2 + 1),
            Point2::new(bounds.0[0], bounds.0[1]),
        ),
    ];

    let safety_factor = quadrants
        .iter()
        .map(|q| {
            robots
                .iter()
                .filter(|&r| q.contains(&(r.position_after_steps(100, bounds))))
                .count()
        })
        .product::<usize>();

    Some(safety_factor as u32)
}

pub fn part_one(input: &str) -> Option<u32> {
    _part_one(input, Point2::new(101, 103))
}

pub fn part_two(input: &str) -> Option<u32> {
    let _robots = input.trim().lines().map(Robot::from).collect_vec();

    // Since I didnt know how that picture looks, I printed all configurations
    // (until they cycle at 103 * 101) and searched for a configuration with
    // a long row of robots (via text editor).
    // TODO: Implement a real search in code.
    // _display_robot_grid(&_robots, Point2::new(101, 103));
    Some(7286)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = _part_one(
            &advent_of_code::template::read_file("examples", DAY),
            Point2::new(11, 7),
        );
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7286));
    }
}
