use crate::cube::position::Position;
use crate::cube::transpose::Transpose;
use crate::cube::face::Face;

mod cube;

fn main() {
    let mut cor = cube::Corner::from_nums(0, 5, 1, 3, 4, 2);
    println!("{:?}", cor);
    cor = cor.transpose(
        Position::from(2, 3),
        Position::from(5, 0)
    );
    println!("{:?}", cor)
}