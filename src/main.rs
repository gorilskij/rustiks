#![allow(unused_macros)]

#[allow(unused_imports)] use cube::transpose::{Transpose, Transposed};
#[allow(unused_imports)] use cube::Cube;
#[allow(unused_imports)] use cube::algorithm::Algorithm;
#[allow(unused_imports)] use std::process::exit;
#[allow(unused_imports)] use crate::cube::piece::face::Face;
#[allow(unused_imports)] use crate::cube::piece::Piece;

extern crate serde_json;

#[macro_use]
mod support;

#[macro_use]
mod cube;
mod test;

mod algorithm_data;

fn main() {
//    println!("{}", Cube::solved());
//    println!("{}", Cube::solved().colored());

    let alg = Algorithm::from("R U R' U'");
    println!("{:?}", alg);
    let ser = serde_json::to_string(&alg).unwrap();
    println!("{}", ser.chars().next().unwrap());
    println!("{:?}", ser);
    let de: Algorithm = serde_json::from_str(&ser).unwrap();
    println!("{:?}", de);
    println!("{}", alg == de);


//    let mut cube = Cube::solved();
//    cube.apply(&alg!("U R R' U'"));
//    println!("{}", cube.colored());

//    cube.rotate_corners_at(pos!(0, 1, 2), pos!(3, 4, 5));
//    cube.flip_edges_at(pos!(1, 5), pos!(1, 3));
//    println!("{:?}", cube.colored());
//    cube.transpose(cpos!(1,3), cpos!(2,3));
//    println!("{:?}", cube.colored());
}