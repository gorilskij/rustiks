#[macro_export]
macro_rules! pos {
    ($( $x:expr ),*$(,)?) => {{
        use $crate::cube::position::Pos;
        let array = [$( $x.into() ),*];
        Pos(array)
    }};
}

use super::Face;
use std::ops::{Deref, Index, IndexMut};
use crate::cube::transpose::{Projection, Transpose};
use std::iter::{once, FromIterator};
use std::fmt::{Debug, Formatter, Error};
use std::hash::{Hash, Hasher};
use itertools::Itertools;

#[derive(Copy, Clone)]
pub struct Pos<const N: usize>(pub [Face; N]);

// todo derive when possible
impl Hash for Pos<2> { fn hash<H: Hasher>(&self, state: &mut H) { self.0.hash(state) } }
impl Hash for Pos<3> { fn hash<H: Hasher>(&self, state: &mut H) { self.0.hash(state) } }


// TODO derive these traits when const generics are fully implemented
impl<const N: usize> PartialEq for Pos<N> {
    fn eq(&self, other: &Self) -> bool {
        // todo switch to this when c.g. allow
        // <[_]>::eq(&self.0, &other.0)
        let a: &[_] = &self.0;
        let b: &[_] = &other.0;
        a == b
    }
}

impl<const N: usize> Eq for Pos<N> {}

impl<const N: usize> Debug for Pos<N> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{{{}}}", self.0.iter()
            .map(|f| format!("{:?}", f))
            .join(" ")
        )
    }
}

// TODO implement From<[T; N]>

impl<const N: usize> Deref for Pos<N> {
    type Target = [Face];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const N: usize> Pos<N> {
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

pub fn projection(position: Pos<2>) -> Projection {
    let Pos([front, down]) = position;

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

impl<const N: usize> Transpose for Pos<N> {
    fn transpose_with_projection(&mut self, from: Projection, to: Projection) {
        for x in &mut self.0 as &mut[Face] {
            x.transpose_with_projection(from, to)
        }
    }
}

impl<const N: usize> Pos<N> {
    // fucked up because N - 1 and N - 1 are different types, TODO: clean up
    pub fn without(&self, face: Face) -> Vec<Face> {
//        unsafe { std::mem::transmute( Position(array_collect!(self.iter().filter(|&&x| x != face).copied(), [Face; {N - 1}])) ) }
        let vec = self.iter()
            .copied()
            .filter(|&f| f != face)
            .collect::<Vec<_>>();
        assert_eq!(vec.len(), N - 1);
        vec
    }
}

impl<const N: usize> Index<usize> for Pos<N> {
    type Output = Face;

    fn index(&self, index: usize) -> &Self::Output { &self.0[index] }
}

impl<const N: usize> IndexMut<usize> for Pos<N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output { &mut self.0[index] }
}

// todo make generic when possible
impl<A: Into<Face>> FromIterator<A> for Pos<2> {
    fn from_iter<T: IntoIterator<Item=A>>(iter: T) -> Self {
        let mut faces = [Face::new(0); 2];
        let mut i = 0;
        for t in iter {
            faces[i] = t.into();
            i += 1;
        }
        assert_eq!(i, 2); // strict
        Self(faces)
    }
}
impl<A: Into<Face>> FromIterator<A> for Pos<3> {
    fn from_iter<T: IntoIterator<Item=A>>(iter: T) -> Self {
        let mut faces = [Face::new(0); 3];
        let mut i = 0;
        for t in iter {
            faces[i] = t.into();
            i += 1;
        }
        assert_eq!(i, 3); // strict
        Self(faces)
    }
}