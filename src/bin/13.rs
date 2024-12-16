advent_of_code::solution!(13);

const INPUT_REGEX: &str =
    r"Button A: X\+(\d+), Y\+(\d+)\s+Button B: X\+(\d+), Y\+(\d+)\s+Prize: X=(\d+), Y=(\d+)";

struct Equation {
    a: (isize, isize),
    b: (isize, isize),
    result: (isize, isize),
}

impl From<&str> for Equation {
    fn from(value: &str) -> Self {
        let re = regex::Regex::new(INPUT_REGEX).unwrap();
        let caps = re.captures(value.trim()).unwrap();
        Self {
            a: (
                caps[1].parse::<isize>().unwrap(),
                caps[2].parse::<isize>().unwrap(),
            ),
            b: (
                caps[3].parse::<isize>().unwrap(),
                caps[4].parse::<isize>().unwrap(),
            ),
            result: (
                caps[5].parse::<isize>().unwrap(),
                caps[6].parse::<isize>().unwrap(),
            ),
        }
    }
}

impl Equation {
    fn solve(&self) -> Option<(isize, isize)> {
        // Calculate:
        // I                => x * a1 + y * b1 = result1
        // II               => x * a2 + y * b2 = result2
        // I * a2 - II * a1
        // insert y in I

        let y_top = self.a.1 * self.result.0 - self.a.0 * self.result.1;
        let y_bottom = self.a.1 * self.b.0 - self.a.0 * self.b.1;
        let (y, y_rem) = (y_top / y_bottom, y_top % y_bottom);

        if y_rem != 0 {
            return None;
        }

        let x_top = self.result.0 - y * self.b.0;
        let x_bottom = self.a.0;
        let (x, x_rem) = (x_top / x_bottom, x_top % x_bottom);

        if x_rem != 0 {
            return None;
        }

        Some((x, y))
    }

    fn token_cost(&self) -> Option<isize> {
        if let Some((x, y)) = self.solve() {
            return Some(x * 3 + y);
        }
        None
    }
}

pub fn part_one(input: &str) -> Option<isize> {
    let equations = input
        .trim()
        .split("\n\n")
        .map(Equation::from)
        .collect::<Vec<Equation>>();

    Some(equations.iter().filter_map(|e| e.token_cost()).sum())
}

pub fn part_two(input: &str) -> Option<isize> {
    let mut equations = input
        .trim()
        .split("\n\n")
        .map(Equation::from)
        .collect::<Vec<Equation>>();

    for e in equations.iter_mut() {
        e.result = (e.result.0 + 10000000000000, e.result.1 + 10000000000000)
    }

    Some(equations.iter().filter_map(|e| e.token_cost()).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}
