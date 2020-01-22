//mod cube_position;
//mod edge_position;
//mod corner_position;
//
//pub use cube_position::*;
//pub use edge_position::*;
//pub use corner_position::*;
//use std::hash::Hash;
//
//pub trait Position where Self: Hash {}
//
//impl Position for EdgePosition {}
//
//impl Position for CornerPosition {}

#[macro_export]
macro_rules! pos {
    ($( $x:expr ),*$(,)?) => {{
        // TODO: try $super::...
        use $crate::cube::piece::position::Position;
        Position::from([$( $x ),*])
    }};
}

use super::Face;
use std::ops::Deref;
use crate::cube::transpose::Projection;
use std::iter::once;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Position<const N: usize>(pub [Face; N]);

impl<T: Into<Face>, const N: usize> From<[T; N]> for Position<N> {
    fn from(array: [T; N]) -> Self {
        todo!()
//        Self(array_collect!(array.iter().copied().map(Into::into)))
    }
}

impl<const N: usize> Deref for Position<N> {
    type Target = [T; N];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn projection(position: Position<2>) -> Projection {
    let Position([front, down]) = position;

    let mut mid = front.adjacent_clockwise();

    let len = mid.len();
    let index = mid
        .iter()
        .position(|x| *x == down)
        .unwrap();

    mid.rotate_left((index + 3) % len);

    // this is ugly, TODO: improve
    let opposite = front.opposite();
    let iterator = once(&front)
        .chain(&mid)
        .chain(once(&opposite))
        .copied();

    array_collect!(iterator, [Face; 6])
}