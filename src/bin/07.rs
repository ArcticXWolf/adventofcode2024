use itertools::{repeat_n, Itertools, MultiProduct};

advent_of_code::solution!(7);

#[derive(Debug, Clone, Copy)]
enum Operator {
    Addition,
    Multiplication,
    Concatenation,
}

impl Operator {
    fn get_iterator(length: usize, with_concatenation: bool) -> OperatorIterator {
        let iterator: MultiProduct<std::vec::IntoIter<Operator>> = if with_concatenation {
            repeat_n(
                vec![
                    Operator::Addition,
                    Operator::Multiplication,
                    Operator::Concatenation,
                ],
                length,
            )
            .multi_cartesian_product()
        } else {
            repeat_n(vec![Operator::Addition, Operator::Multiplication], length)
                .multi_cartesian_product()
        };
        OperatorIterator { iterator }
    }
}
struct OperatorIterator {
    iterator: MultiProduct<std::vec::IntoIter<Operator>>,
}

impl Iterator for OperatorIterator {
    type Item = Vec<Operator>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator.next()
    }
}

struct Equation {
    test_value: u64,
    numbers: Vec<u64>,
}

impl From<&str> for Equation {
    fn from(value: &str) -> Self {
        let (test_value_str, number_str) = value.split_once(": ").unwrap();
        let numbers = number_str
            .split_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        Self {
            test_value: test_value_str.parse().unwrap(),
            numbers,
        }
    }
}

impl Equation {
    fn is_valid_with_operators(&self, operators: &[Operator]) -> bool {
        let first_element = *self.numbers.first().unwrap();

        let result =
            self.numbers
                .iter()
                .skip(1)
                .zip(operators)
                .fold(first_element, |acc, (n, o)| match o {
                    Operator::Addition => acc + *n,
                    Operator::Multiplication => acc * *n,
                    Operator::Concatenation => {
                        acc * 10_u64.pow((*n).checked_ilog10().unwrap_or(0) + 1) + *n
                        // could be faster by caching the number of digits during parsing
                    }
                });
        result == self.test_value
    }

    fn has_valid_operator_set(&self, with_concatenation: bool) -> bool {
        Operator::get_iterator(self.numbers.len() - 1, with_concatenation)
            .any(|os| self.is_valid_with_operators(&os))
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let equations = input
        .trim()
        .lines()
        .map(|l| l.into())
        .filter(|e: &Equation| e.has_valid_operator_set(false))
        .collect::<Vec<Equation>>();

    Some(equations.iter().map(|e| e.test_value).sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let equations = input
        .trim()
        .lines()
        .map(|l| l.into())
        .filter(|e: &Equation| e.has_valid_operator_set(true))
        .collect::<Vec<Equation>>();

    Some(equations.iter().map(|e| e.test_value).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
