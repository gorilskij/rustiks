use super::face::Face;
use crate::cube::piece::Piece;
use crate::cube::resort::Resort;
use super::position::{Position, EdgePosition};
use crate::cube::transpose::{Transpose, Projection};
use std::fmt::{Display, Formatter, Error, Debug};

#[macro_export]
macro_rules! edge {
    ($f0: expr, $f1: expr) => {{
        let id = $crate::position![$f0, $f1];
        Edge::new(id, id)
    }};
    ($id0: expr, $id1: expr, $pos0: expr, $pos1: expr) => {{
        let id = position![$id0, $id1];
        let pos = position![$pos0, $pos1];
        Edge::new(id, pos)
    }}
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Edge(EdgePosition, EdgePosition);

impl Edge {
    pub fn new(id: EdgePosition, pos: EdgePosition) -> Self {
        let mut edge = Self(id, pos);
        edge.resort();
        edge
    }

    pub fn is_at(&self, position: EdgePosition) -> bool {
        self.1.sorted() == position.sorted()
    }

    pub fn has_face_on(&self, face: Face) -> bool {
        let pos = self.1.faces();
        pos.0 == face || pos.1 == face
    }

    pub fn id_on(&self, pos_face: Face) -> Face {
        let (id, pos) = (self.0.faces(), self.1.faces());
        match pos_face {
            f if f == pos.0 => id.0,
            f if f == pos.1 => id.1,
            _ => panic!("edge {} not on face {}", self, pos_face)
        }
    }

    #[cfg(test)]
    pub fn as_ruby(&self) -> [[Face; 2]; 2] {
        let id = self.0.faces();
        let pos = self.1.faces();
        [[id.0, id.1], [pos.0, pos.1]]
    }

    // this is only used in Cube::solved, TODO: replace this with something better
    pub fn between(f0: Face, f1: Face) -> Self {
        edge!(f0, f1)
    }
}

impl Piece for Edge {

}

impl Resort for Edge {
    fn resort(&mut self) {
        let ids = self.0.faces();
        if ids.0 > ids.1 {
            self.0 = Position::new(ids.1, ids.0);
            let poss = self.1.faces();
            self.1 = Position::new(poss.1, poss.0)
        }
    }
}

impl Transpose for Edge {
    fn transpose_with_projection(&mut self, from: Projection, to: Projection) {
        self.0.transpose_with_projection(from, to);
        self.1.transpose_with_projection(from, to);
        self.resort();
    }
}

impl Display for Edge {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let (id0, id1) = self.0.faces();
        let (pos0, pos1) = self.1.faces();
        write!(f, "E[{}, {}]->({}, {})", id0, id1, pos0, pos1)
    }
}

impl Debug for Edge {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let (id0, id1) = self.0.faces();
        let (pos0, pos1) = self.1.faces();
        write!(f, "E[{:?}, {:?}]->({:?}, {:?})", id0, id1, pos0, pos1)
    }
}