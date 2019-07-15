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
    println!("{:?}", face!(5).transpose(position!(0, 5), position!(5, 0)))

//    let mut cube = Cube::solved();
//    println!("{:?}", cube.get_face(0.into(), 5.into()))
}