use std::fmt::{Debug, Display};
use crate::cube::transpose::{Transpose, Projection};
use crate::cube::piece::face::Face;
//use crate::cube::piece::edge::Piece;
use crate::cube::piece::position::Position;
use crate::cube::resort::Resort;
use itertools::Itertools;

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

// TODO: check if resort trait is needed and implement lazy resort (on get, not on set)
impl<const N: usize> Resort for Piece<N> {
    fn resort(&mut self) {
//        let sorted_pairs = array_collect!(self.id.iter().copied().zip(self.pos.iter().copied()),
//            [(Face, Face); N]);

//        self.id = array_collect!(sorted_pairs.iter().map(|x| x.0), [Face; N]).into();
//        self.pos = array_collect!(sorted_pairs.iter().map(|x| x.1), [Face; N]).into();

        // this is a bad implementation, TODO: improve when const generics allow
        let mut pairs = self.id.iter().copied()
            .zip(self.pos.iter().copied())
            .collect::<Vec<_>>();

        pairs.sort();

        let mut ida = [Face::new(0); N];
        let mut posa = [Face::new(0); N];

        for (i, (iv, pv)) in pairs.into_iter().enumerate() {
            ida[i] = iv;
            posa[i] = pv;
        }

        self.id = Position(ida);
        self.pos = Position(posa);
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

    pub fn id(&self) -> &Position<N> { &self.id }

    pub fn pos(&self) -> &Position<N> { &self.pos }

    pub fn transpose_pos(&mut self, from: Position<2>, to: Position<2>) {
        self.pos.transpose(from, to)
    }

    pub fn is_on(&self, face: Face) -> bool {
        self.pos.iter().any(|&f| f == face)
    }

    pub fn id_on(&self, face: Face) -> Face {
        let idx = self.pos.iter()
            .position(|&f| f == face)
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