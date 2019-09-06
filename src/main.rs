#![allow(unused_imports)]

use cube::transpose::{Transpose, Transposed};
use cube::Cube;
use cube::algorithm::Algorithm;
use std::process::exit;
use crate::cube::piece::face::Face;
use crate::cube::piece::Piece;

#[macro_use]
mod support;

#[macro_use]
mod cube;
mod test;

// TODO: implement move_to for pieces

fn main() {
    let mut cube = Cube::solved();

    for piece in cube.iter_corners_mut() {
        piece.transpose_pos(position!(0, 5), position!(2, 0))
    }

    println!("{:?}", cube);
}