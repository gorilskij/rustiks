use crate::cube::piece::face::Face;
use std::fmt::{Debug, Display, Formatter, Error};
use std::iter::once;

#[derive(Copy, Clone)]
pub struct CubePosition {
    pub front: Face,
    pub bottom: Face,
}

// TODO: re-privatize fields and check validity on creation
// TODO: also for EdgePosition and CornerPosition
impl CubePosition {
    pub fn projection(&self) -> [Face; 6] {
        let mut mid = self.front.adjacent_clockwise();

        let len = mid.len();
        let index = mid
            .iter()
            .position(|x| *x == self.bottom)
            .unwrap();

        mid.rotate_left((index + 3) % len);

        // this is ugly, TODO: improve
        let opposite = self.front.opposite();
        let iterator = once(&self.front)
            .chain(&mid)
            .chain(once(&opposite))
            .map(|x| *x);

        array_collect!(iterator, [Face; 6])
    }
}

impl Debug for CubePosition {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "<f{:?}b{:?}>", self.front, self.bottom)
    }
}

impl Display for CubePosition {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "<f{}b{}>", self.front, self.bottom)
    }
}