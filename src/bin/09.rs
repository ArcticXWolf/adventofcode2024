use std::fmt::Display;

advent_of_code::solution!(9);

#[derive(Debug)]
enum DriveContent {
    Free(usize),        // length
    File(usize, usize), // length, id
}

impl Display for DriveContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (&length, character) = match self {
            DriveContent::Free(l) => (l, '.'),
            DriveContent::File(l, id) => (l, (id % 10).to_string().chars().next().unwrap()),
        };
        for i in 0..length {
            write!(f, "{}", character)?;
        }
        write!(f, "")
    }
}

#[derive(Debug)]
struct Drive(Vec<DriveContent>);

impl Drive {
    fn get_first_free_of_size(&self, size: usize) -> Option<(usize, usize)> {
        self.0
            .iter()
            .enumerate()
            .filter_map(|(idx, dc)| match dc {
                DriveContent::Free(l) => {
                    if *l >= size {
                        Some((idx, *l))
                    } else {
                        None
                    }
                }
                DriveContent::File(_, _) => None,
            })
            .nth(0)
    }

    fn get_last_file(&self) -> (usize, usize, usize) {
        self.0
            .iter()
            .enumerate()
            .filter_map(|(idx, dc)| match dc {
                DriveContent::Free(_) => None,
                DriveContent::File(l, id) => Some((idx, *l, *id)),
            })
            .last()
            .expect("There should always be a file block")
    }

    fn get_file_of_id(&self, search_id: usize) -> Option<(usize, usize, usize)> {
        self.0
            .iter()
            .enumerate()
            .filter_map(|(idx, dc)| match dc {
                DriveContent::File(l, id) if *id == search_id => Some((idx, *l, *id)),
                _ => None,
            })
            .last()
    }

    fn compress_once_part1(&mut self) -> bool {
        let (free_idx, free_length) = self
            .get_first_free_of_size(1)
            .expect("There should always be a free block");
        let (file_idx, file_length, file_id) = self.get_last_file();

        if free_idx > file_idx {
            return false;
        }

        let length_to_swap = free_length.min(file_length);

        // swap free
        if let Some(dc) = self.0.get_mut(free_idx) {
            *dc = DriveContent::File(length_to_swap, file_id);
        }
        // swap file
        if let Some(dc) = self.0.get_mut(file_idx) {
            *dc = DriveContent::Free(length_to_swap);
        }

        // cleanup leftovers
        if free_length > length_to_swap {
            self.0.insert(
                free_idx + 1,
                DriveContent::Free(free_length - length_to_swap),
            );
        } else if file_length > length_to_swap {
            self.0.insert(
                file_idx,
                DriveContent::File(file_length - length_to_swap, file_id),
            );
        }

        true
    }

    fn compress_part1(&mut self) {
        while self.compress_once_part1() {}
    }

    fn compress_once_part2(&mut self, file_id: usize) -> bool {
        let file_info = self.get_file_of_id(file_id);
        if file_info.is_none() {
            return false;
        }

        let (file_idx, file_length, _) = file_info.unwrap();
        let free_info = self.get_first_free_of_size(file_length);
        if free_info.is_none() {
            return false;
        }

        let (free_idx, free_length) = free_info.unwrap();
        if free_idx > file_idx {
            return false;
        }

        // swap free
        if let Some(dc) = self.0.get_mut(free_idx) {
            *dc = DriveContent::File(file_length, file_id);
        }
        // swap file
        if let Some(dc) = self.0.get_mut(file_idx) {
            *dc = DriveContent::Free(file_length);
        }

        // cleanup leftovers
        if free_length > file_length {
            self.0
                .insert(free_idx + 1, DriveContent::Free(free_length - file_length));
        }

        true
    }

    fn compress_part2(&mut self) {
        let (_, _, max_file_id) = self.get_last_file();

        for file_id in (0..=max_file_id).rev() {
            self.compress_once_part2(file_id);
        }
    }

    fn checksum(&self) -> usize {
        let mut acc = 0;
        let mut block_idx = 0;

        for dc in &self.0 {
            match dc {
                DriveContent::Free(l) => block_idx += l,
                DriveContent::File(l, file_id) => {
                    for i in block_idx..(block_idx + l) {
                        acc += i * file_id;
                    }
                    block_idx += l;
                }
            }
        }

        acc
    }
}

impl From<&str> for Drive {
    fn from(value: &str) -> Self {
        let mut result = vec![];
        for (i, c) in value.trim().chars().enumerate() {
            let length = c.to_digit(10).expect("Length has to be a digit") as usize;
            if i % 2 == 0 {
                result.push(DriveContent::File(length, i / 2));
            } else {
                result.push(DriveContent::Free(length));
            }
        }
        Drive(result)
    }
}

impl Display for Drive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in &self.0 {
            write!(f, "{}", i)?;
        }
        write!(f, "")
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut drive = Drive::from(input);
    drive.compress_part1();
    Some(drive.checksum())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut drive = Drive::from(input);
    drive.compress_part2();
    Some(drive.checksum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
