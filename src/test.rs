use std::collections::HashMap;
use crate::cube::position::Position;
use crate::cube::face::Face;
use crate::cube::Edge;
use crate::cube::transpose::Transpose;
use crate::cube::algorithm::Algorithm;

// apply a testing macro with 2 arguments (e.g. assert_eq) to many pairs of inputs
macro_rules! apply_ab_tests {
    {$eq_fn: ident; $(($a: tt, $b: tt));*} => {
        $( $eq_fn!($a, $b) );*
    };
    {$($s: tt);*;} => {apply_ab_tests!{ $($s);* }} // for trailing `;`
}

// convert a list of ints to an array of faces
macro_rules! to_faces {
    [$($num: expr),*] => {
        [$(
            Face::new($num),
        )*]
    }
}

#[test]
fn test_projection() {
    macro_rules! assert_eq_projection {
        ([$($id: expr),*], [$($face: expr),*]) => {{
            let position: Position = position![$($id),*];
            let correct_projection: [Face; 6] = to_faces![$($face),*];

            assert_eq!(position.projection(), correct_projection)
        }}
    }

    apply_ab_tests! {
        assert_eq_projection;
        ([0, 1], [0, 2, 1, 5, 4, 3]);
        ([0, 2], [0, 4, 2, 1, 5, 3]);
        ([0, 4], [0, 5, 4, 2, 1, 3]);
        ([0, 5], [0, 1, 5, 4, 2, 3]);
        ([1, 0], [1, 5, 0, 2, 3, 4]);
        ([1, 2], [1, 0, 2, 3, 5, 4]);
        ([1, 3], [1, 2, 3, 5, 0, 4]);
        ([1, 5], [1, 3, 5, 0, 2, 4]);
        ([2, 0], [2, 1, 0, 4, 3, 5]);
        ([2, 1], [2, 3, 1, 0, 4, 5]);
        ([2, 3], [2, 4, 3, 1, 0, 5]);
        ([2, 4], [2, 0, 4, 3, 1, 5]);
        ([3, 1], [3, 5, 1, 2, 4, 0]);
        ([3, 2], [3, 1, 2, 4, 5, 0]);
        ([3, 4], [3, 2, 4, 5, 1, 0]);
        ([3, 5], [3, 4, 5, 1, 2, 0]);
        ([4, 0], [4, 2, 0, 5, 3, 1]);
        ([4, 2], [4, 3, 2, 0, 5, 1]);
        ([4, 3], [4, 5, 3, 2, 0, 1]);
        ([4, 5], [4, 0, 5, 3, 2, 1]);
        ([5, 0], [5, 4, 0, 1, 3, 2]);
        ([5, 1], [5, 0, 1, 3, 4, 2]);
        ([5, 3], [5, 1, 3, 4, 0, 2]);
        ([5, 4], [5, 3, 4, 0, 1, 2]);
    }
}

#[test]
fn test_transpose() {
    // TODO: implement more tests
    let edge: Edge = edge![0, 1, 3, 4];
    let moved = edge.transpose(position![2, 1], position![1, 2]);
    assert_eq!(moved, edge![2, 3, 5, 0])
}

#[test]
fn test_adjacent() {
    macro_rules! assert_eq_adjacent {
        ($face: expr, [$($fc: expr),*]) => {
            assert_eq!(face!($face).adjacent(), to_faces![$($fc),*])
        }
    }

    macro_rules! assert_eq_adjacent_clockwise {
        ($face: expr, [$($fc: expr),*]) => {
            assert_eq!(face!($face).adjacent_clockwise(), to_faces![$($fc),*])
        }
    }

    apply_ab_tests!{
        assert_eq_adjacent;
        (0, [1, 2, 4, 5]);
        (1, [0, 2, 3, 5]);
        (2, [0, 1, 3, 4]);
        (3, [1, 2, 4, 5]);
        (4, [0, 2, 3, 5]);
        (5, [0, 1, 3, 4]);
    }

    apply_ab_tests! {
        assert_eq_adjacent_clockwise;
        (0, [5, 4, 2, 1]);
        (1, [0, 2, 3, 5]);
        (2, [4, 3, 1, 0]);
        (3, [1, 2, 4, 5]);
        (4, [5, 3, 2, 0]);
        (5, [0, 1, 3, 4]);
    }
}

#[test]
fn test_adjacent_edges() {
    macro_rules! assert_adjacent_edges {
        ($face: expr, $edges: expr) => {{
            let edges: [[u8; 2]; 4] = $edges;
            for (e, c) in face!($face).adjacent_edges().iter().zip(edges.iter()) {
                let faces = e.as_ruby()[0];
                assert_eq!(faces, to_faces![c[0], c[1]])
            }
        }}
    }

    apply_ab_tests! {
        assert_adjacent_edges;
        (0, [[0, 1], [0, 2], [0, 4], [0, 5]]);
        (1, [[0, 1], [1, 2], [1, 3], [1, 5]]);
        (2, [[0, 2], [1, 2], [2, 3], [2, 4]]);
        (3, [[1, 3], [2, 3], [3, 4], [3, 5]]);
        (4, [[0, 4], [2, 4], [3, 4], [4, 5]]);
        (5, [[0, 5], [1, 5], [3, 5], [4, 5]]);
    }
}

#[test]
fn test_adjacent_corners() {
    macro_rules! assert_adjacent_corners {
        ($face: expr, $corners: expr) => {{
            let corners: [[u8; 3]; 4] = $corners;
            for (e, c) in face!($face).adjacent_corners().iter().zip(corners.iter()) {
                let faces = e.as_ruby()[0];
                assert_eq!(faces, to_faces!(c[0], c[1], c[2]));
            }
        }}
    }

    apply_ab_tests! {
        assert_adjacent_corners;
        (0, [[0, 1, 2], [0, 2, 4], [0, 4, 5], [0, 1, 5]]);
        (1, [[0, 1, 2], [1, 2, 3], [1, 3, 5], [0, 1, 5]]);
        (2, [[0, 1, 2], [1, 2, 3], [2, 3, 4], [0, 2, 4]]);
        (3, [[1, 2, 3], [2, 3, 4], [3, 4, 5], [1, 3, 5]]);
        (4, [[0, 2, 4], [2, 3, 4], [3, 4, 5], [0, 4, 5]]);
        (5, [[0, 1, 5], [1, 3, 5], [3, 4, 5], [0, 4, 5]]);
    }
}

#[test]
fn test_algorithm_reversed() {
    macro_rules! assert_eq_reversed {
        ($alg: expr, $rev: expr) => {
            assert_eq!(Algorithm::from($alg).reversed(), Algorithm::from($rev))
        }
    }

    apply_ab_tests!{
        assert_eq_reversed;
        ("R' D L R2 U' B'",          "B U R2 L' D' R");
        ("U2 F D U2 L' R F2",        "F2 R' L U2 D' F' U2");
        ("R' F' D2 L B2 L2 R2 U F'", "F U' R2 L2 B2 L' D2 F R");
        ("R2 B' U' F2 D B F'",       "F B' D' F2 U B R2");
        ("R F L2 R B2 F U' L2 R'",   "R L2 U F' B2 R' L2 F' R'");
        ("D2 F R2 F U R D' L2 B",    "B' L2 D R' U' F' R2 F' D2");
        ("F D U' B U' L B2 F L2",    "L2 F' B2 L' U B' U D' F'");
        ("U B' D2 U2 L",             "L' U2 D2 B U'");
        ("B' U2 L' U B2 L2 B' D",    "D' B L2 B2 U' L U2 B");
        ("D' B' D R2 U2 L2 F",       "F' L2 U2 R2 D' B D");
    }
}

#[test]
fn test_algorithm_simplified() {
    macro_rules! assert_eq_simplified {
        ($alg: expr, $sim: expr) => {
            assert_eq!(Algorithm::from($alg).simplified(), Algorithm::from($sim))
        }
    }
    
    // note: these tests depend on the base moves (tested with L, D, B (like Rubyks))
    apply_ab_tests!{
        assert_eq_simplified;
        ("F U U2 R2 B' F2",                         "F U' R2 B' F2");
        ("R2 F2 D' B B' L'",                        "R2 F2 D' L'");
        ("R2 D2 D' L' F U",                         "R2 D L' F U");
        ("B2 L2 D U' R' U",                         "B2 L2 D U' R' U");
        ("R2 B L U L' L2",                          "R2 B L U L");
        ("B2 U2 B2 U2 U F'",                        "B2 U2 B2 U' F'");
        ("U2 F2 F' B D' U2",                        "U2 B F D' U2");
        ("D2 L' R B' F D'",                         "D2 L' R B' F D'");
        ("D' R B D2 R D",                           "D' R B D2 R D");
        ("D2 B2 B2 D L' D'",                        "D' L' D'");
        ("R L2 U' D F2 B R R2 R B' F2 D' U L L R'", "");
    }
}