use crate::cube::face::Face;
use crate::cube::transpose::{Transpose, Projection};
use crate::cube::resort::Resort;
use std::mem::MaybeUninit;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Position(Face, Face);

impl Position {
    pub fn new(f0: Face, f1: Face) -> Self {
        Self(f0, f1)
    }

    pub fn faces(&self) -> (Face, Face) {
        (self.0, self.1)
    }

    pub fn projection(&self) -> [Face; 6] {
        let mut array: [Face; 6] = unsafe {
            std::mem::transmute([MaybeUninit::<Face>::uninit(); 6])
        };

        array[0] = self.0;
        array[5] = self.0.opposite();

        let mut mid = self.0.adjacent_clockwise();

        let index = mid
            .iter()
            .position(|x| *x == self.1)
            .unwrap();

        let len = mid.len();

        mid.rotate_left((index + 3) % len);

        for i in 1..=4 {
            array[i] = mid[i - 1]
        }

        array
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

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CornerPosition(Face, Face, Face);

impl CornerPosition {
    pub fn new(f0: Face, f1: Face, f2: Face) -> Self {
        Self(f0, f1, f2)
    }

    pub fn faces(&self) -> (Face, Face, Face) {
        (self.0, self.1, self.2)
    }
}

impl Transpose for CornerPosition {
    fn transpose_with_projection(&self, from: Projection, to: Projection) -> Self {
        CornerPosition(
            self.0.transpose_with_projection(from, to),
            self.1.transpose_with_projection(from, to),
            self.2.transpose_with_projection(from, to)
        )
    }
}