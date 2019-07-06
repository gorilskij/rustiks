pub use crate::cube::piece::{Edge, Corner};
use crate::cube::face::Face;

#[macro_use]
pub mod piece;

pub mod face;
pub mod position;
pub mod transpose;
pub mod resort;

pub struct Cube {
    edges: Vec<Edge>,
    corners: Vec<Corner>
}

impl Cube {
//    pub fn solved() -> Self {
//        let mut edges = Vec::with_capacity(12);
////        let mut corners = Vec::with_capacity(8);
//
//        edges.append(&mut Face::new(0).adjacent_edges());
//        edges.append(&mut Face::new(3).adjacent_edges());
//        edges.push(Edge::default_from_nums(1, 2));
//        edges.push(Edge::default_from_nums(2, 4));
//        edges.push(Edge::default_from_nums(4, 5));
//        edges.push(Edge::default_from_nums(5, 1));
//
//        unimplemented!()
//    }
}