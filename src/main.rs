use crate::cube::position::Position;
use crate::cube::transpose::Transpose;
use crate::cube::face::Face;
use crate::cube::{Edge, Corner};
use crate::cube::algorithm::Algorithm;

#[macro_use]
mod support;

#[macro_use]
mod cube;
mod test;

fn main() {
    let alg = Algorithm::from("F U U2 R2 B' F2");
    println!("{:?}", alg);
    let sim = alg.simplified();
    println!("{:?}", sim);
}