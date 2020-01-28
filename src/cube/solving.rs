use itertools::Itertools;

use crate::cube::Cube;
use crate::cube::algorithm::Alg;
use crate::algorithm_data::*;
use crate::cube::transpose::Transposed;
use boolinator::Boolinator;
use crate::cube::face::Face;

// NOTE: ..._default methods work on a default cube (down, front = 0, 5)
impl Cube {
    fn cross_solution_default(&self, down: Face) -> Alg {
        let default_id = pos!(0, 5).sorted();

        let adjacent = down.adjacent();
        let mut algs = vec![];

        for order in adjacent.iter().permutations(adjacent.len()) {
            let mut tester = *self;
            let mut order_alg = Alg::new();

            for &front in order {
                // todo refactor search as transposed cube and piece_at
                let edges = tester.iter_edges()
                    .filter_map(|edge| edge.id().contains(&down)
                        .as_some(edge.transposed(pos!(down, front), pos!(0, 5))))
                    .collect::<Vec<_>>();

                let position = edges.iter()
                    .find(|e| *e.id() == default_id)
                    .unwrap_or_else(|| panic!("didn't find piece with id '{:?}'", default_id))
                    .pos();

                let alg = cross_data()[&position]
                    .eval_by(|pos|
                        pos.iter().any(|&p|
                            edges.iter().any(|e| {
                                //todo!("choose one of these two, pos: {:?}, p: {:?}, e; {:?}", pos, p, e)
                                let r1 = *e.pos() == p && e.is_solved();
                                let r2 = e.pos().sorted() == p && e.is_solved();

                                if r1 == r2 { r1 } else {
                                    println!("PRE-PANIC CONTEXT");
                                    println!("pos: {:?}", pos);
                                    println!("p: {:?}", p);
                                    println!("e: {:?}", e);
                                    panic!("{:?}   !=   {:?}", r1, r2);
                                }
                            })
                        )
                    )
                    .transposed(pos!(0, 5), pos!(down, front));

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
    pub fn solution(&self) -> Alg {
//        todo!()
        let cross_alg = self.cross_solution_default(Face::new(0));
        cross_alg
    }
}