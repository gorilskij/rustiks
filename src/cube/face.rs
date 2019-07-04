use std::mem::transmute;
use crate::cube::transpose::{Transpose, Projection};
use std::fmt::{Display, Formatter, Error, Debug};

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Face {
    White = 0,
    Green = 1,
    Red =  2,
    Yellow = 3,
    Blue = 4,
    Orange = 5
}

impl Face {
    fn as_u8(self) -> u8 {
        unsafe { transmute(self) }
    }

    pub fn from(value: u8) -> Self {
        assert!(value <= 5);
        unsafe { transmute(value) }
    }

    fn do_as_u8<F>(self, f: F) -> Self
        where F: FnOnce(u8) -> u8
    {
        Self::from(f(self.as_u8()))
    }

    pub fn opposite(self) -> Self {
        Self::from((self.as_u8() + 3) % 6)
    }

    pub fn adjacent(self) -> Vec<Face> {
        // [4, 5, 1, 2] = [-2, -1, 1, 2] (mod 6)
        [4, 5, 1, 2].iter().map(|delta| {
            self.do_as_u8(|slf| (slf + *delta) % 6)
        }).collect()
    }

    pub fn adjacent_clockwise(self) -> Vec<Face> {
        let mut adjacent = self.adjacent();
        if self.as_u8() % 2 == 0 { adjacent.reverse() }
        adjacent
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
        write!(f, "{}", match self {
            Face::White => "W",
            Face::Green => "G",
            Face::Red => "R",
            Face::Yellow => "Y",
            Face::Blue => "B",
            Face::Orange => "O",
        })
    }
}

impl Debug for Face {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.as_u8())
    }
}