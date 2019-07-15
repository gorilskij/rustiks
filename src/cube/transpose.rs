use crate::cube::position::Position;
use crate::cube::face::Face;

pub type Projection = [Face; 6];

pub trait Transpose where Self: Sized {
    fn transpose_with_projection(&self, from: Projection, to: Projection) -> Self;

    fn transpose(&self, from: Position, to: Position) -> Self {
        self.transpose_with_projection(from.projection(), to.projection())
    }

    fn transpose_from_default(&self, to:Position) -> Self {
        // TODO: check if 0, 5 or 5, 0
        self.transpose(position![0, 5], to)
    }
}