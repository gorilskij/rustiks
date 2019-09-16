use super::face::Face;
use crate::cube::transpose::{Transpose, Projection};
use std::fmt::{Debug, Display, Formatter, Error};
use std::iter::once;

#[macro_export]
macro_rules! position {
    ($f0:expr, $f1:expr) => {
        crate::cube::piece::position::Position::new($crate::face!($f0), $crate::face!($f1))
    };
    ($f0:expr, $f1:expr, $f2:expr) => {
        crate::cube::piece::position::CornerPosition::new(
            $crate::face!($f0),
            $crate::face!($f1),
            $crate::face!($f2)
        )
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Position(Face, Face);

impl<F: Into<Face>> From<(F, F)> for Position {
    fn from((f0, f1): (F, F)) -> Self {
        Self(f0.into(), f1.into())
    }
}

impl Position {
    pub fn new(f0: Face, f1: Face) -> Self {
        Self(f0, f1)
    }

    pub fn faces(&self) -> (Face, Face) {
        (self.0, self.1)
    }

    pub fn projection(&self) -> [Face; 6] {
        let mut mid = self.0.adjacent_clockwise();

        let len = mid.len();
        let index = mid
            .iter()
            .position(|x| *x == self.1)
            .unwrap();

        mid.rotate_left((index + 3) % len);

        // this is ugly, TODO: improve
        let opposite = self.0.opposite();
        let iterator = once(&self.0)
            .chain(&mid)
            .chain(once(&opposite))
            .map(|x| *x);

        array_collect!(iterator, [Face; 6])
    }

    pub fn sorted(&self) -> Self {
        if self.0 > self.1 {
            Self(self.1, self.0)
        } else {
            *self
        }
    }
}

impl Transpose for Position {
    fn transpose_with_projection(&mut self, from: Projection, to: Projection) {
        self.0.transpose_with_projection(from, to);
        self.1.transpose_with_projection(from, to);
    }
}

pub type EdgePosition = Position;

impl Debug for EdgePosition {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "({:?} {:?})", self.0, self.1)
    }
}

impl Display for EdgePosition {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "({} {})", self.0, self.1)
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CornerPosition(Face, Face, Face);

impl<F: Into<Face>> From<(F, F, F)> for CornerPosition {
    fn from((f0, f1, f2): (F, F, F)) -> Self {
        Self(f0.into(), f1.into(), f2.into())
    }
}

impl CornerPosition {
    pub fn new(f0: Face, f1: Face, f2: Face) -> Self {
        Self(f0, f1, f2)
    }

    pub fn faces(&self) -> (Face, Face, Face) {
        (self.0, self.1, self.2)
    }

    pub fn sorted(&self) -> Self {
        let fs = self.faces();
        let mut vec = vec![fs.0, fs.1, fs.2];
        vec.sort();
        Self(vec[0], vec[1], vec[2])
    }
}

impl Transpose for CornerPosition {
    fn transpose_with_projection(&mut self, from: Projection, to: Projection) {
        self.0.transpose_with_projection(from, to);
        self.1.transpose_with_projection(from, to);
        self.2.transpose_with_projection(from, to);
    }
}

impl Debug for CornerPosition {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "({:?} {:?} {:?})", self.0, self.1, self.2)
    }
}

impl Display for CornerPosition {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "({} {} {})", self.0, self.1, self.2)
    }
}