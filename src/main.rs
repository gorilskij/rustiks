use crate::cube::transpose::{Transpose, Transposed};
use crate::cube::face::Face;
use crate::cube::{Edge, Corner, Cube};
use crate::cube::algorithm::Algorithm;

#[macro_use]
mod support;

#[macro_use]
mod cube;
mod test;

// TODO: implement move_to for pieces

fn main() {
//    println!("{:?}", face!(5).transpose(position!(0, 5), position!(5, 0)))

    let mut cube = Cube::solved();
    println!("{:?}", cube.get_face(0.into(), 1.into()));
    println!("{:?}", cube.iter_pieces().next().unwrap());

    for piece in cube.iter_pieces_mut() {
        piece.transpose(position!(0, 5), position!(5, 0))
    }

    println!();
    println!("{:?}", cube.get_face(0.into(), 1.into()));
    println!("{:?}", cube.iter_pieces().next().unwrap());
}