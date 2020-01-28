use itertools::Itertools;
use std::fmt::{Display, Formatter, Error, Debug};
use std::iter::FromIterator;
use crate::cube::transpose::{Transpose, Projection};
use crate::cube::face::Face;

#[macro_export]
macro_rules! alg {
    ($alg:expr) => {
        $crate::cube::algorithm::Alg::from($alg)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum MoveType {
    L, R, U, D, F, B
}

impl From<char> for MoveType {
    fn from(c: char) -> Self {
        use MoveType::*;
        match c {
            'L' => L,
            'R' => R,
            'U' => U,
            'D' => D,
            'F' => F,
            'B' => B,
            _ => panic!("invalid move type '{}'", c),
        }
    }
}

impl MoveType {
    fn base_move(self) -> Self {
        use MoveType::*;
        match self {
            L | R => L,
            U | D => D,
            F | B => B,
        }
    }

    fn opposite(self) -> Self {
        use MoveType::*;
        match self {
            L => R, R => L,
            U => D, D => U,
            F => B, B => F,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Move(MoveType, u8);

impl From<&str> for Move {
    fn from(s: &str) -> Self {
        let mut chars = s.chars();

        let move_type = MoveType::from(
            chars.next().unwrap_or_else(|| panic!("move must have a length of at least 1"))
        );

        let quantifier = match chars.next() {
            None => 1,
            Some('2') => 2,
            Some('\'') => 3,
            Some(c) => panic!("invalid character '{}' as move quantifier", c),
        };

        Self(move_type, quantifier)
    }
}

impl Move {
    fn reversed(self) -> Self {
        Self(self.0, match self.1 {
            1 => 3,
            2 => 2,
            3 => 1,
            _ => unreachable!()
        })
    }

    fn base_move(self) -> MoveType {
        self.0.base_move()
    }

    // TODO: maybe implement some cube state where 'R' isn't always the same face
    pub fn face(self) -> Face {
        match self.0 {
            MoveType::U => 0,
            MoveType::L => 5,
            MoveType::F => 1,
            MoveType::R => 2,
            MoveType::B => 4,
            MoveType::D => 3,
        }.into()
    }

    pub fn times(self) -> u8 {
        self.1
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:?}{}", self.0, match self.1 {
            1 => "",
            2 => "2",
            3 => "'",
            _ => unreachable!()
        })
    }
}

impl Debug for Move {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "Rot(face: {:?}, times: {})", self.face(), self.1)
    }
}

#[derive(Eq, PartialEq, Clone)]
pub struct Alg(Vec<Move>);

impl<S: AsRef<str>> From<S> for Alg {
    fn from(s: S) -> Self {
        Self(s.as_ref().split_whitespace().map(Move::from).collect())
    }
}


//impl From<&str> for Algorithm {
//    fn from(s: &str) -> Self {
//        Self(s.split_whitespace().map(|s| Move::from(s)).collect())
//    }
//}

//impl From<String> for Algorithm {
//    fn from(s: String) -> Self {
//        Self::from(s.as_str())
//    }
//}

impl FromIterator<Move> for Alg {
    fn from_iter<I: IntoIterator<Item=Move>>(iter: I) -> Self {
        Self(Vec::from_iter(iter))
    }
}

#[allow(dead_code)]
impl Default for Alg {
    fn default() -> Self {
        Self(Vec::new())
    }
}

#[allow(dead_code)]
impl Alg {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn push(&mut self, mut alg: Self) {
        self.0.append(&mut alg.0)
    }

    pub fn reversed(&self) -> Self {
        let reversed_iter = self.0
            .iter()
            .copied()
            .rev()
            .map(Move::reversed);
        Self(reversed_iter.collect())
    }

    pub fn simplified(&self) -> Self {
        let mut another_pass = false;

        let processed = self.to_owned().0
            .iter()
            // group moves by base move ([[U D] [F B F'] ...])
            .batching(|it| {
                let first = match it.next() {
                    None => return None,
                    Some(m) => m
                };

                let base_move = first.base_move();

                let mut ret_vec = vec![first];
                ret_vec.extend(
                    it.peeking_take_while(|m| m.base_move() == base_move)
                );

                Some((base_move, ret_vec))
            })
            // simplify ([F B F' B2] => [B'])
            .flat_map(|(base_move, group)| {
                let opposite_move = base_move.opposite();
                let (mut base_sum, mut opposite_sum) = (0, 0);

                for m in group {
                    match m.0 {
                        t if t == base_move =>
                            base_sum = (base_sum + m.1) % 4,
                        t if t == opposite_move =>
                            opposite_sum = (opposite_sum + m.1) % 4,
                        _ => unreachable!()
                    }
                }

                let mut vec = vec![];

                if base_sum != 0 { vec.push(Move(base_move, base_sum)) }
                if opposite_sum != 0 { vec.push(Move(base_move.opposite(), opposite_sum)) }

                // e.g. "U F F' U'" => "U U'" requires another pass
                if base_sum == 0 && opposite_sum == 0 { another_pass = true }

                vec
            })
            .collect::<Self>();

        // TODO: switch to loop (previous attempt yielded infinite loop weirdness)
        // possible recursive second pass for situations like (R U U' R')
        if another_pass { processed.simplified() } else { processed }
    }
}

impl IntoIterator for Alg {
    type Item = Move;
    type IntoIter = <Vec<Move> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Alg {
    type Item = &'a Move;
    type IntoIter = <&'a Vec<Move> as IntoIterator>::IntoIter;

    #[must_use]
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl Debug for Alg {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.0.iter().join(" "))
    }
}

// WARN: unfinished and probably wrong
impl Transpose for Alg {
    fn transpose_with_projection(&mut self, from: Projection, to: Projection) {

        eprintln!("warning: <Alg as Transpose>::transpose_with_projection skipped");
        return;

//        todo!("hi, this is probably very wrong,\
//            both in style and in function, especially \
//            the letter array, please refer to the main \
//            file current cube printing representation");
        const M: [MoveType; 6] = {
            use MoveType::*;
            [D, L, B, U, R, F] // TODO: understand where this is from
        };

        *self = self.0.iter().map(|mov| {
            let index1 = Face::new(M.iter()
                .position(|&m| m == mov.0)
                .unwrap() as u8);

            let index2 = from.iter()
                .position(|&f| f == index1)
                .unwrap();

            Move(M[to[index2].unwrap() as usize], mov.1)
        }).collect()
    }
}