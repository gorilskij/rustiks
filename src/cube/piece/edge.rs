use super::face::Face;
use crate::cube::resort::Resort;
//use super::position::EdgePosition;
use crate::cube::transpose::{Transpose, Projection};
use std::fmt::{Display, Formatter, Error, Debug};
use std::ops::Deref;
use crate::cube::piece::position::Position;

impl<const N: usize> Piece<N> {
    pub fn new(id: EdgePosition, pos: EdgePosition) -> Self {
        let mut edge = Self { id, pos };
        edge.resort();
        edge
    }

    pub fn is_solved(self) -> bool {
        self.id == self.pos
    }

    pub fn is_at(self, position: EdgePosition) -> bool {
        self.pos.sorted() == position.sorted()
    }

    pub fn has_id(self, id: EdgePosition) -> bool {
        self.id.sorted() == id.sorted()
    }

    pub fn id_contains(self, face: Face) -> bool {
        self.id.0 == face || self.id.1 == face
    }

    pub fn id_on(self, pos_face: Face) -> Face {
        match pos_face {
            f if f == self.pos.0 => self.id.0,
            f if f == self.pos.1 => self.id.1,
            _ => panic!("edge {} not on face {}", self, pos_face)
        }
    }

    #[cfg(test)]
    pub fn as_ruby(self) -> [[Face; 2]; 2] {
        let id = self.id;
        let pos = self.pos;
        [[id.0, id.1], [pos.0, pos.1]]
    }

    pub fn position_without<F: Into<Face>>(self, face: F) -> Face {
        self.pos.without(face.into())
    }
}

impl Piece for Edge {
    fn is_on(&self, face: Face) -> bool {
        let pos = self.pos;
        pos.0 == face || pos.1 == face
    }

    fn transpose_pos_with_projection(&mut self, from: Projection, to: Projection) {
        self.pos.transpose_with_projection(from, to);
        self.resort();
    }
}

impl Resort for Edge {
    fn resort(&mut self) {
        let ids = self.id;
        if ids.0 > ids.1 {
            self.id = EdgePosition(ids.1, ids.0);
            let poss = self.pos;
            self.pos = EdgePosition(poss.1, poss.0)
        }
    }
}

impl Transpose for Edge {
    fn transpose_with_projection(&mut self, from: Projection, to: Projection) {
        self.id.transpose_with_projection(from, to);
        self.pos.transpose_with_projection(from, to);
        self.resort();
    }
}

impl Display for Edge {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let Self { id, pos } = self;
        write!(f, "[{} {}]({} {})", id.0, id.1, pos.0, pos.1)
    }
}

impl Debug for Edge {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let Self { id, pos } = self;
        write!(f, "[{:?} {:?}]({:?} {:?})", id.0, id.1, pos.0, pos.1)
    }
}