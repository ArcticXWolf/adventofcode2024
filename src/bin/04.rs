use advent_of_code::algebra_helpers::{Point2, Point2Direction, PointGrid, PointGridIterator};

advent_of_code::solution!(4);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XMAS {
    X,
    M,
    A,
    S,
}

const XMAS_WORD: &[XMAS] = &[XMAS::X, XMAS::M, XMAS::A, XMAS::S];
const MAS_WORD: &[XMAS] = &[XMAS::M, XMAS::A, XMAS::S];

struct WordGrid(PointGrid<isize, 2, XMAS>);

impl TryFrom<&str> for WordGrid {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut grid = PointGrid::default();
        for (y, row) in value.trim().lines().enumerate() {
            for (x, c) in row.chars().enumerate() {
                grid.insert(
                    Point2::new(x as isize, y as isize),
                    match c {
                        'X' => XMAS::X,
                        'M' => XMAS::M,
                        'A' => XMAS::A,
                        'S' => XMAS::S,
                        _ => return Err(()),
                    },
                );
            }
        }

        Ok(Self { 0: grid })
    }
}

impl WordGrid {
    fn is_word_into_direction(
        &self,
        position: &Point2<isize>,
        direction: &Point2Direction,
        word: &[XMAS],
    ) -> bool {
        for (i, c) in word.iter().enumerate() {
            let current_position = position.get_point_in_direction(direction, i as isize);
            if self.0.get(&current_position) != Some(c) {
                return false;
            }
        }
        true
    }

    fn character_iter(&self, character_to_filter: XMAS) -> CharacterGridIterator {
        CharacterGridIterator {
            grid: self,
            character_to_filter: character_to_filter,
            iterator: self.0.iter_full_bounds(),
        }
    }

    fn x_mas_iter(&self) -> XMASIterator {
        XMASIterator {
            grid: self,
            iterator: self.character_iter(XMAS::M),
        }
    }
}

struct CharacterGridIterator<'a> {
    grid: &'a WordGrid,
    character_to_filter: XMAS,
    iterator: PointGridIterator<isize, 2>,
}

impl<'a> Iterator for CharacterGridIterator<'a> {
    type Item = (Point2<isize>, XMAS);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(p) = self.iterator.next() {
            if self.grid.0.get(&p) == Some(&self.character_to_filter) {
                return Some((p.clone(), self.character_to_filter.clone()));
            }
        }
        None
    }
}

struct XMASIterator<'a> {
    grid: &'a WordGrid,
    iterator: CharacterGridIterator<'a>,
}

impl<'a> Iterator for XMASIterator<'a> {
    type Item = (Point2<isize>, u32);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((p, _)) = self.iterator.next() {
            let mut count = 0;
            // check for x mas into southeast direction
            if self
                .grid
                .is_word_into_direction(&p, &Point2Direction::SouthEast, MAS_WORD)
                && (self.grid.is_word_into_direction(
                    &p.get_point_in_direction(&Point2Direction::East, 2),
                    &Point2Direction::SouthWest,
                    MAS_WORD,
                ) || self.grid.is_word_into_direction(
                    &p.get_point_in_direction(&Point2Direction::South, 2),
                    &Point2Direction::NorthEast,
                    MAS_WORD,
                ))
            {
                count += 1;
            }
            // check for x mas into northwest direction
            if self
                .grid
                .is_word_into_direction(&p, &Point2Direction::NorthWest, MAS_WORD)
                && (self.grid.is_word_into_direction(
                    &p.get_point_in_direction(&Point2Direction::North, 2),
                    &Point2Direction::SouthWest,
                    MAS_WORD,
                ) || self.grid.is_word_into_direction(
                    &p.get_point_in_direction(&Point2Direction::West, 2),
                    &Point2Direction::NorthEast,
                    MAS_WORD,
                ))
            {
                count += 1;
            }
            if count > 0 {
                return Some((p, count));
            }
        }
        None
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let wordgrid = WordGrid::try_from(input).unwrap();
    let mut count = 0;

    for (p1, _) in wordgrid.character_iter(XMAS::X) {
        for pd in Point2Direction::all_with_diagonals() {
            if wordgrid.is_word_into_direction(&p1, &pd, XMAS_WORD) {
                count += 1;
            }
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let wordgrid = WordGrid::try_from(input).unwrap();
    Some(wordgrid.x_mas_iter().map(|(_, c)| c).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
