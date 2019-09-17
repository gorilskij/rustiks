//#![allow(unused_imports)]
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

// TODO: implement move_to for pieces

fn main() {
    let mut cube = Cube::solved();

//    for piece in cube.iter_corners_mut() {
//        piece.transpose_pos((0, 5).into(), (2, 0).into())
//    }

    cube.apply(&Algorithm::from("R"));
//    cube.iter_edges_mut().for_each(|e| {
//        println!("was {:?}", e);
//        e.transpose((0, 1).into(), (0, 2).into());
//        println!("is {:?}", e);
//        println!()
//    });

    println!("{:?}", cube);
}