use crate::cube::position::Position;
use crate::cube::transpose::Transpose;
use crate::cube::face::Face;
use crate::cube::{Edge, Corner};

#[macro_use]
mod support;

#[macro_use]
mod cube;
mod test;

fn main() {
    test::test();

//    let mut corner = Corner::from([
//        [3, 4, 5],
//        [1, 0, 5]
//    ]);
//
//    println!("{:?}", corner);
//
//    corner = corner.transpose(
//        Position::new(Face::new(1), Face::new(3)),
//        Position::new(Face::new(5), Face::new(0)),
//    );
//
//    println!("{:?}", corner)
}