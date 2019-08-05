use super::piece::{face::Face, position::Position};

pub type Projection = [Face; 6];

pub trait Transpose {
    fn transpose_with_projection(&mut self, from: Projection, to: Projection);

    fn transpose(&mut self, from: Position, to: Position){
        self.transpose_with_projection(from.projection(), to.projection())
    }

    fn transpose_from_default(&mut self, to: Position) {
        // TODO: check if 0, 5 or 5, 0
        self.transpose(position![0, 5], to)
    }
}

pub trait Transposed where Self: Sized {
    fn transposed_with_projection(&self, from: Projection, to: Projection) -> Self;
    fn transposed(&self, from: Position, to: Position) -> Self;
    fn transposed_from_default(&self, to: Position) -> Self;
}

impl<T: Transpose + Clone> Transposed for T {
    fn transposed_with_projection(&self, from: Projection, to: Projection) -> Self {
        let mut clone = self.clone();
        clone.transpose_with_projection(from, to);
        clone
    }

    fn transposed(&self, from: Position, to: Position) -> Self {
        let mut clone = self.clone();
        clone.transpose(from, to);
        clone
    }

    fn transposed_from_default(&self, to: Position) -> Self {
        let mut clone = self.clone();
        clone.transpose_from_default(to);
        clone
    }
}