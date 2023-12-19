use std::cmp::{max, min};
use std::ops::{Add, Div, Mul, Sub};
use crate::Direction::{*};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Point2d<T>
{
    pub x: T,
    pub y: T
}

impl<T> Point2d<T> {
    pub fn new(x: T, y: T) -> Self {
        Self {x, y}
    }
}

impl<T> Add for Point2d<T>
where T: Add<Output = T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {x: self.x + other.x, y: self.y + other.y}
    }
}

impl<T> Sub for Point2d<T>
where T: Sub<Output = T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {x: self.x - other.x, y: self.y - other.y}
    }
}

impl<T> Mul<T> for Point2d<T>
where T: Mul<Output = T> + Copy {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {x: self.x * rhs, y: self.y * rhs}
    }
}

impl<T> Div<T> for Point2d<T>
where T: Div<Output = T> + Copy {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self {x: self.x / rhs, y: self.y / rhs}
    }
}

pub fn corners<'a, T,C>(mut points: C) -> Option<(Point2d<T>, Point2d<T>)>
where
    C: Iterator<Item = &'a Point2d<T>>,
    T: Ord + Copy + 'a
{
    let first = points.next()?;

    let mut min_x = first.x;
    let mut max_x = first.x;
    let mut min_y = first.y;
    let mut max_y = first.y;
    for point in points {
        min_x = min(point.x, min_x);
        max_x = max(point.x, max_x);
        min_y = min(point.y, min_y);
        max_y = max(point.y, max_y);
    }

    Some((Point2d::new(min_x, min_y), Point2d::new(max_x, max_y)))
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Direction {
    Up, Down, Left, Right
}

pub fn dir_delta(direction: Direction) -> Point2d<i32> {
    match direction {
        Up => Point2d::new(0, -1),
        Down => Point2d::new(0, 1),
        Left => Point2d::new(-1, 0),
        Right => Point2d::new(1, 0),
    }
}

pub fn dir_opposite(direction: Direction) -> Direction {
    match direction {
        Up => Down,
        Down => Up,
        Left => Right,
        Right => Left,
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let a = Point2d::new(1 as i32, 2 as i32);
        let b = Point2d::new(3 as i32, 4 as i32);
        let c = a + b;
        assert_eq!(c.x, 4);
        assert_eq!(c.y, 6);
    }

    #[test]
    fn sub() {
        let a = Point2d::new(1 as i32, 2 as i32);
        let b = Point2d::new(3 as i32, 5 as i32);
        let c = b - a;
        assert_eq!(c.x, 2);
        assert_eq!(c.y, 3);
    }

    #[test]
    fn mul() {
        let a = Point2d::new(1, 2);
        let b = a * 3;
        assert_eq!(b.x, 3);
        assert_eq!(b.y, 6);
    }

    #[test]
    fn div() {
        let a = Point2d::new(9, 12);
        let b = a / 3;
        assert_eq!(b.x, 3);
        assert_eq!(b.y, 4);
    }

    #[test]
    fn corner() {
        let points = vec![
            Point2d::new(-1, 100),
            Point2d::new(1000, 17),
            Point2d::new(23, -300),
            Point2d::new(88, 2222),
        ];
        match corners(points.iter()) {
            None => panic!("Didn't get the corners"),
            Some((top_left, bottom_right)) => {
                assert_eq!(top_left, Point2d::new(-1, -300));
                assert_eq!(bottom_right, Point2d::new(1000, 2222))
            }
        };
    }
}
