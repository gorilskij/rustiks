//Moves = 'UDFBRL'.split('').flat_map {|i| [i, i + '2', i + "'"]}
extern crate serde;
extern crate serde_json;

use std::collections::HashMap;
use crate::cube::piece::face::Face;
use crate::support::Tern;
use crate::cube::piece::position::EdgePosition;
use crate::cube::algorithm::Algorithm;
use serde::Deserialize;
use std::path::Path;
use std::fs::File;
use std::io::{Read, BufReader, BufRead};
use std::str::Chars;
use itertools::Itertools;
use std::ops::Index;
use crate::support::IndexOf;

//#[derive(Deserialize)]
//pub struct CrossAlg {
//    piece: EdgePosition,
//    select: Face, // TODO: understand what this is and rename it
//    algorithms: HashMap<EdgePosition, Tern<Vec<EdgePosition>, Algorithm>>,
//}

const EXPECT_CHARS: &str = "chars ended earlier than expected";

// char to u8
fn c2u8(c: char) -> u8 {
    c.to_digit(10).expect("not a valid base-10 digit") as u8
}

macro_rules! cpos {
    ($c1:expr, $c2:expr) => { pos!(c2u8($c1), c2u8($c2)) };
}

fn extract_edge_position(chars: &mut impl Iterator<Item=char>) -> EdgePosition {
    cpos!(
        chars.next().expect(EXPECT_CHARS),
        chars.next().expect(EXPECT_CHARS)
    )
}

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

fn split_at_first_space(s: &str) -> (&str, &str) {
    let mut iter = s.splitn(2, ' ');
    let exp = "failed to split at first space";
    (iter.next().expect(exp), iter.next().expect(exp))
}

// NOTE: default_piece = [0,5]
// NOTE: select = [0] TODO: what is this
pub(crate) fn load_cross<P: AsRef<Path>>(path: P)
    -> HashMap<EdgePosition, Tern<Vec<EdgePosition>, Algorithm>>
{
    let file = File::open(path).expect("failed to open file");
    let mut reader = BufReader::new(file);

    let mut map = HashMap::new();

    for mut line in reader.lines()
        .map(|l| l.expect("failed to read line")) {

        if &line == "///" { break }
        if line.starts_with("//") { continue }

        let (piece, mut line) = split_at_first_space(&line);
        let piece = piece.chars()
            .map(|c| c.to_digit(10)
                .expect(&format!("{} is not a valid integer", c)) as u8)
            .collect::<EdgePosition>();

        let mut tern_vec =
            Vec::<(Vec<EdgePosition>, Algorithm)>::with_capacity(10);
        while !line.is_empty() {
            let mut predicates = vec![];
            while line.chars().nth(0).unwrap().is_numeric() {
                let (pred, rest) = line.split_at(2);
                line = rest;
                predicates.push(pred.chars().collect::<EdgePosition>())
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

        map.insert(piece, tern).expect_none("Duplicate key in map");
    }

    map
}