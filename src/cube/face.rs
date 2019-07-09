use std::mem::{transmute, MaybeUninit};
use crate::cube::transpose::{Transpose, Projection};
use std::fmt::{Display, Formatter, Error, Debug};
use crate::cube::{Edge, Corner};
use std::ops::{Add, Sub, Deref};
use std::cmp::Ordering;

// TODO: implement this better
macro_rules! collect_to_array {
    ($iter: expr, [$type: ty; $len: expr]) => {{
        let mut iter = $iter;

        let mut array: [$type; $len] = unsafe {
            std::mem::transmute([std::mem::MaybeUninit::<$type>::uninit(); $len])
        };

        for i in 0..$len { array[i] = iter.next().unwrap(); }
        array
    }}
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

    fn sub(self, mut other: u8) -> Self::Output {
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
    pub fn new(value: u8) -> Self {
        assert!(value <= 5);
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
        collect_to_array!(
            adjacent.iter().map(|f| edge![*f, self]),
            [Edge; 4]
        )
    }

    pub fn adjacent_corners(self) -> [Corner; 4] {
        let adjacent = self.adjacent();
        collect_to_array!(
            (0..4).map(|i| corner![self, adjacent[i], adjacent[(i + 1) % 4]]),
            [Corner; 4]
        )
    }
}

impl Transpose for Face {
    fn transpose_with_projection(&self, from: Projection, to: Projection) -> Self {
        let index = from
            .iter()
            .position(|x| x == self)
            .unwrap();

        to[index]
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