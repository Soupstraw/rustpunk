use num::signum;
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

    pub fn to_dir(self) -> Option<Dir> {
        match self.tup() {
            ( 0, -1) => Some(Dir::N),
            ( 0,  1) => Some(Dir::S),
            (-1,  0) => Some(Dir::W),
            ( 1,  0) => Some(Dir::E),
            ( 1, -1) => Some(Dir::NE),
            (-1, -1) => Some(Dir::NW),
            ( 1,  1) => Some(Dir::SE),
            (-1,  1) => Some(Dir::SW),
            _        => None,
        }
    }

    pub fn dir_towards(self, target: Pos) -> Option<Dir> {
        let delta = target - self;
        let x = signum(delta.x);
        let y = signum(delta.y);
        Pos::new(x, y).to_dir()
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

#[derive(Clone, Copy, Debug)]
pub enum Dir {
    N, E, S, W,
    NE, SE, SW, NW,
}

impl Dir {
    pub fn to_pos(self) -> Pos {
        match self {
            Dir::N => Pos::new( 0, -1),
            Dir::S => Pos::new( 0,  1),
            Dir::W => Pos::new(-1,  0),
            Dir::E => Pos::new( 1,  0),
            Dir::NE => Pos::new( 1, -1),
            Dir::NW => Pos::new(-1, -1),
            Dir::SE => Pos::new( 1,  1),
            Dir::SW => Pos::new(-1,  1),
        }
    }
}

