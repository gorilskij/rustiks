use std::fmt::{Debug, Display};
use crate::cube::transpose::{Transpose, Projection};
use crate::cube::piece::position::CubePosition;
use crate::cube::piece::face::Face;

#[macro_use] pub mod position;
#[macro_use] pub mod edge;
#[macro_use] pub mod corner;
pub mod face;

pub trait Piece: Debug + Display + Transpose {
    fn is_on(&self, face: Face) -> bool;
    fn transpose_pos_with_projection(&mut self, from: Projection, to: Projection);
    fn transpose_pos(&mut self, from: CubePosition, to: CubePosition) {
        self.transpose_pos_with_projection(
            from.projection(),
            to.projection()
        );
    }
}

// TODO: reimplement displays and debugs in terms of positions