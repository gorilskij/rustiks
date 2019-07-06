use crate::cube::position::Position;
use crate::cube::face::Face;

pub type Projection = [Face; 6];

pub trait Transpose {
    fn transpose_with_projection(&self, from: Projection, to: Projection) -> Self;
    fn transpose(&self, from: Position, to: Position) -> Self where Self: Sized {
        self.transpose_with_projection(from.projection(), to.projection())
    }
}