#![allow(unused_imports)]

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
    for piece in cube.iter_pieces_on_mut(0) {
        piece.transpose_pos(position!(0, 5), position!(2, 0))
    }
    println!("{:?}", cube);
}