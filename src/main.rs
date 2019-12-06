#![allow(unused_macros)]
#![feature(option_expect_none)]

#[allow(unused_imports)] use cube::transpose::{Transpose, Transposed};
#[allow(unused_imports)] use cube::Cube;
#[allow(unused_imports)] use cube::algorithm::Algorithm;
#[allow(unused_imports)] use std::process::exit;
#[allow(unused_imports)] use crate::cube::piece::face::Face;
#[allow(unused_imports)] use crate::cube::piece::Piece;

#[allow(unused_imports)] use std::fs::File;
#[allow(unused_imports)] use std::io::Read;
#[allow(unused_imports)] //use algorithm_data::{load_cross, load_f2l, load2};
#[allow(unused_imports)] use std::hash::Hash;
#[allow(unused_imports)] use itertools::Itertools;
#[allow(unused_imports)] use std::collections::HashMap;
#[allow(unused_imports)] use crate::cube::piece::position::EdgePosition;
#[allow(unused_imports)] use crate::support::Tern;
#[allow(unused_imports)] use crate::algorithm_data::{cross_data, f2l_data, oll_data, pll_data};

extern crate md5;

#[macro_use]
mod support;

#[macro_use]
mod cube;
mod test;

mod algorithm_data;

fn main() {
//    println!("{}", Cube::solved());
//    println!("{}", Cube::solved().colored());

//    let mut s = String::new();
//    let text = File::open("src/algorithm_data/data/cross.txt")
//        .expect("failed to open file")
//        .read_to_string(&mut s);

    cross_data();
    f2l_data();
    oll_data();
    pll_data();

//    Cube::solved().solve();
//    exit(0);

//    let cross = load_cross("src/algorithm_data/data/cross.txt");
//    let f2l = load_f2l("src/algorithm_data/data/f2l.txt");
//    let oll = load2("src/algorithm_data/data/oll.txt");
//    let pll = load2("src/algorithm_data/data/pll.txt");

//    let ca: CrossAlg = serde_json::from_str(&s).expect("failed to deserialize");

//    let mut cube = Cube::solved();
//    cube.apply(&alg!("U R R' U'"));
//    println!("{}", cube.colored());

//    cube.rotate_corners_at(pos!(0, 1, 2), pos!(3, 4, 5));
//    cube.flip_edges_at(pos!(1, 5), pos!(1, 3));
//    println!("{:?}", cube.colored());
//    cube.transpose(cpos!(1,3), cpos!(2,3));
//    println!("{:?}", cube.colored());
}