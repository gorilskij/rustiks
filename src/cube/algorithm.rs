use itertools::Itertools;
use std::fmt::{Display, Formatter, Error, Debug};
use std::iter::FromIterator;
use super::piece::face::Face;
use std::hint::unreachable_unchecked;

#[macro_export]
macro_rules! alg {
    ($str:expr) => {
        crate::cube::algorithm::Algorithm::from($str)
    };
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum MoveType {
    L, R, U, D, F, B
}

impl From<char> for MoveType {
    fn from(c: char) -> Self {
        match c {
            'L' => MoveType::L,
            'R' => MoveType::R,
            'U' => MoveType::U,
            'D' => MoveType::D,
            'F' => MoveType::F,
            'B' => MoveType::B,
            _ => panic!("invalid move type '{}'", c),
        }
    }
}

impl MoveType {
    fn base_move(&self) -> Self {
        match self {
            MoveType::L | MoveType::R => MoveType::L,
            MoveType::U | MoveType::D => MoveType::D,
            MoveType::F | MoveType::B => MoveType::B,
        }
    }

    fn opposite(&self) -> Self {
        match self {
            MoveType::L => MoveType::R,
            MoveType::R => MoveType::L,
            MoveType::U => MoveType::D,
            MoveType::D => MoveType::U,
            MoveType::F => MoveType::B,
            MoveType::B => MoveType::F,
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
    fn reversed(&self) -> Self {
        Self(self.0, match self.1 {
            1 => 3,
            2 => 2,
            3 => 1,
            _ => unsafe { unreachable_unchecked() }
        })
    }

    fn base_move(&self) -> MoveType {
        self.0.base_move()
    }

    // TODO: maybe implement some cube state where 'R' isn't always the same face
    pub fn face(&self) -> Face {
        match self.0 {
            MoveType::U => 0,
            MoveType::L => 5,
            MoveType::F => 1,
            MoveType::R => 2,
            MoveType::B => 4,
            MoveType::D => 3,
        }.into()
    }

    pub fn times(&self) -> u8 {
        self.1
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:?}{}", self.0, match self.1 {
            1 => "",
            2 => "2",
            3 => "'",
            _ => unsafe { unreachable_unchecked() }
        })
    }
}

impl Debug for Move {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "Rot(face: {:?}, times: {})", self.face(), self.1)
    }
}

#[derive(Eq, PartialEq)]
pub struct Algorithm(Vec<Move>);

impl From<&str> for Algorithm {
    fn from(s: &str) -> Self {
        Self(s.split_whitespace().map(|s| Move::from(s)).collect())
    }
}

impl FromIterator<Move> for Algorithm {
    fn from_iter<I: IntoIterator<Item=Move>>(iter: I) -> Self {
        Self(Vec::from_iter(iter))
    }
}

#[allow(dead_code)]
impl Algorithm {
    pub fn reversed(&self) -> Self {
        let reversed_iter = self.0
            .iter()
            .rev()
            .map(|m| m.reversed());
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
                        _ => unsafe { unreachable_unchecked() }
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

impl IntoIterator for Algorithm {
    type Item = Move;
    type IntoIter = <Vec<Move> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Algorithm {
    type Item = &'a Move;
    type IntoIter = <&'a Vec<Move> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl Debug for Algorithm {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.0.iter().join(" "))
    }
}