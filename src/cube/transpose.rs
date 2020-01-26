use crate::cube::face::Face;
use crate::cube::position::{Pos, projection};

pub type Projection = [Face; 6];

pub trait Transpose {
    fn transpose_with_projection(&mut self, from: Projection, to: Projection);

    fn transpose(&mut self, from: Pos<2>, to: Pos<2>){
        self.transpose_with_projection(projection(from), projection(to))
    }
}

pub trait Transposed {
    fn transposed_with_projection(&self, from: Projection, to: Projection) -> Self;
    fn transposed(&self, from: Pos<2>, to: Pos<2>) -> Self;
}

macro_rules! convert_methods {
    ( $( $old_name:ident ( $( $param:ident:$type:ty ),* ) => $new_name:ident );* $(;)? ) => {
        $(
            fn $new_name(&self, $( $param:$type ),* ) -> Self {
                let mut clone = self.clone();
                clone.$old_name( $( $param ),* );
                clone
            }
        )*
    }
}

impl<T: Transpose + Clone> Transposed for T {
    convert_methods! {
        transpose_with_projection(from: Projection, to: Projection) => transposed_with_projection;
        transpose(from: Pos<2>, to: Pos<2>) => transposed;
    }
}