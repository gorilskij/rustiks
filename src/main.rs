use crate::cube::position::Position;
use crate::cube::transpose::Transpose;
use crate::cube::face::Face;
use crate::cube::{Edge, Corner, Cube};
use crate::cube::algorithm::Algorithm;

#[macro_use]
mod support;

#[macro_use]
mod cube;
mod test;

fn main() {
    let mut cube = Cube::solved();
//    println!("{:?}", cube.edge_at(position![0, 5]).id_on(face!(3)))
    println!("{:?}", cube.front_face())
}