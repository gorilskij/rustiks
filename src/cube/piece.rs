use crate::cube::position::{CornerPosition, EdgePosition, Position};
use crate::cube::transpose::{Transpose, Projection};
use crate::cube::face::Face;
use std::fmt::{Debug, Formatter, Error, Display};

pub struct Edge(EdgePosition, EdgePosition);

impl Edge {
//    #[cfg(debug)] // TODO: check what this does
    pub fn from_nums(id0: u8, id1: u8, pos0: u8, pos1: u8) -> Self {
        Self::from(
            Position(Face::from(id0), Face::from(id1)),
            Position(Face::from(pos0), Face::from(pos1))
        )
    }

    fn from(id: EdgePosition, pos: EdgePosition) -> Self {
        let mut vec = vec![
            (id.0, pos.0),
            (id.1, pos.1)
        ];

        vec.sort();

        Self(
            Position(vec[0].0, vec[1].0),
            Position(vec[0].1, vec[1].1)
        )
    }
}

impl Transpose for Edge {
    fn transpose_with_projection(&self, from: Projection, to: Projection) -> Self {
        Self::from(
            self.0.transpose_with_projection(from, to),
            self.1.transpose_with_projection(from, to)
        )
    }
}

impl Display for Edge {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "E[{}, {}]->({}, {})", (self.0).0, (self.0).1, (self.1).0, (self.1).1)
    }
}

impl Debug for Edge {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "E[{:?}, {:?}]->({:?}, {:?})", (self.0).0, (self.0).1, (self.1).0, (self.1).1)
    }
}

pub struct Corner(CornerPosition, CornerPosition);

impl Corner {
    pub fn from_nums(id0: u8, id1: u8, id2: u8, pos0: u8, pos1: u8, pos2: u8) -> Self {
        Self::from(
            CornerPosition(Face::from(id0), Face::from(id1), Face::from(id2)),
            CornerPosition(Face::from(pos0), Face::from(pos1), Face::from(pos2))
        )
    }

    fn from(id: CornerPosition, pos: CornerPosition) -> Self {
        let mut vec = vec![
            (id.0, pos.0),
            (id.1, pos.1),
            (id.2, pos.2)
        ];

        vec.sort();

        Self(
            CornerPosition(vec[0].0, vec[1].0, vec[2].0),
            CornerPosition(vec[0].1, vec[1].1, vec[2].1)
        )
    }
}

impl Transpose for Corner {
    fn transpose_with_projection(&self, from: Projection, to: Projection) -> Self {
        Self::from(
            self.0.transpose_with_projection(from, to),
            self.1.transpose_with_projection(from, to)
        )
    }
}

impl Display for Corner {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let id = &self.0;
        let pos = &self.1;
        write!(f, "C[{}, {}, {}]->({}, {}, {})", id.0, id.1, id.2, pos.0, pos.1, pos.2)
    }
}

impl Debug for Corner {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let id = &self.0;
        let pos = &self.1;
        write!(f, "C[{:?}, {:?}, {:?}]->({:?}, {:?}, {:?})", id.0, id.1, id.2, pos.0, pos.1, pos.2)
    }
}