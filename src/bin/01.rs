advent_of_code::solution!(1);

fn parse_lists(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left = vec![];
    let mut right = vec![];

    for line in input.trim().lines() {
        let (l, r) = line.split_once(' ').unwrap();
        left.push(l.trim().parse::<i32>().unwrap());
        right.push(r.trim().parse::<i32>().unwrap());
    }

    (left, right)
}

pub fn part_one(input: &str) -> Option<i32> {
    let (mut left, mut right) = parse_lists(input);
    left.sort();
    right.sort();

    Some(left.iter().zip(right).map(|(l, r)| (l - r).abs()).sum())
}

pub fn part_two(input: &str) -> Option<i32> {
    let (left, right) = parse_lists(input);

    Some(
        left.iter()
            .map(|l| *l * right.iter().filter(|r| *r == l).count() as i32)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
