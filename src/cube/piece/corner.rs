use super::face::Face;
use crate::cube::piece::Piece;
use crate::cube::resort::Resort;
use super::position::CornerPosition;
use crate::cube::transpose::{Transpose, Projection};
use std::fmt::{Display, Formatter, Error, Debug};

#[macro_export]
macro_rules! corner {
    ($f0:expr, $f1:expr, $f2:expr) => {{
        let id = pos![$f0, $f1, $f2];
        Corner::new(id, id)
    }};
    ($id0:expr, $id1:expr, $id2:expr, $pos0:expr, $pos1:expr, $pos2:expr) => {{
        let id = pos![$id0, $id1, $id2];
        let pos = pos![$pos0, $pos1, $pos2];
        Corner::new(id, pos)
    }}
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Corner {
    pub id: CornerPosition,
    pub pos: CornerPosition,
}

impl Corner {
    pub fn new(id: CornerPosition, pos: CornerPosition) -> Self {
        let mut corner = Self { id, pos };
        corner.resort();
        corner
    }

    pub fn is_at(self, position: CornerPosition) -> bool {
        self.pos.sorted() == position.sorted()
    }

    pub fn id_on(self, pos_face: Face) -> Face {
        let Self { id, pos } = self;
        match pos_face {
            f if f == pos.0 => id.0,
            f if f == pos.1 => id.1,
            f if f == pos.2 => id.2,
            _ => panic!("corner {} not on face {}", self, pos_face)
        }
    }

    #[cfg(test)]
    pub fn as_ruby(self) -> [[Face; 3]; 2] {
        let Self { id, pos } = self;
        [[id.0, id.1, id.2], [pos.0, pos.1, pos.2]]
    }

    pub fn position_without<F: Into<Face>>(self, face: F) -> (Face, Face) {
        self.pos.without(face.into())
    }
}

impl Piece for Corner {
    fn is_on(&self, face: Face) -> bool {
        let pos = self.pos;
        pos.0 == face || pos.1 == face || pos.2 == face
    }

    fn transpose_pos_with_projection(&mut self, from: Projection, to: Projection) {
        self.pos.transpose_with_projection(from, to);
        self.resort();
    }
}

impl Resort for Corner {
    fn resort(&mut self) {
        let Self { id, pos } = self;
        let mut vec = vec![
            (id.0, pos.0),
            (id.1, pos.1),
            (id.2, pos.2)
        ];
        vec.sort();

        self.id = (vec[0].0, vec[1].0, vec[2].0).into();
        self.pos = (vec[0].1, vec[1].1, vec[2].1).into();
    }
}

impl Transpose for Corner {
    fn transpose_with_projection(&mut self, from: Projection, to: Projection) {
        self.id.transpose_with_projection(from, to);
        self.pos.transpose_with_projection(from, to);
        self.resort();
    }
}

impl Display for Corner {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let Self { id, pos } = self;
        write!(f, "E[{}, {}, {}]->({}, {}, {})", id.0, id.1, id.2, pos.0, pos.1, pos.2)
    }
}

impl Debug for Corner {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let Self { id, pos } = self;
        write!(f, "C[{:?}, {:?}, {:?}]->({:?}, {:?}, {:?})", id.0, id.1, id.2, pos.0, pos.1, pos.2)
    }
}