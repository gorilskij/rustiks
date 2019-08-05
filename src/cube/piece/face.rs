use crate::cube::transpose::{Transpose, Projection};
use std::fmt::{Display, Formatter, Error, Debug};
use std::ops::{Add, Sub};
use crate::cube::piece::{edge::Edge, corner::Corner};

// Face is fully qualified to work when exported
#[macro_export]
macro_rules! face {
    ($v: expr) => { crate::cube::piece::face::Face::from($v) }
}

// convert a list of ints to an array of faces
macro_rules! to_faces {
    [$($num: expr),*] => {
        [$(Face::new($num),)*]
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Face(u8);

impl<T> From<T> for Face where u8: From<T> {
    fn from(value: T) -> Self {
        Self::new(value.into())
    }
}

impl Add<u8> for Face {
    type Output = Self;

    fn add(self, other: u8) -> Self::Output {
        Self((self.0 + other) % 6)
    }
}

impl Add for Face {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        self + other.0
    }
}

impl Sub<u8> for Face {
    type Output = Self;

    fn sub(self, other: u8) -> Self::Output {
        let mut self_0 = self.0;
        while other > self_0 { self_0 += 6; }
        Self((self_0 - other) % 6)
    }
}

impl Sub for Face {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        self - other.0
    }
}

impl Face {
    pub const fn new(value: u8) -> Self {
        // commented due to assert not working with const fn
        // TODO: find a way to still check
//        assert!(value <= 5);
        Self(value)
    }

    pub fn is_even(self) -> bool {
        self.0 % 2 == 0
    }

    pub fn opposite(self) -> Self {
        self + 3
    }

    pub fn adjacent(self) -> [Face; 4] {
        let mut adjacent = [self - 2, self - 1, self + 1, self + 2];
        adjacent.sort();
        adjacent
    }

    pub fn adjacent_clockwise(self) -> [Face; 4] {
        let mut adjacent = self.adjacent();
        if self.is_even() { adjacent.reverse() }
        adjacent
    }

    pub fn adjacent_edges(self) -> [Edge; 4] {
        let adjacent = self.adjacent();
        array_collect!(
            adjacent.iter().map(|f| edge![*f, self]),
            [Edge; 4]
        )
    }

    pub fn adjacent_corners(self) -> [Corner; 4] {
        let adjacent = self.adjacent();
        array_collect!(
            (0..4).map(|i| corner![self, adjacent[i], adjacent[(i + 1) % 4]]),
            [Corner; 4]
        )
    }
}

impl Transpose for Face {
    fn transpose_with_projection(&mut self, from: Projection, to: Projection) {
        let index = from
            .iter()
            .position(|x| x == self)
            .unwrap();

        *self = to[index]
    }
}

impl Display for Face {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", match self.0 {
            0 => "W",
            1 => "G",
            2 => "R",
            3 => "Y",
            4 => "B",
            5 => "O",
            x => panic!("Face has inner value {}", x)
        })
    }
}

impl Debug for Face {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.0)
    }
}