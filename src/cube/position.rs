use crate::cube::face::Face;
use crate::cube::transpose::{Transpose, Projection};

pub struct Position(pub Face, pub Face);

impl Position {
    pub fn from(p0: u8, p1: u8) -> Self {
        Self(Face::from(p0), Face::from(p1))
    }

    pub fn projection(&self) -> Vec<Face> {
        let mut vec = Vec::with_capacity(6);
        let mut mid = self.0.adjacent_clockwise();

        let index = mid
            .iter()
            .position(|x| *x == self.1)
            .unwrap();

        mid.rotate_left(index - 1);

        vec.push(self.0);
        vec.append(&mut mid);
        vec.push(self.0.opposite());

        vec
    }
}

impl Transpose for Position {
    fn transpose_with_projection(&self, from: Projection, to: Projection) -> Self {
        Position(
            self.0.transpose_with_projection(from, to),
            self.1.transpose_with_projection(from, to)
        )
    }
}

pub type EdgePosition = Position;

pub struct CornerPosition(pub Face, pub Face, pub Face);

impl Transpose for CornerPosition {
    fn transpose_with_projection(&self, from: &Vec<Face>, to: &Vec<Face>) -> Self {
        CornerPosition(
            self.0.transpose_with_projection(from, to),
            self.1.transpose_with_projection(from, to),
            self.2.transpose_with_projection(from, to)
        )
    }
}