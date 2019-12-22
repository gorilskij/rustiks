use crate::cube::piece::face::Face;
use std::fmt::{Debug, Display, Formatter, Error};
use std::iter::once;

#[derive(Copy, Clone)]
pub struct CubePosition {
    pub front: Face,
    pub down: Face,
}

// TODO: re-privatize fields and check validity on creation
// TODO: also for EdgePosition and CornerPosition
impl CubePosition {
    pub fn projection(&self) -> [Face; 6] {
        let mut mid = self.front.adjacent_clockwise();

        let len = mid.len();
        let index = mid
            .iter()
            .position(|x| *x == self.down)
            .unwrap();

        mid.rotate_left((index + 3) % len);

        // this is ugly, TODO: improve
        let opposite = self.front.opposite();
        let iterator = once(&self.front)
            .chain(&mid)
            .chain(once(&opposite))
            .copied();

        array_collect!(iterator, [Face; 6])
    }
}

impl Debug for CubePosition {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "<f{:?}d{:?}>", self.front, self.down)
    }
}

impl Display for CubePosition {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "<f{}d{}>", self.front, self.down)
    }
}