use itertools::Itertools;

use crate::cube::Cube;
use crate::cube::algorithm::Alg;
use crate::algorithm_data::*;
use crate::cube::transpose::{Transposed, Transpose};
use boolinator::Boolinator;
use crate::cube::face::Face;

// NOTE: ..._default methods work on a default cube (down, front = 0, 5)
impl Cube {
    // todo return all solutions
    // note face, other used to be down, front (might be relevant)
    // note this might mean that solution tables can be described as "for this face, ..."
    //  removing all down-front strangeness from code
    // todo ^
    fn get_cross_alg(&self, face: Face) -> Alg {
        let default_id = pos!(0, 5);

//        for order in adjacent.iter().permutations(adjacent.len()) {
        // todo try in all permutations of order


        let mut test_cube = *self;
        let mut final_algorithm = Alg::new();

        for &other in &face.adjacent() {
            // todo refactor search as transposed cube and piece_at
            let edges = test_cube.iter_edges()
                .filter_map(|edge|
                    match edge.id().contains(&face) {
                        true => Some(edge.transposed(pos!(face, other), default_id)),
                        _ => None,
                    }
//                    edge.id()
//                        .contains(&down)
//                        .as_some(
//                            edge.transposed(pos!(face, other), pos!(0, 5))
//                        )
                )
                .collect::<Vec<_>>();

            let position_of_default = edges.iter()
                .find(|e| *e.id() == default_id)
                .expect(&format!("didn't find piece with default id '{:?}'", default_id))
                .pos();

            let mut alg = cross_data()[&position_of_default]
                .eval_by(|already_there|
                    already_there.iter().any(|&id|
                        edges.iter()
                            .find(|&&e| e.id() == &id)
                            .unwrap()
                            .is_solved()
                    )
                )
                .clone();

            alg.transpose(pos!(0, 5), pos!(face, other));
            final_algorithm.push(alg);

//            test_cube.apply(&alg);
//            alg.push(alg);
        }

//        algs.push(order_alg.simplified());

//        algs.into_iter()
//            .min_by(|a, b| a.len().cmp(&b.len()))
//            .unwrap()

        final_algorithm.simplified()
    }

    #[allow(dead_code)]
    pub fn solution(&self) -> Alg {
//        todo!()
        let cross_alg = self.get_cross_alg(Face::new(0));
        cross_alg
    }
}