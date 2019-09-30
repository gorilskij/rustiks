pub use piece::{edge::Edge, corner::Corner};

use piece::face::Face;

use std::fmt::{Debug, Formatter, Error, Display};
use crate::cube::transpose::{Transpose, Projection, Transposed};
use piece::position::{EdgePosition, CornerPosition};
use itertools::Itertools;
use crate::cube::algorithm::{Algorithm, Move};
use crate::cube::piece::Piece;
use crate::cube::piece::position::CubePosition;

#[macro_use]
pub mod piece;

pub mod transpose;
mod resort;

#[macro_use]
pub mod algorithm;

mod manipulation;

#[derive(PartialEq, Copy, Clone)]
pub struct Cube {
    edges: [Edge; 12],
    corners: [Corner; 8],
}

pub struct FaceMatrix([[Face; 3]; 3]);

macro_rules! impl_face_matrix_fmt {
    ($trait: ty, $fmt: expr) => {
        impl $trait for FaceMatrix {
            fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
                for line in &self.0 {
                    writeln!(f, $fmt, line[0], line[1], line[2])?
                }
                Ok(())
            }
        }
    };
}

impl_face_matrix_fmt!(Display, "{} {} {}");
impl_face_matrix_fmt!(Debug, "{:?} {:?} {:?}");

impl Cube {
    // this method is generally ugly both visually and in implementation
    // TODO: remove stink
    pub fn solved() -> Self {
        let edges_on_0 = Face::from(0).adjacent_edges();
        let edges_on_3 = Face::from(3).adjacent_edges();
        let edges_around = [
            Edge::between(1, 2),
            Edge::between(2, 4),
            Edge::between(4, 5),
            Edge::between(5, 1),
        ];

        let edges = edges_on_0.iter()
            .chain(edges_on_3.iter())
            .chain(&edges_around)
            .map(|e| *e);

        let corners_on_0 = Face::from(0).adjacent_corners();
        let corners_on_3 = Face::from(3).adjacent_corners();

        let corners = corners_on_0.iter()
            .chain(&corners_on_3)
            .map(|c| *c);

        Self {
            edges: array_collect!(edges, [Edge; 12]),
            corners: array_collect!(corners, [Corner; 8]),
        }
    }

    pub fn is_solved(&self) -> bool {
        *self == Self::solved()
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

    pub fn corner_at_mut(&mut self, position: CornerPosition) -> &mut Corner {
        match self.corners.iter_mut().find(|c| c.is_at(position)) {
            Some(c) => c,
            None => unreachable!("no corner at {:?}", position)
        }
    }

    pub fn get_face(&self, position: CubePosition) -> FaceMatrix {
        let CubePosition { front: f, bottom: d } = position;

        let mut adjacent_clockwise = f.adjacent_clockwise();
        let mid = adjacent_clockwise.iter()
            .position(|x| *x == d).unwrap();
        adjacent_clockwise.rotate_left(mid);

        let [d, l, u, r] = adjacent_clockwise;

        macro_rules! at_on {
            ($f0: expr, $f1: expr) => {
                self.edge_at(pos!($f0, $f1)).id_on(f)
            };
            ($f0: expr, $f1: expr, $f2: expr) => {
                self.corner_at(pos!($f0, $f1, $f2)).id_on(f)
            };
        }

        FaceMatrix([
            [at_on!(f, l, u), at_on!(f, u), at_on!(f, r, u)],
            [at_on!(f, l)   , f           , at_on!(f, r)   ],
            [at_on!(f, l, d), at_on!(f, d), at_on!(f, r, d)],
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
                edge.transpose_pos(cpos!(face, missing), cpos!(face, next));
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
                corner.transpose_pos(cpos!(face, missing), cpos!(face, next));
            });
    }

    pub fn apply(&mut self, algorithm: &Algorithm) {
        for m in algorithm.iter() {
//            println!("applying {:?}", m);
            self.apply_move(m)
        }
    }
}

// iteration
#[allow(dead_code)]
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

macro_rules! impl_cube_fmt {
    ($trait: ty, $fmt: expr) => {
        impl $trait for Cube {
            fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
                macro_rules! format_face {
                    ($face: expr, $below: expr) => {
                        format!($fmt, self.get_face(cpos!($face, $below)))
                    };
                }

                macro_rules! push_right {
                    ($face: expr) => {
                        $face.lines().map(|l| {
                            vec!["       ", l].join("")
                        }).collect::<Vec<_>>().join("\n");
                    }
                }

                // FACES (for printing purposes)
                //   a
                // b c d e
                //   f
                // ACTUAL FACES (current representation)
                //   3
                // 5 1 2 4
                //   0

                let face_a = push_right!(format_face!(0, 1));
                let face_b = format_face!(5, 3);
                let face_c = format_face!(1, 3);
                let face_d = format_face!(2, 3);
                let face_e = format_face!(4, 3);
                let face_f = push_right!(format_face!(3, 4));

                let central_band = face_b.lines()
                    .zip(face_c.lines())
                    .zip(face_d.lines())
                    .zip(face_e.lines())
                    .map(|(((b, c), d), e)|
                        vec![b, c, d, e].join("  ")
                    ).join("\n");

                writeln!(f, "{}\n", face_a)?;
                writeln!(f, "{}\n", central_band)?;
                writeln!(f, "{}", face_f)
            }
        }
    }
}

impl_cube_fmt!(Debug, "{:?}");
impl_cube_fmt!(Display, "{}");