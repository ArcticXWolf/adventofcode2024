use itertools::Itertools;

advent_of_code::solution!(5);

#[derive(Debug)]
struct Rule {
    before: u32,
    after: u32,
}

impl From<&str> for Rule {
    fn from(value: &str) -> Self {
        let (before, after) = value.split_once("|").unwrap();
        Self {
            before: before.parse().unwrap(),
            after: after.parse().unwrap(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Update {
    pages: Vec<u32>,
}

impl From<&str> for Update {
    fn from(value: &str) -> Self {
        let pages = value
            .split(',')
            .map(|p| p.parse().unwrap())
            .collect::<Vec<_>>();
        Self { pages }
    }
}

impl Update {
    fn is_valid_with_rule(&self, rule: &Rule) -> bool {
        let max_index_before = match self.pages.iter().positions(|p| *p == rule.before).max() {
            Some(i) => i,
            None => return true,
        };
        let min_index_after = match self.pages.iter().position(|p| *p == rule.after) {
            Some(i) => i,
            None => return true,
        };
        max_index_before < min_index_after
    }

    fn is_valid_with_ruleset(&self, rules: &[Rule]) -> bool {
        rules.iter().all(|r| self.is_valid_with_rule(r))
    }

    fn middle_page_number(&self) -> u32 {
        let middle_element = self.pages.len() / 2;
        *self.pages.get(middle_element).unwrap()
    }

    fn fix_with_ruleset(&mut self, rules: &[Rule]) {
        while !self.is_valid_with_ruleset(rules) {
            let failed_rule = match rules.iter().find(|r| !self.is_valid_with_rule(r)) {
                Some(r) => r,
                None => return,
            };

            self.fix_with_rule(failed_rule);
        }
    }

    fn fix_with_rule(&mut self, rule: &Rule) {
        let max_index_before = match self.pages.iter().positions(|p| *p == rule.before).max() {
            Some(i) => i,
            None => return,
        };
        let min_index_after = match self.pages.iter().position(|p| *p == rule.after) {
            Some(i) => i,
            None => return,
        };

        let moving_page = self.pages.remove(min_index_after);
        self.pages.insert(max_index_before, moving_page);
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (rule_str, pages_str) = input.trim().split_once("\n\n").unwrap();

    let rules = rule_str.lines().map(|l| l.into()).collect::<Vec<Rule>>();
    let updates = pages_str.lines().map(|l| l.into()).collect::<Vec<Update>>();

    Some(
        updates
            .iter()
            .filter(|u| u.is_valid_with_ruleset(&rules))
            .map(|u| u.middle_page_number())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rule_str, pages_str) = input.trim().split_once("\n\n").unwrap();

    let rules = rule_str.lines().map(|l| l.into()).collect::<Vec<Rule>>();
    let mut updates = pages_str
        .lines()
        .map(|l| l.into())
        .filter(|u: &Update| !u.is_valid_with_ruleset(&rules))
        .collect::<Vec<Update>>();

    for u in updates.iter_mut() {
        u.fix_with_ruleset(&rules);
    }

    Some(updates.iter().map(|u| u.middle_page_number()).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }

    #[test]
    fn test_rule_fixing() {
        let rules = vec![
            Rule {
                before: 1,
                after: 2,
            },
            Rule {
                before: 2,
                after: 3,
            },
            Rule {
                before: 3,
                after: 4,
            },
        ];

        let mut update = Update::from("1,2,3,4");
        update.fix_with_ruleset(&rules);
        assert_eq!(update, Update::from("1,2,3,4"));

        let mut update = Update::from("2,1,3,4");
        update.fix_with_ruleset(&rules);
        assert_eq!(update, Update::from("1,2,3,4"));
    }
}
