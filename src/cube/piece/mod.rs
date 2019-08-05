use std::fmt::{Debug, Display};
use crate::cube::transpose::Transpose;

#[macro_use]
pub mod position;

#[macro_use]
pub mod edge;

#[macro_use]
pub mod corner;

#[macro_export]
#[macro_use]
pub mod face;

pub trait Piece: Debug + Display + Transpose {

}




// TODO: reimplement displays and debugs in terms of positions