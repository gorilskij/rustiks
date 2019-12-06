//Moves = 'UDFBRL'.split('').flat_map {|i| [i, i + '2', i + "'"]}
extern crate serde;
extern crate serde_json;

use std::collections::HashMap;
use crate::cube::piece::face::Face;
use crate::support::Tern;
use crate::cube::piece::position::{EdgePosition, CornerPosition};
use crate::cube::algorithm::Algorithm;
use serde::Deserialize;
use std::path::Path;
use std::fs::File;
use std::io::{Read, BufReader, BufRead};
use std::str::{Chars, FromStr};
use itertools::Itertools;
use std::ops::Index;
use crate::support::IndexOf;
use std::hash::Hash;
use std::iter::FromIterator;
use std::fmt::Debug;
use std::process::exit;

const EXPECT_CHARS: &str = "chars ended earlier than expected";

fn extract_algorithm(chars: &mut impl Iterator<Item=char>) -> Algorithm {
    assert_eq!(chars.next().expect(EXPECT_CHARS), '"');
    let mut alg = String::new();
    loop {
        match chars.next().expect(EXPECT_CHARS) {
            '"' => break,
            c => alg.push(c),
        }
    }
    Algorithm::from(alg)
}

fn split_at_first(s: &str, c: char) -> (&str, &str) {
    let mut iter = s.splitn(2, c);
    let exp = &format!("failed to split at first '{}'", c);
    (iter.next().expect(exp), iter.next().expect(exp))
}

pub trait PieceKey {
    const LENGTH: usize;

    fn from_char_iter(iter: impl Iterator<Item=char>) -> Self;
}

pub(crate) fn load<P: AsRef<Path>, K>(path: P)
    -> HashMap<K, Tern<Vec<K>, Algorithm>> where
    K: PieceKey + Eq + Hash + Debug
{
    let file = File::open(path).expect("failed to open file");
    let mut reader = BufReader::new(file);

    let mut map = HashMap::new();

    for mut line in reader.lines()
        .map(|l| l.expect("failed to read line")) {

        if &line == "///" { break }
        if line.starts_with("//") { continue }

        let (pieces, mut line) = split_at_first(&line, ':');
        let pieces = K::from_char_iter(pieces.chars());

        let mut tern_vec =
            Vec::<(Vec<K>, Algorithm)>::with_capacity(10);
        while !line.is_empty() {
            let mut predicates = vec![];
            while line.chars().nth(0).unwrap().is_numeric() {
                let (pred, rest) = line.split_at(K::LENGTH);
                line = rest;
                predicates.push(K::from_char_iter(pred.chars()))
            }

            let end = line.index_of('"', 1)
                .expect("line ended unexpectedly");
            let (alg, rest) = line.split_at(end + 1);
            line = rest;
            let alg = Algorithm::from(&alg[1..alg.len() - 1]);

            tern_vec.push((predicates, alg));
        }

        let mut iter = tern_vec.into_iter().rev();
        let mut tern = Tern::End(iter.next().expect("Unexpected empty condition vector").1);
        for (con, alg) in iter {
            tern = Tern::Con(con, alg, Box::new(tern));
        }

        if let Some(prev) = map.insert(pieces, tern) {
            panic!("Duplicate key in map, previous value: {:?}", prev)
        }
    }

    map
}

impl PieceKey for EdgePosition {
    const LENGTH: usize = 2;

    fn from_char_iter(iter: impl Iterator<Item=char>) -> Self {
        iter.collect()
    }
}

pub(crate) fn load_cross<P: AsRef<Path>>(path: P)
    -> HashMap<EdgePosition, Tern<Vec<EdgePosition>, Algorithm>> {
    load(path)
}


type CEPosition = (CornerPosition, EdgePosition);

impl PieceKey for CEPosition {
    const LENGTH: usize = 5;

    fn from_char_iter(mut iter: impl Iterator<Item=char>) -> Self {
        let c = (&mut iter).take(3).collect();
        let e = iter.collect();
        (c, e)
    }
}

pub(crate) fn load_f2l<P: AsRef<Path>>(path: P)
    -> HashMap<CEPosition, Tern<Vec<CEPosition>, Algorithm>> {
    load(path)
}