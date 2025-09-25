advent_of_code::solution!(25);

struct Key((usize, usize, usize, usize, usize));
impl TryFrom<&str> for Key {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut depth = [0; 5];
        for (lidx, l) in value.lines().enumerate() {
            if lidx == 0 && l != "....." {
                return Err(());
            }
            for (cidx, c) in l.char_indices() {
                match c {
                    '#' => depth[cidx] += 1,
                    '.' => {}
                    _ => return Err(()),
                }
            }
        }
        Ok(Self((depth[0], depth[1], depth[2], depth[3], depth[4])))
    }
}

struct Lock((usize, usize, usize, usize, usize));
impl TryFrom<&str> for Lock {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut depth = [0; 5];
        for (lidx, l) in value.lines().enumerate() {
            if lidx == 0 && l != "#####" {
                return Err(());
            }
            for (cidx, c) in l.char_indices() {
                match c {
                    '#' => depth[cidx] += 1,
                    '.' => {}
                    _ => return Err(()),
                }
            }
        }
        Ok(Self((depth[0], depth[1], depth[2], depth[3], depth[4])))
    }
}

fn does_key_fit_in_lock(key: &Key, lock: &Lock) -> bool {
    key.0.0 + lock.0.0 <= 7
        && key.0.1 + lock.0.1 <= 7
        && key.0.2 + lock.0.2 <= 7
        && key.0.3 + lock.0.3 <= 7
        && key.0.4 + lock.0.4 <= 7
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut keys = vec![];
    let mut locks = vec![];
    for element in input.trim().split("\n\n") {
        if let Ok(key) = Key::try_from(element) {
            keys.push(key);
        }
        if let Ok(lock) = Lock::try_from(element) {
            locks.push(lock);
        }
    }

    let mut result = 0;

    for l in &locks {
        for k in &keys {
            if does_key_fit_in_lock(k, l) {
                result += 1;
            }
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
