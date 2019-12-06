pub use piece::{edge::Edge, corner::Corner};

use piece::face::Face;

use std::fmt::{Debug, Formatter, Error, Display};
use crate::cube::transpose::{Transpose, Projection};
use piece::position::{EdgePosition, CornerPosition};
use itertools::Itertools;
use crate::cube::algorithm::{Algorithm, Move};
use crate::cube::piece::Piece;
use crate::cube::piece::position::CubePosition;
use std::iter::once;

#[macro_use]
pub mod piece;

pub mod transpose;
mod resort;

#[macro_use]
pub mod algorithm;

mod manipulation;
mod color;
mod solving;

#[derive(Eq, PartialEq, Copy, Clone)]
pub struct Cube {
    edges: [Edge; 12],
    corners: [Corner; 8],
}

pub struct FaceMatrix([[Face; 3]; 3]);

impl FaceMatrix {
    fn from(cube: &Cube, f: Face, d: Face, l: Face, u: Face, r: Face) -> Self {
        macro_rules! f {
            ($f1:expr) => {
                cube.edge_at(pos!(f, $f1)).id_on(f)
            };
            ($f1:expr, $f2:expr) => {
                cube.corner_at(pos!(f, $f1, $f2)).id_on(f)
            };
        }

        Self([
            [f!(l, u), f!(u), f!(r, u)],
            [f!(l)   , f    , f!(r)   ],
            [f!(l, d), f!(d), f!(r, d)],
        ])
    }
}

macro_rules! impl_face_matrix_fmt {
    ($trait:ty, $fmt:expr) => {
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

impl_face_matrix_fmt!(Debug, "{:?} {:?} {:?}");
impl_face_matrix_fmt!(Display, "{} {} {}");

// TODO: test printing
// TODO: implement a way to numerically input a cube
// TODO: implement a way to textually input a cube, test
// TODO: test algorithm application on cube

static mut SOLVED_CUBE: Option<Cube> = None;

impl Cube {
    fn generate_solved() -> Self {
        let edges_on_0 = Face::from(0).adjacent_edges();
        let edges_on_3 = Face::from(3).adjacent_edges();

        let adjacent = Face::from(0).adjacent();
        let edges_around_iter = adjacent.iter()
            .map(|f| *f)
            .chain(once(adjacent[0]))
            .tuple_windows()
            .map(|(f0, f1)| edge!(f0, f1));
        let edges_around = array_collect!(edges_around_iter, [Edge; 4]);

        let edges_iter = edges_on_0.iter()
            .chain(&edges_on_3)
            .chain(&edges_around)
            .map(|e| *e);

        let corners_on_0 = Face::from(0).adjacent_corners();
        let corners_on_3 = Face::from(3).adjacent_corners();

        let corners_iter = corners_on_0.iter()
            .chain(&corners_on_3)
            .map(|c| *c);

        Self {
            edges: array_collect!(edges_iter, [Edge; 12]),
            corners: array_collect!(corners_iter, [Corner; 8]),
        }
    }

    pub fn solved() -> Self {
        unsafe {
            if SOLVED_CUBE.is_none() {
                SOLVED_CUBE = Some(Self::generate_solved())
            }
            SOLVED_CUBE.unwrap()
        }
    }

    #[allow(dead_code)]
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

    fn get_face_matrix(&self, position: CubePosition) -> FaceMatrix {
        let CubePosition { front: f, down: d } = position;

        let mut adjacent_clockwise = f.adjacent_clockwise();
        let mid = adjacent_clockwise
            .iter()
            .position(|x| *x == d)
            .unwrap();
        adjacent_clockwise.rotate_left(mid);

        let [d, l, u, r] = adjacent_clockwise;

        FaceMatrix::from(self, f, d, l, u, r)
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

    #[allow(dead_code)]
    pub fn apply(&mut self, algorithm: &Algorithm) {
        for m in algorithm {
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
    pub fn iter_corners_mut(&mut self) -> impl Iterator<Item=&mut Corner> {
        self.corners.iter_mut()
    }

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
    ($trait:ty, $fmt:expr) => {
        impl $trait for Cube {
            fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
                macro_rules! format_face {
                    ($face:expr, $below:expr) => {
                        format!($fmt, self.get_face_matrix(cpos!($face, $below)))
                    };
                }

                macro_rules! push_right {
                    ($face:expr) => {
                        $face.lines().map(|l| {
                            vec!["       ", l].join("")
                        }).collect::<Vec<_>>().join("\n");
                    }
                }

                // current representation
                //   0
                // 5 1 2 4
                //   3

                writeln!(f, "{}\n", push_right!(format_face!(0, 1)))?;

                let central_band = format_face!(5, 3).lines()
                    .zip(format_face!(1, 3).lines())
                    .zip(format_face!(2, 3).lines())
                    .zip(format_face!(4, 3).lines())
                    .map(|(((b, c), d), e)|
                        vec![b, c, d, e].join("  ")
                    ).join("\n");

                writeln!(f, "{}\n", central_band)?;

                writeln!(f, "{}", push_right!(format_face!(3, 4)))
            }
        }
    }
}

impl_cube_fmt!(Debug, "{:?}");
impl_cube_fmt!(Display, "{}");