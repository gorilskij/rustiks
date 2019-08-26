pub use piece::{edge::Edge, corner::Corner};

use piece::face::Face;

use super::support::Lazy;
use std::fmt::{Debug, Formatter, Error, Display};
use crate::cube::transpose::{Transpose, Projection, Transposed};
use piece::position::{EdgePosition, CornerPosition};
use itertools::Itertools;
use crate::cube::algorithm::{Algorithm, Move};
use crate::cube::piece::Piece;
use std::process::exit;

#[macro_use]
pub mod piece;

pub mod transpose;
mod resort;

pub mod algorithm;

pub struct Cube {
    edges: [Edge; 12],
    corners: [Corner; 8],
}



static mut SOLVED_CUBE: Lazy<Cube> = Lazy::new();

macro_rules! collect_edges {
    ($iter:expr) => { array_collect!($iter, [Edge; 12]) }
}

macro_rules! collect_corners {
    ($iter:expr) => { array_collect!($iter, [Corner; 8]) }
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
                   .map(|line|
                       line
                           .iter()
                           .map(|f| format!("{:?}", f))
                           .join(" ")
                   )
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

        let edges = edges_on_0.iter()
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

        Self { edges, corners }
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
        let position = position!(below, face);

        // basic assumptions:
        let f = face!(5).transposed_from_default(position);
        let b = face!(2).transposed_from_default(position);
        let d = face!(0).transposed_from_default(position);
        let u = face!(3).transposed_from_default(position);
        let l = face!(1).transposed_from_default(position);
        let r = face!(4).transposed_from_default(position);

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

    pub fn iter_pieces(&self) -> Iter {
        Iter::new(self)
    }

    pub(crate) fn iter_pieces_mut(&mut self) -> IterMut {
        IterMut::new(self)
    }

    fn apply_move(&mut self, m: &Move) {
//        let (face, times) = (m.face(), m.times());
//        self.edges
//            .iter_mut()
//            .filter(|e| e.has_face_on(face))
//            .map()
    }

    pub fn apply(&mut self, algorithm: &Algorithm) {
        for m in algorithm.iter() {
            self.apply_move(m)
        }
    }
}

pub struct Iter<'a>(usize, &'a Cube);

impl<'a> Iter<'a> {
    fn new(cube: &'a Cube) -> Self {
        Self(0, cube)
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a dyn Piece;

    fn next(& mut self) -> Option<Self::Item> {
        let piece = match self.0 {
            i@0..=11 => &self.1.edges[i] as &_,
            i@12..=19 => &self.1.corners[i - 12] as &_,
            _ => return None
        };
        self.0 += 1;
        Some(piece)
    }
}

pub struct IterMut<'a>(usize, &'a mut Cube);

impl<'a> IterMut<'a> {
    fn new(cube: &'a mut Cube) -> Self {
        Self(0, cube)
    }
}

impl<'a> Iterator for IterMut<'a> {
    type Item = &'a mut dyn Piece;

    fn next(&mut self) -> Option<Self::Item> {
        let piece: *mut dyn Piece = match self.0 {
            i@0..=11 => &mut self.1.edges[i],
            i@12..=19 => &mut self.1.corners[i - 12],
            _ => return None
        };
        self.0 += 1;
        Some(unsafe { &mut *piece })
    }
}

impl Transpose for Cube {
    fn transpose_with_projection(&mut self, from: Projection, to: Projection) {
        self.edges
            .iter_mut()
            .for_each(|e| e.transpose_with_projection(from, to));

        self.corners
            .iter_mut()
            .for_each(|c| c.transpose_with_projection(from, to));
    }
}

impl Debug for Cube {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        unimplemented!()
    }
}