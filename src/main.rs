use crate::cube::position::Position;
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

//fn main() {
////    println!("{:?}", face!(5).transpose(position!(0, 5), position!(5, 0)))
//
//    let mut cube = Cube::solved();
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
//}

fn main() {
    let edge = edge!(1, 3);
    println!("{:?}", edge);
    let pos_from = position!(1, 5);
    let pos_to = position!(5, 0);
    let trans = edge.transposed(pos_from, pos_to);
    println!("{:?}", trans);
}