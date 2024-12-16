use advent_of_code::algebra_helpers::{Point2, Point2Direction, PointGrid, PointGridIterator};

advent_of_code::solution!(4);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Xmas {
    X,
    M,
    A,
    S,
}

const XMAS_WORD: &[Xmas] = &[Xmas::X, Xmas::M, Xmas::A, Xmas::S];
const MAS_WORD: &[Xmas] = &[Xmas::M, Xmas::A, Xmas::S];

struct WordGrid(PointGrid<isize, 2, Xmas>);

impl TryFrom<&str> for WordGrid {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut grid = PointGrid::default();
        for (y, row) in value.trim().lines().enumerate() {
            for (x, c) in row.chars().enumerate() {
                grid.insert(
                    Point2::new(x as isize, y as isize),
                    match c {
                        'X' => Xmas::X,
                        'M' => Xmas::M,
                        'A' => Xmas::A,
                        'S' => Xmas::S,
                        _ => return Err(()),
                    },
                );
            }
        }

        Ok(Self(grid))
    }
}

impl WordGrid {
    fn is_word_into_direction(
        &self,
        position: &Point2<isize>,
        direction: &Point2Direction,
        word: &[Xmas],
    ) -> bool {
        for (i, c) in word.iter().enumerate() {
            let current_position = position.get_point_in_direction(direction, i as isize);
            if self.0.get(&current_position) != Some(c) {
                return false;
            }
        }
        true
    }

    fn character_iter(&self, character_to_filter: Xmas) -> CharacterGridIterator {
        CharacterGridIterator {
            grid: self,
            character_to_filter,
            iterator: self.0.iter_full_bounds(),
        }
    }

    fn x_mas_iter(&self) -> XMASIterator {
        XMASIterator {
            grid: self,
            iterator: self.character_iter(Xmas::M),
        }
    }
}

struct CharacterGridIterator<'a> {
    grid: &'a WordGrid,
    character_to_filter: Xmas,
    iterator: PointGridIterator<isize, 2>,
}

impl Iterator for CharacterGridIterator<'_> {
    type Item = (Point2<isize>, Xmas);

    fn next(&mut self) -> Option<Self::Item> {
        for p in self.iterator.by_ref() {
            if self.grid.0.get(&p) == Some(&self.character_to_filter) {
                return Some((p, self.character_to_filter));
            }
        }
        None
    }
}

struct XMASIterator<'a> {
    grid: &'a WordGrid,
    iterator: CharacterGridIterator<'a>,
}

impl Iterator for XMASIterator<'_> {
    type Item = (Point2<isize>, u32);

    fn next(&mut self) -> Option<Self::Item> {
        for (p, _) in self.iterator.by_ref() {
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

    for (p1, _) in wordgrid.character_iter(Xmas::X) {
        for pd in Point2Direction::all_with_diagonals() {
            if wordgrid.is_word_into_direction(&p1, pd, XMAS_WORD) {
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
