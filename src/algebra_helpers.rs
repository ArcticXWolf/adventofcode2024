use itertools::Itertools;
use num_traits::Num;
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::{self, Index, IndexMut};
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};
use std::slice::Iter;
use std::{array, fmt};

// Taken and adapted from MIT-licensed code library lina: https://github.com/LukasKalbertodt/lina

pub trait Scalar:
    Num
    + Clone
    + Copy
    + fmt::Debug
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
    + PartialEq
    + Eq
    + PartialOrd
    + Ord
    + Hash
{
}

impl<T> Scalar for T where
    T: Num
        + Clone
        + Copy
        + fmt::Debug
        + AddAssign
        + SubAssign
        + MulAssign
        + DivAssign
        + PartialEq
        + Eq
        + PartialOrd
        + Ord
        + Hash
{
}

pub trait Float: Scalar + num_traits::Float + num_traits::FloatConst {}

impl<T> Float for T where T: Scalar + num_traits::Float + num_traits::FloatConst {}

#[repr(transparent)]
pub struct Point<T: Scalar, const N: usize>(pub [T; N]);

pub type Point2<T> = Point<T, 2>;

impl<T: Scalar> Point<T, 2> {
    pub fn new(x: T, y: T) -> Self {
        Self([x, y])
    }

    pub fn get_point_in_direction(&self, direction: &Point2Direction, distance: T) -> Self {
        match direction {
            Point2Direction::North => Self::new(self.0[0], self.0[1] - distance),
            Point2Direction::NorthEast => Self::new(self.0[0] + distance, self.0[1] - distance),
            Point2Direction::East => Self::new(self.0[0] + distance, self.0[1]),
            Point2Direction::SouthEast => Self::new(self.0[0] + distance, self.0[1] + distance),
            Point2Direction::South => Self::new(self.0[0], self.0[1] + distance),
            Point2Direction::SouthWest => Self::new(self.0[0] - distance, self.0[1] + distance),
            Point2Direction::West => Self::new(self.0[0] - distance, self.0[1]),
            Point2Direction::NorthWest => Self::new(self.0[0] - distance, self.0[1] - distance),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Point2Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl<T: Scalar> TryFrom<(Point2<T>, Point2<T>)> for Point2Direction
where
    T: std::ops::Neg<Output = T>,
{
    type Error = String;

    fn try_from(value: (Point2<T>, Point2<T>)) -> Result<Self, Self::Error> {
        match value.1 - value.0 {
            Point::<T, 2>([x, y]) if x == T::zero() && y == -T::one() => Ok(Self::North),
            Point::<T, 2>([x, y]) if x == T::one() && y == -T::one() => Ok(Self::NorthEast),
            Point::<T, 2>([x, y]) if x == T::one() && y == T::zero() => Ok(Self::East),
            Point::<T, 2>([x, y]) if x == T::one() && y == T::one() => Ok(Self::SouthEast),
            Point::<T, 2>([x, y]) if x == T::zero() && y == T::one() => Ok(Self::South),
            Point::<T, 2>([x, y]) if x == -T::one() && y == T::one() => Ok(Self::SouthWest),
            Point::<T, 2>([x, y]) if x == -T::one() && y == T::zero() => Ok(Self::West),
            Point::<T, 2>([x, y]) if x == -T::one() && y == -T::one() => Ok(Self::NorthWest),
            _ => Err(format!(
                "Could not find direction from {} to {}",
                value.0, value.1
            )),
        }
    }
}

impl Point2Direction {
    pub fn all_with_diagonals() -> Iter<'static, Self> {
        static D: [Point2Direction; 8] = [
            Point2Direction::North,
            Point2Direction::NorthEast,
            Point2Direction::East,
            Point2Direction::SouthEast,
            Point2Direction::South,
            Point2Direction::SouthWest,
            Point2Direction::West,
            Point2Direction::NorthWest,
        ];

        D.iter()
    }

    pub fn all() -> Iter<'static, Self> {
        static D: [Point2Direction; 4] = [
            Point2Direction::North,
            Point2Direction::East,
            Point2Direction::South,
            Point2Direction::West,
        ];

        D.iter()
    }

    pub fn direction_left(&self) -> Self {
        match self {
            Self::North => Self::West,
            Self::East => Self::North,
            Self::South => Self::East,
            Self::West => Self::South,
            _ => unimplemented!(),
        }
    }

    pub fn direction_right(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
            _ => unimplemented!(),
        }
    }

    pub fn direction_flip(&self) -> Self {
        match self {
            Self::North => Self::South,
            Self::East => Self::West,
            Self::South => Self::North,
            Self::West => Self::East,
            Self::NorthEast => Self::SouthWest,
            Self::SouthEast => Self::NorthWest,
            Self::NorthWest => Self::SouthEast,
            Self::SouthWest => Self::NorthEast,
        }
    }
}

impl std::fmt::Display for Point2Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Point2Direction::North => write!(f, "↑"),
            Point2Direction::East => write!(f, "→"),
            Point2Direction::South => write!(f, "↓"),
            Point2Direction::West => write!(f, "←"),
            Point2Direction::NorthEast => write!(f, "↗"),
            Point2Direction::SouthEast => write!(f, "↘"),
            Point2Direction::SouthWest => write!(f, "↙"),
            Point2Direction::NorthWest => write!(f, "↖"),
        }
    }
}

pub type Point3<T> = Point<T, 3>;

impl<T: Scalar> Point<T, 3> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self([x, y, z])
    }

    pub fn cross(self, other: Self) -> Self {
        Self::new(
            self.0[1] * other.0[2] - self.0[2] * other.0[1],
            self.0[2] * other.0[0] - self.0[0] * other.0[2],
            self.0[0] * other.0[1] - self.0[1] * other.0[0],
        )
    }
}

pub type Point4<T> = Point<T, 4>;

impl<T: Scalar> Point<T, 4> {
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Self([x, y, z, w])
    }
}

impl<T: Scalar, const N: usize> Point<T, N> {
    pub fn zero() -> Self {
        std::array::from_fn(|_| T::zero()).into()
    }

    pub fn is_zero(&self) -> bool {
        self.0.iter().all(T::is_zero)
    }

    pub fn one() -> Self {
        std::array::from_fn(|_| T::one()).into()
    }

    pub fn is_one(&self) -> bool {
        self.0.iter().all(T::is_one)
    }

    pub fn origin() -> Self {
        Self::zero()
    }

    pub fn filled(value: T) -> Self {
        std::array::from_fn(|_| value).into()
    }

    pub fn unit_in_dimension(dimension: usize) -> Self {
        std::array::from_fn(|i| if i == dimension { T::one() } else { T::zero() }).into()
    }

    pub fn unit_vectors() -> Vec<Self> {
        (0..N).map(|i| Self::unit_in_dimension(i)).collect()
    }

    pub fn directions() -> Vec<Self> {
        Self::unit_vectors()
            .into_iter()
            .map(|p| Point::zero() - p)
            .chain(Self::unit_vectors())
            .collect()
    }

    pub fn directions_with_diagonals() -> Vec<Self> {
        let mut current_vectors = vec![Self::zero()];

        for i in 0..N {
            current_vectors = current_vectors
                .iter()
                .flat_map(|v| {
                    vec![
                        *v - Self::unit_in_dimension(i),
                        *v,
                        *v + Self::unit_in_dimension(i),
                    ]
                })
                .collect();
        }

        current_vectors.remove(usize::pow(3, N as u32) / 2); // remove identity

        current_vectors
    }

    pub fn distance_euclid_squared_from(self, other: Self) -> T {
        (self - other).length_euclid_squared()
    }

    pub fn distance_euclid_from(self, other: Self) -> T
    where
        T: Float,
    {
        (self - other).length_euclid()
    }

    pub fn distance_manhattan_from(self, other: Self) -> T {
        (self - other).length_manhattan()
    }

    pub fn length_euclid_squared(&self) -> T {
        self.0
            .iter()
            .map(|&c| c * c)
            .fold(T::zero(), |acc, e| acc + e)
    }

    pub fn length_euclid(&self) -> T
    where
        T: Float,
    {
        self.length_euclid_squared().sqrt()
    }

    pub fn length_manhattan(&self) -> T {
        self.0.iter().fold(T::zero(), |acc, e| acc + *e)
    }

    pub fn dot(self, other: Self) -> T {
        (0..N)
            .map(|d| self.0[d] * other.0[d])
            .fold(T::zero(), |acc, e| acc + e)
    }

    pub fn min_componentwise(self, other: Self) -> Self {
        std::array::from_fn(|i| self.0[i].min(other.0[i])).into()
    }

    pub fn max_componentwise(self, other: Self) -> Self {
        std::array::from_fn(|i| self.0[i].max(other.0[i])).into()
    }

    pub fn vec_to(self, other: Self) -> Point<T, N> {
        other - self
    }
}

impl<T: Scalar, const N: usize> From<Point<T, N>> for [T; N] {
    fn from(value: Point<T, N>) -> Self {
        value.0
    }
}

impl<T: Scalar, const N: usize> From<[T; N]> for Point<T, N> {
    fn from(value: [T; N]) -> Self {
        Self(value)
    }
}

impl<T: Scalar, const N: usize> Index<usize> for Point<T, N> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T: Scalar, const N: usize> IndexMut<usize> for Point<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<T: Scalar, const N: usize> fmt::Debug for Point<T, N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point")?;
        write!(f, "[")?;
        for (i, e) in self.0.into_iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            e.fmt(f)?;
        }
        write!(f, "]")
    }
}

impl<T: Scalar, const N: usize> fmt::Display for Point<T, N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point")?;
        write!(f, "[")?;
        for (i, e) in self.0.into_iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            e.fmt(f)?;
        }
        write!(f, "]")
    }
}

impl<T: Scalar, const N: usize> ops::Add<Point<T, N>> for Point<T, N> {
    type Output = Self;
    fn add(self, rhs: Point<T, N>) -> Self::Output {
        array::from_fn(|i| self[i] + rhs[i]).into()
    }
}

impl<T: Scalar, const N: usize> ops::AddAssign<Point<T, N>> for Point<T, N> {
    fn add_assign(&mut self, rhs: Point<T, N>) {
        for (lhs, rhs) in IntoIterator::into_iter(&mut self.0).zip(rhs.0) {
            *lhs += rhs;
        }
    }
}

impl<T: Scalar, const N: usize> ops::SubAssign<Point<T, N>> for Point<T, N> {
    fn sub_assign(&mut self, rhs: Point<T, N>) {
        for (lhs, rhs) in IntoIterator::into_iter(&mut self.0).zip(rhs.0) {
            *lhs -= rhs;
        }
    }
}

impl<T: Scalar, const N: usize> ops::Sub<Self> for Point<T, N> {
    type Output = Point<T, N>;
    fn sub(self, rhs: Self) -> Self::Output {
        array::from_fn(|i| self[i] - rhs[i]).into()
    }
}

impl<T: Scalar, const N: usize> ops::Mul<T> for Point<T, N> {
    type Output = Point<T, N>;
    fn mul(self, rhs: T) -> Self::Output {
        array::from_fn(|i| self[i] * rhs).into()
    }
}

impl<T: Scalar + std::hash::Hash, const N: usize> std::hash::Hash for Point<T, N> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}

impl<T: Scalar, const N: usize> PartialEq for Point<T, N> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}
impl<T: Scalar + Eq, const N: usize> Eq for Point<T, N> {}

impl<T: Scalar, const N: usize> Clone for Point<T, N> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: Scalar, const N: usize> Copy for Point<T, N> {}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PointRange<T: Scalar, const N: usize> {
    pub min: Point<T, N>,
    pub max: Point<T, N>, // exclusive
}

pub type Rectangle<T> = PointRange<T, 2>;
pub type Cube<T> = PointRange<T, 3>;
pub type Hypercube<T> = PointRange<T, 4>;

impl<T: Scalar, const N: usize> Default for PointRange<T, N> {
    fn default() -> Self {
        Self {
            min: Point::zero(),
            max: Point::one(),
        }
    }
}

impl<T: Scalar, const N: usize> fmt::Debug for PointRange<T, N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PointRange")?;
        write!(f, "[(")?;
        for (i, e) in self.min.0.into_iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            e.fmt(f)?;
        }
        write!(f, ")->(")?;
        for (i, e) in self.max.0.into_iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            e.fmt(f)?;
        }
        write!(f, ")]")
    }
}

impl<T: Scalar, const N: usize> fmt::Display for PointRange<T, N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PointRange")?;
        write!(f, "[(")?;
        for (i, e) in self.min.0.into_iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            e.fmt(f)?;
        }
        write!(f, ")->(")?;
        for (i, e) in self.max.0.into_iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            e.fmt(f)?;
        }
        write!(f, ")]")
    }
}

impl<T: Scalar, const N: usize> PointRange<T, N> {
    pub fn new(point1: Point<T, N>, point2: Point<T, N>) -> Self {
        let min = point1.min_componentwise(point2);
        let max = point1.max_componentwise(point2);
        Self { min, max }
    }

    pub fn contains(&self, point: &Point<T, N>) -> bool {
        (0..N).all(|i| self.min.0[i] <= point.0[i] && self.max.0[i] > point.0[i])
    }

    pub fn intersects(&self, other: &Self) -> bool {
        !((0..N).any(|i| self.min.0[i] >= other.max.0[i] || other.min.0[i] >= self.max.0[i]))
    }
}

impl<T: Scalar, const N: usize> ops::Add<Point<T, N>> for PointRange<T, N> {
    type Output = Self;
    fn add(self, rhs: Point<T, N>) -> Self::Output {
        PointRange::new(self.min + rhs, self.max + rhs)
    }
}

impl<T: Scalar, const N: usize> ops::AddAssign<Point<T, N>> for PointRange<T, N> {
    fn add_assign(&mut self, rhs: Point<T, N>) {
        self.min += rhs;
        self.max += rhs;
    }
}

impl<T: Scalar, const N: usize> ops::SubAssign<Point<T, N>> for PointRange<T, N> {
    fn sub_assign(&mut self, rhs: Point<T, N>) {
        self.min -= rhs;
        self.max -= rhs;
    }
}

impl<T: Scalar, const N: usize> ops::Sub<Point<T, N>> for PointRange<T, N> {
    type Output = Self;
    fn sub(self, rhs: Point<T, N>) -> Self::Output {
        PointRange::new(self.min - rhs, self.max - rhs)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PointGrid<T: Scalar, const N: usize, U>(pub HashMap<Point<T, N>, U>);

impl<T: Scalar, const N: usize, U> Default for PointGrid<T, N, U> {
    fn default() -> Self {
        Self(HashMap::new())
    }
}

impl<T: Scalar + Ord, const N: usize, U> PointGrid<T, N, U> {
    pub fn iter_full_bounds(&self) -> PointGridIterator<T, N> {
        let (min, max) = self.dimensions();
        PointGridIterator::new(min, max + Point::filled(T::one()))
    }
}

impl<T: Scalar + std::hash::Hash + Eq, const N: usize, U> PointGrid<T, N, U> {
    pub fn insert(&mut self, p: Point<T, N>, value: U) {
        self.0.insert(p, value);
    }

    pub fn get(&self, p: &Point<T, N>) -> Option<&U> {
        self.0.get(p)
    }
}

impl<T: Scalar + Ord, const N: usize, U> PointGrid<T, N, U> {
    pub fn dimensions(&self) -> (Point<T, N>, Point<T, N>) {
        (
            Point(
                (0..N)
                    .map(|n| self.0.keys().map(|p| *p.0.get(n).unwrap()).min().unwrap())
                    .collect_vec()
                    .try_into()
                    .unwrap(),
            ),
            Point(
                (0..N)
                    .map(|n| self.0.keys().map(|p| *p.0.get(n).unwrap()).max().unwrap())
                    .collect_vec()
                    .try_into()
                    .unwrap(),
            ),
        )
    }
}

impl<T: Scalar + Ord + std::iter::Step + std::hash::Hash, U: fmt::Display> fmt::Display
    for PointGrid<T, 2, U>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (min, max) = self.dimensions();
        writeln!(f, "Grid ({}, {}):", min, max)?;
        for y in min.0[1]..(max.0[1] + T::one()) {
            for x in min.0[0]..(max.0[0] + T::one()) {
                if let Some(u) = self.get(&Point2::new(x, y)) {
                    write!(f, "{}", u)?;
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f)?;
        }
        write!(f, "")
    }
}

impl<T: Scalar + Ord + std::iter::Step + std::hash::Hash, U: fmt::Display> fmt::Display
    for PointGrid<T, 3, U>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (min, max) = self.dimensions();
        writeln!(f, "Grid ({}, {}):", min, max)?;
        for z in min.0[2]..(max.0[2] + T::one()) {
            for y in min.0[1]..(max.0[1] + T::one()) {
                for x in min.0[0]..(max.0[0] + T::one()) {
                    if let Some(u) = self.get(&Point3::new(x, y, z)) {
                        write!(f, "{}", u)?;
                    } else {
                        write!(f, " ")?;
                    }
                }
                writeln!(f)?;
            }
        }
        write!(f, "")
    }
}

impl<T: Scalar + Ord + std::iter::Step + std::hash::Hash, U: fmt::Display> fmt::Display
    for PointGrid<T, 4, U>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (min, max) = self.dimensions();
        writeln!(f, "Grid ({}, {}):", min, max)?;
        for w in min.0[3]..(max.0[3] + T::one()) {
            for z in min.0[2]..(max.0[2] + T::one()) {
                for y in min.0[1]..(max.0[1] + T::one()) {
                    for x in min.0[0]..(max.0[0] + T::one()) {
                        if let Some(u) = self.get(&Point4::new(x, y, z, w)) {
                            write!(f, "{}", u)?;
                        } else {
                            write!(f, " ")?;
                        }
                    }
                    writeln!(f)?;
                }
            }
        }
        write!(f, "")
    }
}

pub struct PointGridIterator<T: Scalar, const N: usize> {
    lower_bound: Point<T, N>,
    upper_bound: Point<T, N>,
    last: Point<T, N>,
}

impl<T: Scalar + PartialOrd, const N: usize> PointGridIterator<T, N> {
    pub fn new(lower_bound: Point<T, N>, upper_bound: Point<T, N>) -> Self {
        Self {
            lower_bound,
            upper_bound,
            last: lower_bound,
        }
    }
}

impl<T: Scalar + PartialOrd, const N: usize> Iterator for PointGridIterator<T, N> {
    type Item = Point<T, N>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.last == self.upper_bound {
            return None;
        }

        let result = self.last;
        for n in (0..N).rev() {
            if self.last.0[n] + T::one() >= self.upper_bound.0[n] {
                self.last.0[n] = self.lower_bound.0[n];
            } else {
                self.last.0[n] += T::one();
                return Some(result);
            }
        }
        self.last = self.upper_bound;
        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_directions() {
        assert_eq!(
            Point2::<isize>::directions(),
            vec![
                Point2::new(-1, 0),
                Point2::new(0, -1),
                Point2::new(1, 0),
                Point2::new(0, 1)
            ]
        );
        assert_eq!(
            Point3::<isize>::directions(),
            vec![
                Point3::new(-1, 0, 0),
                Point3::new(0, -1, 0),
                Point3::new(0, 0, -1),
                Point3::new(1, 0, 0),
                Point3::new(0, 1, 0),
                Point3::new(0, 0, 1)
            ]
        );
    }

    #[test]
    fn test_directions_with_diagonals() {
        assert_eq!(
            Point2::<isize>::directions_with_diagonals(),
            vec![
                Point2::new(-1, -1),
                Point2::new(-1, 0),
                Point2::new(-1, 1),
                Point2::new(0, -1),
                Point2::new(0, 1),
                Point2::new(1, -1),
                Point2::new(1, 0),
                Point2::new(1, 1),
            ]
        );
        assert_eq!(
            Point3::<isize>::directions_with_diagonals(),
            vec![
                Point3::new(-1, -1, -1),
                Point3::new(-1, -1, 0),
                Point3::new(-1, -1, 1),
                Point3::new(-1, 0, -1),
                Point3::new(-1, 0, 0),
                Point3::new(-1, 0, 1),
                Point3::new(-1, 1, -1),
                Point3::new(-1, 1, 0),
                Point3::new(-1, 1, 1),
                Point3::new(0, -1, -1),
                Point3::new(0, -1, 0),
                Point3::new(0, -1, 1),
                Point3::new(0, 0, -1),
                Point3::new(0, 0, 1),
                Point3::new(0, 1, -1),
                Point3::new(0, 1, 0),
                Point3::new(0, 1, 1),
                Point3::new(1, -1, -1),
                Point3::new(1, -1, 0),
                Point3::new(1, -1, 1),
                Point3::new(1, 0, -1),
                Point3::new(1, 0, 0),
                Point3::new(1, 0, 1),
                Point3::new(1, 1, -1),
                Point3::new(1, 1, 0),
                Point3::new(1, 1, 1),
            ]
        );
    }

    #[test]
    fn test_point_grid_dimensions() {
        let mut pg: PointGrid<isize, 2, bool> = PointGrid::default();
        pg.insert(Point2::new(0, 0), true);
        pg.insert(Point2::new(-20, 20), true);
        pg.insert(Point2::new(20, -10), true);
        assert_eq!(
            pg.dimensions(),
            (Point2::new(-20, -10), Point2::new(20, 20))
        );
    }

    #[test]
    fn test_point_grid_iterator() {
        let pgi: PointGridIterator<isize, 2> =
            PointGridIterator::new(Point2::new(-2, -1), Point2::new(2, 3));
        assert_eq!(
            pgi.collect_vec(),
            vec![
                Point2::new(-2, -1),
                Point2::new(-2, 0),
                Point2::new(-2, 1),
                Point2::new(-2, 2),
                Point2::new(-1, -1),
                Point2::new(-1, 0),
                Point2::new(-1, 1),
                Point2::new(-1, 2),
                Point2::new(0, -1),
                Point2::new(0, 0),
                Point2::new(0, 1),
                Point2::new(0, 2),
                Point2::new(1, -1),
                Point2::new(1, 0),
                Point2::new(1, 1),
                Point2::new(1, 2),
            ]
        );
    }

    #[test]
    fn test_point_ranges() {
        let rect1: Rectangle<isize> = Rectangle::new(Point2::new(0, 0), Point2::new(10, 10));
        let rect2: Rectangle<isize> = Rectangle::new(Point2::new(5, 5), Point2::new(15, 15));
        let rect3: Rectangle<isize> = Rectangle::new(Point2::new(10, 10), Point2::new(20, 20));

        assert!(rect1.intersects(&rect2));
        assert!(rect2.intersects(&rect3));
        assert!(!rect1.intersects(&rect3));
        assert!(!rect3.intersects(&rect1));

        assert!(rect1.contains(&Point2::new(0, 0)));
        assert!(rect1.contains(&Point2::new(5, 5)));
        assert!(!rect1.contains(&Point2::new(10, 10)));

        let cube: Cube<isize> = Cube::new(Point3::new(1, 0, 1), Point3::new(2, 3, 2));
        assert!(cube.contains(&Point3::new(1, 0, 1)));
        assert!(cube.contains(&Point3::new(1, 1, 1)));
        assert!(cube.contains(&Point3::new(1, 2, 1)));
        assert!(!cube.contains(&Point3::new(1, 3, 1)));
    }
}
