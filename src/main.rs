#![feature(const_generics)]
#![allow(incomplete_features)]

#![allow(unused_macros)]
#![allow(unused_imports)]

//#![warn(clippy::pedantic)]
//#![warn(clippy::style)]

//use cube::transpose::{Transpose, Transposed};
//use cube::Cube;
//use cube::algorithm::Algorithm;
//use std::process::exit;
//use crate::cube::piece::face::Face;
//use crate::cube::piece::Piece;
//
//use std::fs::File;
//use std::io::Read;
////use algorithm_data::{load_cross, load_f2l, load2};
//use std::hash::Hash;
//use itertools::Itertools;
//use std::collections::HashMap;
////use crate::cube::piece::position::EdgePosition;
//use crate::support::Tern;
//use crate::algorithm_data::{cross_data, f2l_data, oll_data, pll_data};

use crate::cube::Cube;

#[macro_use]
mod support;

#[macro_use]
mod cube;
mod test;

mod algorithm_data;

// NOTE: CURRENT CUBE PRINTING REPRESENTATION:
//    0
//  5 1 2 4
//    3

fn main() {
    better_panic::install();
//    println!("{}", Cube::solved());
//    println!("{}", Cube::solved().colored());

//    let mut s = String::new();
//    let text = File::open("src/algorithm_data/data/cross.txt")
//        .expect("failed to open file")
//        .read_to_string(&mut s);

//    cross_data();
//    f2l_data();
//    oll_data();
//    pll_data();




    let mut cube = Cube::new_solved();

    cube.apply(&alg!(U));
    println!("{}", cube.colored());

    let solution = cube.solution();
    println!("solution: \"{:?}\"", solution);

    cube.apply(&solution);
    println!("{}", cube.colored());




//    let mut cube = Cube::solved();
//    cube.apply(&alg!("U R R' U'"));
//    println!("{}", cube.colored());

//    cube.rotate_corners_at(pos!(0, 1, 2), pos!(3, 4, 5));
//    cube.flip_edges_at(pos!(1, 5), pos!(1, 3));
//    println!("{:?}", cube.colored());
//    cube.transpose(cpos!(1,3), cpos!(2,3));
//    println!("{:?}", cube.colored());
}