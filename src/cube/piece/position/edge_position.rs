use crate::cube::piece::face::Face;
use std::iter::once;
use crate::cube::transpose::{Transpose, Projection};
use std::fmt::{Debug, Display, Formatter, Error};

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EdgePosition(pub Face, pub Face);

impl<F: Into<Face>> From<(F, F)> for EdgePosition {
    fn from((f0, f1): (F, F)) -> Self {
        Self(f0.into(), f1.into())
    }
}

impl EdgePosition {
    pub fn sorted(&self) -> Self {
        if self.0 > self.1 {
            Self(self.1, self.0)
        } else {
            *self
        }
    }

    pub fn without(&self, face: Face) -> Face {
        match face {
            f if f == self.0 => self.1,
            f if f == self.1 => self.0,
            _ => panic!(),
        }
    }
}

impl Transpose for EdgePosition {
    fn transpose_with_projection(&mut self, from: Projection, to: Projection) {
        self.0.transpose_with_projection(from, to);
        self.1.transpose_with_projection(from, to);
    }
}

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