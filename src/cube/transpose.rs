use super::piece::{face::Face, position::Position};

pub type Projection = [Face; 6];

pub trait Transpose {
    fn transpose_with_projection(&mut self, from: Projection, to: Projection);

    fn transpose(&mut self, from: Position, to: Position){
        self.transpose_with_projection(from.projection(), to.projection())
    }

    fn transpose_from_default(&mut self, to: Position) {
        // TODO: check if 0, 5 or 5, 0
        self.transpose(position![0, 5], to)
    }
}

pub trait Transposed: Sized {
    fn transposed_with_projection(&self, from: Projection, to: Projection) -> Self;
    fn transposed(&self, from: Position, to: Position) -> Self;
    fn transposed_from_default(&self, to: Position) -> Self;
}

macro_rules! convert_methods {
    ( $( $old_name: ident ( $( $param: ident $type: ty ),* ) => $new_name: ident );* $(;)? ) => {
        $(
            fn $new_name(&self, $( $param: $type ),* ) -> Self {
                let mut clone = self.clone();
                clone.$new_name( $( $param ),* );
                clone
            }
        )*
    }
}

impl<T: Transpose + Clone + Sized> Transposed for T {
    convert_methods! {
        transpose_with_projection(from Projection, to Projection) => transposed_with_projection;
        transpose(from Position, to Position) => transposed;
        transpose_from_default(to Position) => transposed_from_default;
    }
}