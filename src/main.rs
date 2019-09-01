use cube::transpose::{Transpose, Transposed};
use cube::Cube;
use cube::algorithm::Algorithm;
use std::process::exit;
use crate::cube::piece::face::Face;

#[macro_use]
mod support;

#[macro_use]
mod cube;
mod test;

// TODO: implement move_to for pieces

fn main() {
    let mut cube = Cube::solved();
    println!("{:?}", cube);
    for piece in cube.iter_pieces_mut() {
        piece.transpose(position!(0, 5), position!(5, 0))
    }
//    println!("{:?}", cube.get_face(0.into(), 1.into()));
//    println!("{:?}", cube.iter_pieces().next().unwrap());
//
//    for piece in cube.iter_pieces_mut() {
//        piece.transpose(position!(0, 5), position!(5, 0))
//    }
//
//    println!();
//    println!("{:?}", cube.get_face(0.into(), 1.into()));
//    println!("{:?}", cube.iter_pieces().next().unwrap());
}