pub use piece::{edge::Edge, corner::Corner};

use piece::face::Face;

use super::support::Lazy;
use std::fmt::{Debug, Formatter, Error, Display};
use crate::cube::transpose::{Transpose, Projection, Transposed};
use piece::position::{EdgePosition, CornerPosition};
use itertools::Itertools;
use crate::cube::algorithm::{Algorithm, Move};
use crate::cube::piece::Piece;

#[macro_use]
pub mod piece;

pub mod transpose;
mod resort;

pub mod algorithm;

pub struct Cube {
    edges: [Edge; 12],
    corners: [Corner; 8],
}

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

    pub fn get_face<F: Into<Face>>(&self, face: F, below: F) -> FaceMatrix {
        let position = (below, face).into();

        println!("{:?}", position);

        println!("[5, 2, 0, 3, 1, 4]");

        // basic assumptions:
        let f = face!(5).transposed_from_default(position);
        let b = face!(2).transposed_from_default(position);
        let d = face!(0).transposed_from_default(position);
        let u = face!(3).transposed_from_default(position);
        let l = face!(1).transposed_from_default(position);
        let r = face!(4).transposed_from_default(position);

        println!("{:?}", [f,b,d,u,l,r]);

        FaceMatrix([
            [
                self.corner_at((f, l, u).into()).id_on(f),
                self.edge_at((f, u).into()).id_on(f),
                self.corner_at((f, r, u).into()).id_on(f),
            ],
            [
                self.edge_at((f, l).into()).id_on(f),
                f,
                self.edge_at((f, r).into()).id_on(f),
            ],
            [
                self.corner_at((f, l, d).into()).id_on(f),
                self.edge_at((f, d).into()).id_on(f),
                self.corner_at((f, r, d).into()).id_on(f),
            ]
        ])
    }

    fn apply_move(&mut self, m: &Move) {
        let (face, times) = (m.face(), m.times());
        let clockwise = face.adjacent_clockwise();
        self.edges
            .iter_mut()
            .filter(|e| e.is_on(face))
            .for_each(|edge| {
                let missing = edge.position_without(face);
                let index: usize = clockwise.iter().position(
                    |x| *x == missing).unwrap();
                let next = clockwise[(index + times as usize) % clockwise.len()];
                println!("from {:?} to {:?}", (face, missing), (face, next));
                println!("was: {:?}", edge);
                edge.transpose_pos((face, missing).into(), (face, next).into());
                println!("is: {:?}", edge);
                println!();
            });

        self.corners
            .iter_mut()
            .filter(|c| c.is_on(face))
            .for_each(|corner| {
                let missing = corner.position_without(face).0;
                let index: usize = clockwise.iter().position(
                    |x| *x == missing
                ).unwrap();
                let next = clockwise[(index + times as usize) % clockwise.len()];
                corner.transpose_pos((face, missing).into(), (face, next).into());
            });
    }

    pub fn apply(&mut self, algorithm: &Algorithm) {
        for m in algorithm.iter() {
            println!("applying {:?}", m);
            self.apply_move(m)
        }
    }
}

// iteration
impl Cube {
    pub fn iter_edges(&self) -> impl Iterator<Item=&Edge> { self.edges.iter() }
    pub fn iter_corners(&self) -> impl Iterator<Item=&Corner> { self.corners.iter() }
    pub fn iter_pieces(&self) -> impl Iterator<Item=&dyn Piece> {
        self.iter_edges()
            .map(|e| e as &dyn Piece)
            .chain(
                self.iter_corners()
                    .map(|c| c as &dyn Piece)
            )
    }
    pub fn iter_edges_mut(&mut self) -> impl Iterator<Item=&mut Edge> { self.edges.iter_mut() }
    pub fn iter_corners_mut(&mut self) -> impl Iterator<Item=&mut Corner> { self.corners.iter_mut() }

    pub fn iter_pieces_on<F: Into<Face>>(&self, face: F) -> impl Iterator<Item=&dyn Piece> {
        let face = face.into();
        self.iter_pieces().filter(move |p: &&dyn Piece| p.is_on(face))
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
        macro_rules! format_face {
            ($face: expr, $below: expr) => {
                format!("{:?}", self.get_face($face, $below))
            };
        }

        macro_rules! push_face {
            ($face: expr) => {
                $face.lines().map(|l| {
                    vec!["       ", l].join("")
                }).collect::<Vec<_>>().join("\n");
            }
        }

        let face0 = push_face!(format_face!(0, 2));
        let face1 = format_face!(1, 3);
        let face2 = format_face!(2, 3);
        let face3 = push_face!(format_face!(3, 5));
        let face4 = format_face!(4, 3);
        let face5 = format_face!(5, 3);

        let central_band = face1.lines().zip(face2.lines()).zip(face4.lines()).zip(face5.lines())
            .map(|(((l1, l2), l4), l5)| {
                vec![l1, l2, l4, l5].join("  ")
            }).collect::<Vec<_>>().join("\n");

        writeln!(f, "{}\n", face0)?;
        writeln!(f, "{}\n", central_band)?;
        writeln!(f, "{}", face3)
    }
}