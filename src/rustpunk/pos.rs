use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Sub;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

impl Pos {
    pub fn new(x: i32, y: i32) -> Pos {
        Pos {x, y}
    }

    pub fn zero() -> Pos {
        Pos {x: 0, y: 0}
    }

    pub fn tup(self) -> (i32, i32) {
        (self.x, self.y)
    }
}

impl Add for Pos {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Pos {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl AddAssign for Pos {
    fn add_assign(&mut self, other: Self) {
        self.x = other.x;
        self.y = other.y;
    }
}
