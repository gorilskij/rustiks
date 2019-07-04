pub use crate::cube::piece::{Edge, Corner};

pub mod face;
pub mod position;
pub mod piece;
pub mod transpose;

pub struct Cube {
    edges: Vec<Edge>,
    corners: Vec<Corner>
}

impl Cube {
//    pub fn solved() -> Self {
//        Self {
//
//        }
//    }
}