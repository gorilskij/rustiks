#[macro_export]
macro_rules! pos {
    ($f0:expr, $f1:expr) => {
        crate::cube::piece::position::EdgePosition(
            $f0.into(),
            $f1.into(),
        )
    };
    ($f0:expr, $f1:expr, $f2:expr) => {
        crate::cube::piece::position::CornerPosition(
            $f0.into(),
            $f1.into(),
            $f2.into(),
        )
    };
}

#[macro_export]
macro_rules! cpos {
    ($front:expr, $down:expr) => {
        crate::cube::piece::position::CubePosition {
            front: $front.into(),
            down: $down.into(),
        }
    };
}

mod cube_position;
mod edge_position;
mod corner_position;

pub use cube_position::*;
pub use edge_position::*;
pub use corner_position::*;