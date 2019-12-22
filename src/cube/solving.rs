extern crate itertools;

use itertools::Itertools;

use crate::cube::Cube;
use crate::cube::algorithm::Algorithm;
use crate::algorithm_data::*;
use crate::cube::piece::face::Face;
use crate::cube::transpose::Transposed;

// NOTE: ..._default methods work on a default cube (down, front = 0, 5)
impl Cube {
    fn cross_solution_default(&self, down: Face) -> Algorithm {
        let default = pos!(0, 5);

        let adjacent = down.adjacent();
        let mut algs = vec![];

        for order in adjacent.iter().permutations(adjacent.len()) {
            let mut tester = *self;
            let mut order_alg = Algorithm::new();

            for &front in order {
                let edges = tester.iter_edges()
                    .filter(|edge| edge.id_contains(down))
                    .map(|e|
                        e.transposed(cpos!(down, front), cpos!(0, 5)))
                    .collect::<Vec<_>>();

                let position = edges.iter()
                    .find(|e| e.has_id(default))
                    .unwrap_or_else(|| panic!("didn't find piece with id '{:?}'", default))
                    .pos;

                let alg = cross_data()[&position]
                    .eval_by(|pos|
                        pos.iter().any(|&p|
                            edges.iter().any(|e|
                                e.is_at(p) && e.is_solved()
                            )
                        )
                    )
                    .transposed(cpos!(0, 5), cpos!(down, front));

                tester.apply(&alg);
                order_alg.push(alg);
            }

            algs.push(order_alg.simplified());
        }

        algs.into_iter()
            .min_by(|a, b| a.len().cmp(&b.len()))
            .unwrap()
    }

    #[allow(dead_code)]
    pub fn solution(&self) -> Algorithm {
        let cross_alg = self.cross_solution_default(Face::new(0));

//        unimplemented!()
        cross_alg
    }
}