use super::face::Face;
use crate::cube::piece::Piece;
use crate::cube::resort::Resort;
use super::position::CornerPosition;
use crate::cube::transpose::{Transpose, Projection};
use std::fmt::{Display, Formatter, Error, Debug};

#[macro_export]
macro_rules! corner {
    ($f0:expr, $f1:expr, $f2:expr) => {{
        let id = position![$f0, $f1, $f2];
        Corner::new(id, id)
    }};
    ($id0:expr, $id1:expr, $id2:expr, $pos0:expr, $pos1:expr, $pos2:expr) => {{
        let id = position![$id0, $id1, $id2];
        let pos = position![$pos0, $pos1, $pos2];
        Corner::new(id, pos)
    }}
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Corner(CornerPosition, CornerPosition);

impl Corner {
    pub fn new(id: CornerPosition, pos: CornerPosition) -> Self {
        let mut corner = Self(id, pos);
        corner.resort();
        corner
    }

    pub fn is_at(&self, position: CornerPosition) -> bool {
        self.1.sorted() == position.sorted()
    }

    pub fn id_on(&self, pos_face: Face) -> Face {
        let (id, pos) = (self.0.faces(), self.1.faces());
        match pos_face {
            f if f == pos.0 => id.0,
            f if f == pos.1 => id.1,
            f if f == pos.2 => id.2,
            _ => panic!("corner {} not on face {}", self, pos_face)
        }
    }

    #[cfg(test)]
    pub fn as_ruby(&self) -> [[Face; 3]; 2] {
        let id = self.0.faces();
        let pos = self.1.faces();
        [[id.0, id.1, id.2], [pos.0, pos.1, pos.2]]
    }
}

impl Piece for Corner {
    fn is_on(&self, face: Face) -> bool {
        let pos = self.1.faces();
        pos.0 == face || pos.1 == face || pos.2 == face
    }

    fn transpose_pos_with_projection(&mut self, from: Projection, to: Projection) {
        self.1.transpose_with_projection(from, to);
        self.resort();
    }
}

impl Resort for Corner {
    fn resort(&mut self) {
        let id = self.0.faces();
        let pos = self.1.faces();
        let mut vec = vec![
            (id.0, pos.0),
            (id.1, pos.1),
            (id.2, pos.2)
        ];
        vec.sort();

        self.0 = CornerPosition::new(vec[0].0, vec[1].0, vec[2].0);
        self.1 = CornerPosition::new(vec[0].1, vec[1].1, vec[2].1);
    }
}

impl Transpose for Corner {
    fn transpose_with_projection(&mut self, from: Projection, to: Projection) {
        self.0.transpose_with_projection(from, to);
        self.1.transpose_with_projection(from, to);
        self.resort();
    }
}

impl Display for Corner {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let (id0, id1, id2) = self.0.faces();
        let (pos0, pos1, pos2) = self.1.faces();
        write!(f, "C[{}, {}, {}]->({}, {}, {})", id0, id1, id2, pos0, pos1, pos2)
    }
}

impl Debug for Corner {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let (id0, id1, id2) = self.0.faces();
        let (pos0, pos1, pos2) = self.1.faces();
        write!(f, "C[{:?}, {:?}, {:?}]->({:?}, {:?}, {:?})", id0, id1, id2, pos0, pos1, pos2)
    }
}