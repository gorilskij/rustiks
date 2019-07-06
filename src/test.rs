use std::collections::HashMap;
use crate::cube::position::Position;
use crate::cube::face::Face;
use crate::cube::Edge;
use crate::cube::transpose::Transpose;

fn test_projection() {
    let mut correct = HashMap::new();
    correct.insert([0, 1], [0, 2, 1, 5, 4, 3]);
    correct.insert([0, 2], [0, 4, 2, 1, 5, 3]);
    correct.insert([0, 4], [0, 5, 4, 2, 1, 3]);
    correct.insert([0, 5], [0, 1, 5, 4, 2, 3]);
    correct.insert([1, 0], [1, 5, 0, 2, 3, 4]);
    correct.insert([1, 2], [1, 0, 2, 3, 5, 4]);
    correct.insert([1, 3], [1, 2, 3, 5, 0, 4]);
    correct.insert([1, 5], [1, 3, 5, 0, 2, 4]);
    correct.insert([2, 0], [2, 1, 0, 4, 3, 5]);
    correct.insert([2, 1], [2, 3, 1, 0, 4, 5]);
    correct.insert([2, 3], [2, 4, 3, 1, 0, 5]);
    correct.insert([2, 4], [2, 0, 4, 3, 1, 5]);
    correct.insert([3, 1], [3, 5, 1, 2, 4, 0]);
    correct.insert([3, 2], [3, 1, 2, 4, 5, 0]);
    correct.insert([3, 4], [3, 2, 4, 5, 1, 0]);
    correct.insert([3, 5], [3, 4, 5, 1, 2, 0]);
    correct.insert([4, 0], [4, 2, 0, 5, 3, 1]);
    correct.insert([4, 2], [4, 3, 2, 0, 5, 1]);
    correct.insert([4, 3], [4, 5, 3, 2, 0, 1]);
    correct.insert([4, 5], [4, 0, 5, 3, 2, 1]);
    correct.insert([5, 0], [5, 4, 0, 1, 3, 2]);
    correct.insert([5, 1], [5, 0, 1, 3, 4, 2]);
    correct.insert([5, 3], [5, 1, 3, 4, 0, 2]);
    correct.insert([5, 4], [5, 3, 4, 0, 1, 2]);

    for f0 in 0i32..=5 {
        for f1 in 0..=5 {
            if f0 == f1 { continue }
            if (f0 - f1).abs() == 3 { continue }

            let key = [f0, f1];
            let pos = Position::new((f0 as u8).into(), (f1 as u8).into());
            let proj = pos.projection();

            let r = correct[&key];
            let rf: [Face; 6] = [
                (r[0] as u8).into(),
                (r[1] as u8).into(),
                (r[2] as u8).into(),
                (r[3] as u8).into(),
                (r[4] as u8).into(),
                (r[5] as u8).into(),
            ];

            if rf != proj {
                panic!(
                    "\nexpected {:?} => {:?}\ngot             => {:?}\n",
                    key, rf, proj
                )
            }
        }
    }
}

fn test_transpose() {
    let edge: Edge = edge![0, 1, 3, 4];
    let moved = edge.transpose(position![2, 1], position![1, 2]);
    assert_eq!(moved, edge![2, 3, 5, 0])
}

pub fn test() {
    test_projection();
    test_transpose();

    println!("ALL TESTS PASSED SUCCESSFULLY")
}