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
use std::ops::{Deref, Index};
use crate::cube::transpose::{Projection, Transpose};
use std::iter::once;
use std::fmt::{Debug, Formatter, Error};

#[derive(Copy, Clone)]
pub struct Position<const N: usize>(pub [Face; N]);


// this is bad, TODO: derive these traits when const generics are fully implemented
impl<const N: usize> PartialEq for Position<N> {
    fn eq(&self, other: &Self) -> bool {
        self.iter().zip(other.iter()).all(|(a, b)| a == b)
    }
}

impl<const N: usize> Eq for Position<N> {}

impl<const N: usize> Debug for Position<N> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "[")?;
        for x in self.iter() {
            write!(f, "{}, ", x)?
        }
        writeln!(f, "]")
    }
}
// end of bad shit













impl<T: Into<Face>, const N: usize> From<[T; N]> for Position<N> {
    fn from(array: [T; N]) -> Self {
        todo!()
//        Self(array_collect!(array.iter().copied().map(Into::into)))
    }
}

impl<const N: usize> Deref for Position<N> {
    type Target = [Face];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const N: usize> Position<N> {
    // TODO: remove in favor of deref
    pub fn iter(&self) -> impl Iterator<Item=&Face> {
        self.0.iter()
    }

    pub fn sorted(mut self) -> Self {
        self.0.sort();
        self
    }

    pub fn has(self, face: Face) -> bool {
        self.0.contains(&face)
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

impl<const N: usize> Transpose for Position<N> {
    fn transpose_with_projection(&mut self, from: Projection, to: Projection) {
        for x in &mut self.0 as &mut[Face] {
            x.transpose_with_projection(from, to)
        }
    }
}

impl<const N: usize> Position<N> {
    // fucked up because N - 1 and N - 1 are different types, TODO: clean up
    pub fn without(&self, face: Face) -> Position<{N - 1}> {
//        unsafe { std::mem::transmute( Position(array_collect!(self.iter().filter(|&&x| x != face).copied(), [Face; {N - 1}])) ) }
        let mut new = [Face::new(0); {N - 1}];
        self.iter().filter(|&&x| x != face).copied().enumerate().for_each(|(i, f)| new[i] = f);
        Position(new)
    }
}

impl<const N: usize> Index<usize> for Position<N> {
    type Output = Face;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}