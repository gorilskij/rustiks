use std::fmt::{Debug, Display};
use crate::cube::transpose::{Transpose, Projection};
use crate::cube::piece::face::Face;
//use crate::cube::piece::edge::Piece;
use crate::cube::piece::position::Position;
use crate::cube::resort::Resort;
use itertools::Itertools;
use crate::cube::piece::hack::PublicPiece;

#[macro_use] pub mod position;
//#[macro_use] pub mod edge;
//#[macro_use] pub mod corner;

pub type Edge = Piece<2>;
pub type Corner = Piece<3>;

//#[macro_export]
macro_rules! edge {
    ($a:expr, $b:expr) => {{
        use $crate::cube::piece::{Piece, position::Position};
        let pos = Position([$a, $b]);
        Piece::new(pos, pos)
    }};
}

macro_rules! corner {
    ($a:expr, $b:expr, $c:expr) => {{
        use $crate::cube::piece::{Piece, position::Position};
        let pos = Position([$a, $b, $c]);
        Piece::new(pos, pos)
    }};
}

pub mod face;

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

// TODO: check if resort trait is needed and implement lazy resort (on get, not on set)
impl<const N: usize> Resort for Piece<N> {
    fn resort(&mut self) {
        let sorted_pairs = array_collect!(self.id.iter().zip(self.pos.iter()),
            [(Face, Face); N]);

        self.id = array_collect!(sorted_pairs.iter().map(|x| x.0), [Face; N]).into();
        self.pos = array_collect!(sorted_pairs.iter().map(|x| x.1), [Face; N]).into();
    }
}

impl<const N: usize> Transpose for Piece<N> {
    fn transpose_with_projection(&mut self, from: [Face; 6], to: [Face; 6]) {
        self.id.transpose_with_projection(from, to);
        self.pos.transpose_with_projection(from, to);
        self.resort();
    }
}

impl<const N: usize> Piece<N> {
//    pub fn transpose_pos(&mut self, from: Position<2>, Position<2>) {
//        self.pos.transpose()
//    }

    pub fn new(id: Position<N>, pos: Position<N>) -> Self {
        let mut piece = Self { id, pos };
        piece.resort();
        piece
    }

    pub fn modify(&mut self, f: impl FnOnce(&mut PublicPiece<N>)) {
        let mut pp = PublicPiece { id: self.id, pos: self.pos };
        f(&mut pp);
        let mut new = Self { id: pp.id, pos: pp.pos };
        new.resort();
        *self = new;
    }

    pub fn is_on(&self, face: Face) -> bool {
        self.pos.iter().any(|f| f == face)
    }

    pub fn id_on(&self, face: Face) -> Face {
        let idx = self.pos.iter()
            .position(|f| f == face)
            .expect("not on that face");

        self.id[idx]
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