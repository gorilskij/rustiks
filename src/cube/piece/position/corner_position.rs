use crate::cube::piece::face::Face;
use crate::cube::transpose::{Transpose, Projection};
use std::fmt::{Debug, Display, Formatter, Error};

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CornerPosition(pub Face, pub Face, pub Face);

impl<F: Into<Face>> From<(F, F, F)> for CornerPosition {
    fn from((f0, f1, f2): (F, F, F)) -> Self {
        Self(f0.into(), f1.into(), f2.into())
    }
}

impl CornerPosition {
    pub fn sorted(&self) -> Self {
        let mut vec = vec![self.0, self.1, self.2];
        vec.sort();
        Self(vec[0], vec[1], vec[2])
    }

    pub fn without(&self, face: Face) -> (Face, Face) {
        match face {
            f if f == self.0 => (self.1, self.2),
            f if f == self.1 => (self.0, self.2),
            f if f == self.2 => (self.1, self.2),
            _ => panic!(),
        }
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