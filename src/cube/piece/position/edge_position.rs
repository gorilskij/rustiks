use crate::cube::piece::face::Face;
use crate::cube::transpose::{Transpose, Projection};
use std::fmt::{Debug, Display, Formatter, Error};
use serde::Deserialize;
use std::iter::FromIterator;
use std::cmp::Ordering;

#[derive(Copy, Clone, Eq, PartialEq, Deserialize, Hash)]
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
            f => panic!("face {:?} not in EdgePosition", f),
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

impl<A: Into<Face>> FromIterator<A> for EdgePosition {
    fn from_iter<T: IntoIterator<Item=A>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        let exp = "Expected 2 values, got fewer";
        pos!(iter.next().expect(exp), iter.next().expect(exp))
    }
}

impl PartialOrd for EdgePosition {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for EdgePosition {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.0 == other.0 {
            self.1.cmp(&other.1)
        } else {
            self.0.cmp(&other.0)
        }
    }
}