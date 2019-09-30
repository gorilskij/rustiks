#![allow(unused_macros)]

#[allow(unused_imports)] use cube::transpose::{Transpose, Transposed};
#[allow(unused_imports)] use cube::Cube;
#[allow(unused_imports)] use cube::algorithm::Algorithm;
#[allow(unused_imports)] use std::process::exit;
#[allow(unused_imports)] use crate::cube::piece::face::Face;
#[allow(unused_imports)] use crate::cube::piece::Piece;

#[macro_use]
mod support;

#[macro_use]
mod cube;
mod test;

fn main() {
    let mut cube = Cube::solved();

    cube.rotate_corners_at(pos!(0, 1, 2), pos!(3, 4, 5), 1);
    cube.flip_edges_at(pos!(1, 5), pos!(1, 3));
    println!("{:?}", cube);
    cube.transpose(cpos!(1,3), cpos!(2,3));
    println!("{:?}", cube);
}