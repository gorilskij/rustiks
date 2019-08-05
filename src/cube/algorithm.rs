use itertools::Itertools;
use std::fmt::{Display, Formatter, Error, Debug};
use std::iter::FromIterator;
use super::piece::face::Face;

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
            _ => panic!("invalid move type '{}'", c)
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
            Some(c) => panic!("invalid character '{}' as move quantifier", c)
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
            _ => panic!()
        })
    }

    fn base_move(&self) -> MoveType {
        self.0.base_move()
    }

    // TODO: maybe implement some cube state where 'R' isn't always the same face
    pub fn face(&self) -> Face {
        face!(
            match self.0 {
                MoveType::D => 0,
                MoveType::L => 1,
                MoveType::B => 2,
                MoveType::U => 3,
                MoveType::R => 4,
                MoveType::F => 5,
            }
        )
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
            _ => panic!()
        })
    }
}

impl Debug for Move {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self)
    }
}

#[derive(Eq, PartialEq)]
pub struct Algorithm(Vec<Move>);

impl From<&str> for Algorithm {
    fn from(s: &str) -> Self {
        Self(
            s.split_whitespace()
                .map(|s| Move::from(s))
                .collect()
        )
    }
}

impl FromIterator<Move> for Algorithm {
    fn from_iter<I: IntoIterator<Item=Move>>(iter: I) -> Self {
        Self(Vec::from_iter(iter))
    }
}

impl Algorithm {
    pub fn reversed(&self) -> Self {
        let mut move_reversed = self.0
            .iter()
            .map(|m| m.reversed())
            .collect::<Vec<_>>();
        move_reversed.reverse();
        Self(move_reversed)
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
                        _ => panic!()
                    }
                }

                let mut vec = vec![];

                if base_sum != 0 { vec.push(Move(base_move, base_sum)) }
                if opposite_sum != 0 { vec.push(Move(base_move.opposite(), opposite_sum)) }

                if base_sum == 0 && opposite_sum == 0 { another_pass = true }

                vec
            })
            .collect::<Self>();

        // TODO: switch to loop (previous attempt yielded infinite loop weirdness)
        // possible recursive second pass for situations like (R U U' R')
        if another_pass { processed.simplified() } else { processed }
    }

    pub fn iter(&self) -> AlgorithmIterator {
        AlgorithmIterator(0, &self.0)
    }
}

impl Debug for Algorithm {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.0
            .iter()
            .map(|m| m.to_string())
            .intersperse(" ".into())
            .collect::<String>()
        )
    }
}

impl IntoIterator for Algorithm {
    type Item = Move;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

pub struct AlgorithmIterator<'a>(usize, &'a [Move]);

impl<'a> Iterator for AlgorithmIterator<'a> {
    type Item = &'a Move;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 >= self.1.len() { return None }

        let m = &self.1[self.0];
        self.0 += 1;
        Some(m)
    }
}