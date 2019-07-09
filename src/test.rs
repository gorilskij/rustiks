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

macro_rules! to_faces {
    [$($num: expr),*] => {
        [$(
            Face::new($num),
        )*]
    }
}

fn test_adjacent() {
    assert_eq!(face!(0).adjacent(), to_faces![1, 2, 4, 5]);
    assert_eq!(face!(1).adjacent(), to_faces![0, 2, 3, 5]);
    assert_eq!(face!(2).adjacent(), to_faces![0, 1, 3, 4]);
    assert_eq!(face!(3).adjacent(), to_faces![1, 2, 4, 5]);
    assert_eq!(face!(4).adjacent(), to_faces![0, 2, 3, 5]);
    assert_eq!(face!(5).adjacent(), to_faces![0, 1, 3, 4]);

    assert_eq!(face!(0).adjacent_clockwise(), to_faces![5, 4, 2, 1]);
    assert_eq!(face!(1).adjacent_clockwise(), to_faces![0, 2, 3, 5]);
    assert_eq!(face!(2).adjacent_clockwise(), to_faces![4, 3, 1, 0]);
    assert_eq!(face!(3).adjacent_clockwise(), to_faces![1, 2, 4, 5]);
    assert_eq!(face!(4).adjacent_clockwise(), to_faces![5, 3, 2, 0]);
    assert_eq!(face!(5).adjacent_clockwise(), to_faces![0, 1, 3, 4]);
}

macro_rules! assert_adjacent_edges {
    ($face: expr, $edges: expr) => {{
        let edges: [[u8; 2]; 4] = $edges;

        for (e, c) in face!($face).adjacent_edges().iter().zip(edges.iter()) {
            let faces = e.as_ruby();
            assert_eq!(faces[0][0], face!(c[0]));
            assert_eq!(faces[0][1], face!(c[1]))
        }
    }}
}

fn test_adjacent_edges() {
    assert_adjacent_edges!(0, [[0, 1], [0, 2], [0, 4], [0, 5]]);
    assert_adjacent_edges!(1, [[0, 1], [1, 2], [1, 3], [1, 5]]);
    assert_adjacent_edges!(2, [[0, 2], [1, 2], [2, 3], [2, 4]]);
    assert_adjacent_edges!(3, [[1, 3], [2, 3], [3, 4], [3, 5]]);
    assert_adjacent_edges!(4, [[0, 4], [2, 4], [3, 4], [4, 5]]);
    assert_adjacent_edges!(5, [[0, 5], [1, 5], [3, 5], [4, 5]]);
}

macro_rules! assert_adjacent_corners {
    ($face: expr, $corners: expr) => {{
        let corners: [[u8; 3]; 4] = $corners;

        for (e, c) in face!($face).adjacent_corners().iter().zip(corners.iter()) {
            let faces = e.as_ruby();
            assert_eq!(faces[0][0], face!(c[0]));
            assert_eq!(faces[0][1], face!(c[1]));
            assert_eq!(faces[0][2], face!(c[2]))
        }
    }}
}

fn test_adjacent_corners() {
    assert_adjacent_corners!(0, [[0, 1, 2], [0, 2, 4], [0, 4, 5], [0, 1, 5]]);
    assert_adjacent_corners!(1, [[0, 1, 2], [1, 2, 3], [1, 3, 5], [0, 1, 5]]);
    assert_adjacent_corners!(2, [[0, 1, 2], [1, 2, 3], [2, 3, 4], [0, 2, 4]]);
    assert_adjacent_corners!(3, [[1, 2, 3], [2, 3, 4], [3, 4, 5], [1, 3, 5]]);
    assert_adjacent_corners!(4, [[0, 2, 4], [2, 3, 4], [3, 4, 5], [0, 4, 5]]);
    assert_adjacent_corners!(5, [[0, 1, 5], [1, 3, 5], [3, 4, 5], [0, 4, 5]]);
}

pub fn test() {
    test_projection();
    test_transpose();
    test_adjacent();
    test_adjacent_edges();
    test_adjacent_corners();

    println!("ALL TESTS PASSED SUCCESSFULLY")
}