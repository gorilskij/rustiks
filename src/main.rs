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
//    println!("{}", Cube::solved());
//    println!("{}", Cube::solved().colored());
    let mut cube = Cube::solved();
    cube.apply(&alg!("U"));
    println!("{}", cube.colored());

//    cube.rotate_corners_at(pos!(0, 1, 2), pos!(3, 4, 5));
//    cube.flip_edges_at(pos!(1, 5), pos!(1, 3));
//    println!("{:?}", cube.colored());
//    cube.transpose(cpos!(1,3), cpos!(2,3));
//    println!("{:?}", cube.colored());
}