use std::fmt::{Debug, Display, Formatter, Error};
use crate::cube::transpose::{Transpose, Projection};
use crate::cube::resort::Resort;
use itertools::Itertools;
use crate::cube::position::Pos;
use crate::cube::face::Face;


pub type Edge = Piece<2>;
pub type Corner = Piece<3>;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Piece<const N: usize> {
    id: Pos<N>,
    pos: Pos<N>,
}

// todo derive
impl<const N: usize> Debug for Piece<N> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "({})[{}]",
            self.id.iter()
                .map(|f| format!("{:?}", f))
                .join(" "),
            self.pos.iter()
               .map(|f| format!("{:?}", f))
               .join(" "),
        )
    }
}

// TODO: check if resort trait is needed and implement lazy resort (on get, not on set)
impl<const N: usize> Resort for Piece<N> {
    // todo improve implementation when const generics allow
    fn resort(&mut self) {
        let mut pairs = self.id.iter().copied()
            .zip(self.pos.iter().copied())
            .collect::<Vec<_>>();

        pairs.sort();

        for (i, (iv, pv)) in pairs.into_iter().enumerate() {
            self.id[i] = iv;
            self.pos[i] = pv;
        }
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

    pub fn new(id: Pos<N>, pos: Pos<N>) -> Self {
        let mut piece = Self { id, pos };
        piece.resort();
        piece
    }

    pub fn id(&self) -> &Pos<N> { &self.id }

    pub fn pos(&self) -> &Pos<N> { &self.pos }

    pub fn transpose_pos(&mut self, from: Pos<2>, to: Pos<2>) {
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

    pub fn is_solved(&self) -> bool {
        self.id == self.pos
    }
}

impl Piece<2> {
    pub fn new_edge<T: Into<Face>>(a: T, b: T) -> Self {
        let pos = pos!(a, b);
        Self::new(pos, pos)
    }
}

impl Piece<3> {
    pub fn new_corner<T: Into<Face>>(a: T, b: T, c: T) -> Self {
        let pos = pos!(a, b, c);
        Self::new(pos, pos)
    }
}