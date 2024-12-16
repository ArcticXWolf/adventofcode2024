use regex::Regex;

advent_of_code::solution!(3);

const INSTRUCTION_REGEX: &str = r"(mul\((\d{1,3}),(\d{1,3})\))|(do\(\))|(don't\(\))";

enum Instruction {
    Multiplication(u32, u32),
    Do,
    Dont,
}

impl Instruction {
    fn scan(input: &str) -> Vec<Self> {
        let i_regex = Regex::new(INSTRUCTION_REGEX).unwrap();
        let mut results = vec![];
        for capture in i_regex.captures_iter(input) {
            if let Some(x) = capture.get(2) {
                if let Some(y) = capture.get(3) {
                    results.push(Instruction::Multiplication(
                        x.as_str().parse::<u32>().unwrap(),
                        y.as_str().parse::<u32>().unwrap(),
                    ));
                }
            } else if capture.get(4).is_some() {
                results.push(Instruction::Do);
            } else if capture.get(5).is_some() {
                results.push(Instruction::Dont);
            } else {
                unreachable!()
            }
        }
        results
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let instructions = Instruction::scan(input);

    Some(
        instructions
            .iter()
            .filter_map(|i| match i {
                Instruction::Multiplication(x, y) => Some(x * y),
                _ => None,
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let instructions = Instruction::scan(input);
    let mut enabled = true;
    let mut result = 0;

    for instruction in instructions {
        match instruction {
            Instruction::Do => enabled = true,
            Instruction::Dont => enabled = false,
            Instruction::Multiplication(x, y) => {
                if enabled {
                    result += x * y
                }
            }
        }
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
