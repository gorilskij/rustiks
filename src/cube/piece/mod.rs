use std::fmt::{Debug, Display};
use crate::cube::transpose::{Transpose, Projection};
use crate::cube::piece::face::Face;
use crate::cube::piece::edge::Piece;
use crate::cube::piece::position::Position;

#[macro_use] pub mod position;
//#[macro_use] pub mod edge;
//#[macro_use] pub mod corner;
pub mod face;

pub type Edge = Piece<2>;
pub type Corner = Piece<3>;


#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Piece<const N: usize> {
    id: Position<N>,
    pos: Position<N>,
}

mod hack {
    use std::ops::Deref;
    use super::Piece;
    use crate::cube::piece::position::Position;

    pub struct PublicPiece<const N: usize> {
        pub id: Position<N>,
        pub pos: Position<N>,
    }

    impl<const N: usize> Deref for Piece<N> {
        type Target = PublicPiece<N>;

        fn deref(&self) -> &Self::Target {
            &PublicPiece { id: self.id, pos: self.pos }
        }
    }
}





















//pub trait Piece: Debug + Display + Transpose {
//    fn is_on(&self, face: Face) -> bool;
//    fn transpose_pos_with_projection(&mut self, from: Projection, to: Projection);
//    fn transpose_pos(&mut self, from: CubePosition, to: CubePosition) {
//        self.transpose_pos_with_projection(
//            from.projection(),
//            to.projection()
//        );
//    }
//}
//
// TODO: reimplement displays and debugs in terms of positions