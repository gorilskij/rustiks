use crate::cube::transpose::{Transpose, Projection};
use std::fmt::{Display, Formatter, Error, Debug};
use std::ops::{Add, Sub};
use crate::cube::piece::{edge::Edge, corner::Corner};
use serde::Deserialize;

// TODO: consider converting to an enum
// TODO: or writing a strong tie between front, back, ... and 0, 1, ...
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Deserialize, Hash)]
pub struct Face(u8);

impl From<u8> for Face {
    fn from(n: u8) -> Self {
        Self::new(n)
    }
}

impl From<char> for Face {
    fn from(c: char) -> Self {
        Self::new(c.to_digit(10)
            .unwrap_or_else(|| panic!("{} is not a valid integer", c)) as u8)
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

    pub fn unwrap(self) -> u8 {
        self.0
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
            adjacent.iter().map(|&f| edge![f, self]),
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
            1 => "R",
            2 => "B",
            3 => "Y",
            4 => "O",
            5 => "G",
            _ => unreachable!(),
        })
    }
}

impl Debug for Face {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.0)
    }
}