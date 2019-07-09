pub use crate::cube::piece::{Edge, Corner};
use crate::cube::face::Face;
use std::slice::Iter;

#[macro_use]
pub mod piece;

#[macro_use]
pub mod face;

pub mod position;
pub mod transpose;
pub mod resort;

pub struct Cube {
    edges: [Edge; 12],
    corners: [Corner; 8],
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
            .chain(edges_around.iter())
            .map(|e| *e);

        let corners_on_0 = face!(0).adjacent_corners();
        let corners_on_3 = face!(3).adjacent_corners();

        let corners = corners_on_0.iter()
            .chain(corners_on_3.iter())
            .map(|c| *c);

        let edges = collect_to_array!(edges, [Edge; 12]);
        let corners = collect_to_array!(corners, [Corner; 8]);

        Self { edges, corners }
    }
}