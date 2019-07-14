use std::collections::HashMap;
use crate::cube::position::Position;
use crate::cube::face::Face;
use crate::cube::Edge;
use crate::cube::transpose::Transpose;
use crate::cube::algorithm::Algorithm;

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

fn test_algorithm_reverse() {
    assert_eq!(Algorithm::from("R' D L R2 U' B'").reversed(), Algorithm::from("B U R2 L' D' R"));
    assert_eq!(Algorithm::from("U2 F D U2 L' R F2").reversed(), Algorithm::from("F2 R' L U2 D' F' U2"));
    assert_eq!(Algorithm::from("R' F' D2 L B2 L2 R2 U F'").reversed(), Algorithm::from("F U' R2 L2 B2 L' D2 F R"));
    assert_eq!(Algorithm::from("R2 B' U' F2 D B F'").reversed(), Algorithm::from("F B' D' F2 U B R2"));
    assert_eq!(Algorithm::from("R F L2 R B2 F U' L2 R'").reversed(), Algorithm::from("R L2 U F' B2 R' L2 F' R'"));
    assert_eq!(Algorithm::from("D2 F R2 F U R D' L2 B").reversed(), Algorithm::from("B' L2 D R' U' F' R2 F' D2"));
    assert_eq!(Algorithm::from("F D U' B U' L B2 F L2").reversed(), Algorithm::from("L2 F' B2 L' U B' U D' F'"));
    assert_eq!(Algorithm::from("U B' D2 U2 L").reversed(), Algorithm::from("L' U2 D2 B U'"));
    assert_eq!(Algorithm::from("B' U2 L' U B2 L2 B' D").reversed(), Algorithm::from("D' B L2 B2 U' L U2 B"));
    assert_eq!(Algorithm::from("D' B' D R2 U2 L2 F").reversed(), Algorithm::from("F' L2 U2 R2 D' B D"));
}

fn test_algorithm_simplify() {
    // note: these tests depend on the base moves (tested with L, D, B (like Rubyks))
    assert_eq!(Algorithm::from("F U U2 R2 B' F2").simplified(), Algorithm::from("F U' R2 B' F2"));
    assert_eq!(Algorithm::from("R2 F2 D' B B' L'").simplified(), Algorithm::from("R2 F2 D' L'"));
    assert_eq!(Algorithm::from("R2 D2 D' L' F U").simplified(), Algorithm::from("R2 D L' F U"));
    assert_eq!(Algorithm::from("B2 L2 D U' R' U").simplified(), Algorithm::from("B2 L2 D U' R' U"));
    assert_eq!(Algorithm::from("R2 B L U L' L2").simplified(), Algorithm::from("R2 B L U L"));
    assert_eq!(Algorithm::from("B2 U2 B2 U2 U F'").simplified(), Algorithm::from("B2 U2 B2 U' F'"));
    assert_eq!(Algorithm::from("U2 F2 F' B D' U2").simplified(), Algorithm::from("U2 B F D' U2"));
    assert_eq!(Algorithm::from("D2 L' R B' F D'").simplified(), Algorithm::from("D2 L' R B' F D'"));
    assert_eq!(Algorithm::from("D' R B D2 R D").simplified(), Algorithm::from("D' R B D2 R D"));
    assert_eq!(Algorithm::from("D2 B2 B2 D L' D'").simplified(), Algorithm::from("D' L' D'"));
    assert_eq!(Algorithm::from("R L2 U' D F2 B R R2 R B' F2 D' U L L R'").simplified(), Algorithm::from(""))
}

pub fn test() {
    test_projection();
    test_transpose();
    test_adjacent();
    test_adjacent_edges();
    test_adjacent_corners();
    test_algorithm_reverse();
    test_algorithm_simplify();

    println!("ALL TESTS PASSED SUCCESSFULLY")
}