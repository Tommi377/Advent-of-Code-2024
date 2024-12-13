use num::{one, Integer};
use std::ops::{Add, Mul, Sub};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: PartialOrd> Point<T> {
    pub fn in_bound(self, lower: T, upper: T) -> bool {
        self.x >= lower && self.x < upper && self.y >= lower && self.y < upper
    }
}

impl<T: Integer + Copy> Point<T> {
    pub fn right(&self) -> Self {
        Point {
            x: self.x + one(),
            y: self.y,
        }
    }
    pub fn left(&self) -> Self {
        Point {
            x: self.x - one(),
            y: self.y,
        }
    }
    pub fn up(&self) -> Self {
        Point {
            x: self.x,
            y: self.y - one(),
        }
    }
    pub fn down(&self) -> Self {
        Point {
            x: self.x,
            y: self.y + one(),
        }
    }
    pub fn up_right(&self) -> Self {
        Point {
            x: self.x + one(),
            y: self.y - one(),
        }
    }
    pub fn up_left(&self) -> Self {
        Point {
            x: self.x - one(),
            y: self.y - one(),
        }
    }
    pub fn down_right(&self) -> Self {
        Point {
            x: self.x + one(),
            y: self.y + one(),
        }
    }
    pub fn down_left(&self) -> Self {
        Point {
            x: self.x - one(),
            y: self.y + one(),
        }
    }

    pub fn neighbors(&self) -> Vec<Self> {
        vec![self.right(), self.left(), self.up(), self.down()]
    }
}

impl<T: Add<Output = T>> Add for Point<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Point<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T: Integer + Copy> Mul<T> for Point<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
