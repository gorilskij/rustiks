pub use crate::cube::piece::{Edge, Corner};
use crate::cube::face::Face;
use std::slice::Iter;

use super::support::Lazy;
use std::fmt::{Debug, Formatter, Error, Display};
use crate::cube::transpose::{Transpose, Projection};
use crate::cube::position::{EdgePosition, CornerPosition, Position};
use itertools::Itertools;

#[macro_use]
pub mod piece;

#[macro_use]
pub mod face;

pub mod position;
pub mod transpose;
pub mod resort;

pub mod algorithm;

pub struct Cube {
    edges: [Edge; 12],
    corners: [Corner; 8],
}

static mut SOLVED_CUBE: Lazy<Cube> = Lazy::new();

macro_rules! collect_edges {
    ($iter: expr) => { array_collect!($iter, [Edge; 12]) }
}

macro_rules! collect_corners {
    ($iter: expr) => { array_collect!($iter, [Corner; 8]) }
}

pub struct FaceMatrix([[Face; 3]; 3]);

impl Display for FaceMatrix {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}",
               self.0
                   .iter()
                   .map(|line| format!("{} {} {}", line[0], line[1], line[2]))
                   .join("\n")
        )
    }
}

impl Debug for FaceMatrix {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}",
               self.0
                   .iter()
                   .map(|line| format!("{:?} {:?} {:?}", line[0], line[1], line[2]))
                   .join("\n")
        )
    }
}

impl Cube {
    // this method is generally ugly both visually and in implementation
    // TODO: remove stink
    pub fn solved() -> Self {
        // note: lets come first because otherwise a "freed while in use"
        // error is thrown, I think arrays aren't IntoIterator TODO: check
        let edges_on_0 = face!(0).adjacent_edges();
        let edges_on_3 = face!(3).adjacent_edges();
        let edges_around = [
            Edge::between(face!(1), face!(2)),
            Edge::between(face!(2), face!(4)),
            Edge::between(face!(4), face!(5)),
            Edge::between(face!(5), face!(1)),
        ];

        let mut edges = edges_on_0.iter()
            .chain(edges_on_3.iter())
            .chain(&edges_around)
            .map(|e| *e);

        let corners_on_0 = face!(0).adjacent_corners();
        let corners_on_3 = face!(3).adjacent_corners();

        let corners = corners_on_0.iter()
            .chain(&corners_on_3)
            .map(|c| *c);

        let edges = collect_edges!(edges);
        let corners = collect_corners!(corners);

        Self { edges, corners, }
    }

    pub fn edge_at(&self, position: EdgePosition) -> &Edge {
        match self.edges.iter().find(|e| e.is_at(position)) {
            Some(e) => e,
            None => unreachable!("no edge at {:?}", position)
        }
    }

    pub fn corner_at(&self, position: CornerPosition) -> &Corner {
        match self.corners.iter().find(|c| c.is_at(position)) {
            Some(c) => c,
            None => unreachable!("no corner at {:?}", position)
        }
    }

    pub fn get_face(&self, face: Face, below: Face) -> FaceMatrix {
        let position = position!(face, below);

        // basic assumptions:
        let f = face!(5).transpose_from_default(position);
        let b = face!(2).transpose_from_default(position);
        let d = face!(0).transpose_from_default(position);
        let u = face!(3).transpose_from_default(position);
        let l = face!(1).transpose_from_default(position);
        let r = face!(4).transpose_from_default(position);
        
        FaceMatrix([
            [
                self.corner_at(position!(f, l, u)).id_on(f),
                self.edge_at(position!(f, u)).id_on(f),
                self.corner_at(position!(f, r, u)).id_on(f),
            ],
            [
                self.edge_at(position!(f, l)).id_on(f),
                f,
                self.edge_at(position!(f, r)).id_on(f),
            ],
            [
                self.corner_at(position!(f, l, d)).id_on(f),
                self.edge_at(position!(f, d)).id_on(f),
                self.corner_at(position!(f, r, d)).id_on(f),
            ]
        ])
    }
}

impl Transpose for Cube {
    fn transpose_with_projection(&self, from: Projection, to: Projection) -> Self {
        Self {
            edges: collect_edges!(
                self.edges
                    .iter()
                    .map(|e| e.transpose_with_projection(from, to))
            ),

            corners: collect_corners!(
                self.corners
                    .iter()
                    .map(|c| c.transpose_with_projection(from, to))
            ),
        }
    }
}

impl Debug for Cube {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        unimplemented!()
    }
}