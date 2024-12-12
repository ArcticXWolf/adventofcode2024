use std::fmt::Display;

use advent_of_code::algebra_helpers::{Point2, Point2Direction, PointGrid};

advent_of_code::solution!(12);

const EDGE_MASKS: &[(u8, u8)] = &[
    // (EDGE_MASK, EDGE_DETECTOR)
    // Convex edges
    (0b11100000, 0b00000000),
    (0b00111000, 0b00000000),
    (0b00001110, 0b00000000),
    (0b10000011, 0b00000000),
    // Concave edges
    (0b11100000, 0b10100000),
    (0b00111000, 0b00101000),
    (0b00001110, 0b00001010),
    (0b10000011, 0b10000010),
    // Touching/diagonal edges
    (0b11100000, 0b01000000),
    (0b00111000, 0b00010000),
    (0b00001110, 0b00000100),
    (0b10000011, 0b00000001),
];

#[derive(Debug, Default)]
struct Region {
    plots: PointGrid<isize, 2, char>,
    perimeter: usize,
    edges: usize,
    plant_type: char,
}

impl Region {
    fn from_grid_and_position(
        grid: &PointGrid<isize, 2, char>,
        starting_position: &Point2<isize>,
        region_type: char,
    ) -> Self {
        let mut region = Region::default();
        let mut perimeter = 0;
        let mut edges = 0;
        let mut queue = vec![(*starting_position, region_type)];

        // flood fill region
        while let Some((current_pos, current_type)) = queue.pop() {
            region.plots.insert(current_pos, current_type);

            for pd in Point2Direction::all() {
                let new_pos = current_pos.get_point_in_direction(pd, 1);

                if !region.plots.0.contains_key(&new_pos)
                    && !queue.contains(&(new_pos, region_type))
                {
                    if let Some(&new_type) = grid.get(&new_pos) {
                        if new_type == region_type {
                            queue.push((new_pos, new_type));
                            continue;
                        }
                    }
                    // if a direction has no valid neighbor, then it is a unit of perimeter
                    perimeter += 1;
                }
            }
        }

        // detect edges by constructing a neighbor bitmap and comparing it to our edge bitmaps
        for (current_pos, _) in region.plots.0.iter() {
            let neigbor_bitmap: u8 = Point2Direction::all_with_diagonals()
                .map(|pd| current_pos.get_point_in_direction(pd, 1))
                .enumerate()
                .map(|(i, p)| match region.plots.get(&p) {
                    Some(_) => 1 << (7 - i),
                    None => 0,
                })
                .sum();
            for (edge_mask, edge_detector) in EDGE_MASKS {
                if neigbor_bitmap & edge_mask == *edge_detector {
                    edges += 1;
                }
            }
        }

        region.perimeter = perimeter;
        region.edges = edges;
        region.plant_type = region_type;

        region
    }

    fn area(&self) -> usize {
        self.plots.0.len()
    }

    fn fence_cost(&self) -> usize {
        self.area() * self.perimeter
    }

    fn fence_bulk_cost(&self) -> usize {
        self.area() * self.edges
    }
}

impl Display for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Region {} (Perimeter {}, Edges {}, Area {})\n{}",
            self.plant_type,
            self.perimeter,
            self.edges,
            self.area(),
            self.plots
        )
    }
}

#[derive(Debug, Default)]
struct Garden {
    plots: PointGrid<isize, 2, char>,
    regions: Vec<Region>,
}

impl From<&str> for Garden {
    fn from(value: &str) -> Self {
        let mut garden = Self::default();

        for (y, row) in value.trim().lines().enumerate() {
            for (x, c) in row.chars().enumerate() {
                let position = Point2::new(x as isize, y as isize);
                garden.plots.insert(position, c);
            }
        }

        garden
    }
}

impl Garden {
    fn find_regions(&mut self) {
        for (pos, region_type) in self.plots.0.iter() {
            if !self.regions.iter().any(|r| r.plots.0.contains_key(&pos)) {
                let region = Region::from_grid_and_position(&self.plots, &pos, *region_type);
                self.regions.push(region);
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut garden = Garden::from(input);
    garden.find_regions();
    Some(garden.regions.iter().map(|r| r.fence_cost()).sum::<usize>() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut garden = Garden::from(input);
    garden.find_regions();
    Some(
        garden
            .regions
            .iter()
            .map(|r| r.fence_bulk_cost())
            .sum::<usize>() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(140));
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(772));
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 5,
        ));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(80));
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(436));
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(236));
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(368));
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 5,
        ));
        assert_eq!(result, Some(1206));
    }
}
