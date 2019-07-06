use crate::cube::position::{CornerPosition, EdgePosition, Position};
use crate::cube::transpose::{Transpose, Projection};
use crate::cube::resort::Resort;
use crate::cube::face::Face;
use std::fmt::{Debug, Formatter, Error, Display};

#[macro_export]
macro_rules! face {
    ($v: expr) => { Face::from($v) }
}

#[macro_export]
macro_rules! position {
    [$f0: expr, $f1: expr] => {
        crate::cube::position::Position::new(face!($f0), face!($f1))
    };
    [$f0: expr, $f1: expr, $f2: expr] => {
        CornerPosition::new(face!($f0), face!($f1), face!($f2))
    }
}

#[macro_export]
macro_rules! edge {
    ($f0: expr, $f1: expr) => {{
        let id = position![$f0, $f1];
        Edge::new(id, id)
    }};
    ($id0: expr, $id1: expr, $pos0: expr, $pos1: expr) => {{
        let id = position![$id0, $id1];
        let pos = position![$pos0, $pos1];
        Edge::new(id, pos)
    }}
}

#[macro_export]
macro_rules! corner {
    ($f0: expr, $f1: expr, $f2: expr) => {{
        let id = position![$f0, $f1, $f2];
        Corner::new(id, id)
    }};
    ($id0: expr, $id1: expr, $id2: expr, $pos0: expr, $pos1: expr, $pos2: expr) => {{
        let id = position![$id0, $id1, $id2];
        let pos = position![$pos0, $pos1, $pos2];
        Corner::new(id, pos)
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
    fn transpose_with_projection(&self, from: Projection, to: Projection) -> Self {
        Self::new(
            self.0.transpose_with_projection(from, to),
            self.1.transpose_with_projection(from, to)
        )
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

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Corner(CornerPosition, CornerPosition);

impl Corner {
    pub fn new(id: CornerPosition, pos: CornerPosition) -> Self {
        let mut corner = Self(id, pos);
        corner.resort();
        corner
    }
}

impl Resort for Corner {
    fn resort(&mut self) {
        let mut id = self.0.faces();
        let mut pos = self.1.faces();
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
    fn transpose_with_projection(&self, from: Projection, to: Projection) -> Self {
        Self::new(
            self.0.transpose_with_projection(from, to),
            self.1.transpose_with_projection(from, to)
        )
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
