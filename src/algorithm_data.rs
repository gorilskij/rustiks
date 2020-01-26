//Moves = 'UDFBRL'.split('').flat_map {|i| [i, i + '2', i + "'"]}
use std::collections::HashMap;
use crate::support::Tern;
use crate::cube::algorithm::Alg;
use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};
use crate::support::IndexOf;
use std::hash::Hash;
use std::fmt::Debug;
use std::mem;
use crate::cube::position::Pos;

fn iter_lines<P: AsRef<Path>>(path: P) -> impl Iterator<Item=String> {
    let file = File::open(path).expect("failed to open file");
    let reader = BufReader::new(file);
    reader.lines().map(|l| l.expect("failed to read line"))
}

fn split_at_first(s: &str, c: char) -> (&str, &str) {
    let mut iter = s.splitn(2, c);
    let exp = &format!("failed to split at first '{}'", c);
    (iter.next().expect(exp), iter.next().expect(exp))
}

fn safe_insert<K: Eq + Hash, V: Debug>(map: &mut HashMap<K, V>, k: K, v: V) {
    if let Some(prev) = map.insert(k, v) {
        panic!("Duplicate key in map, previous value: {:?}", prev)
    }
}

trait PieceKey {
    const LENGTH: usize;

    fn from_char_iter(iter: impl Iterator<Item=char>) -> Self;
}

fn load1<P: AsRef<Path>, K>(path: P)
                                       -> HashMap<K, Tern<Vec<K>, Alg>> where
    K: PieceKey + Eq + Hash + Debug
{
    let mut map = HashMap::new();

    for line in iter_lines(path) {
        if &line == "///" { break }
        if line.starts_with("//") { continue }

        let (pieces, mut line) = split_at_first(&line, ':');
        let pieces = K::from_char_iter(pieces.chars());

        let mut tern_vec =
            Vec::<(Vec<K>, Alg)>::with_capacity(10);
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
            let alg = Alg::from(&alg[1..alg.len() - 1]);

            tern_vec.push((predicates, alg));
        }

        let mut iter = tern_vec.into_iter().rev();
        let mut tern = Tern::End(iter.next().expect("Unexpected empty condition vector").1);
        for (con, alg) in iter {
            tern = Tern::Con(con, alg, Box::new(tern));
        }

        safe_insert(&mut map, pieces, tern)
    }

    map
}

impl PieceKey for Pos<2> {
    const LENGTH: usize = 2;

    fn from_char_iter(iter: impl Iterator<Item=char>) -> Self {
        iter.collect()
    }
}

// a parsing item (really a tuple or array, todo generalise)
type CEPos = (Pos<3>, Pos<2>);

impl PieceKey for CEPos {
    const LENGTH: usize = 5;

    fn from_char_iter(mut iter: impl Iterator<Item=char>) -> Self {
        let c = (&mut iter).take(3).collect();
        let e = iter.collect();
        (c, e)
    }
}

fn load2<P: AsRef<Path>>(path: P) -> HashMap<Vec<usize>, Alg> {
    let mut map = HashMap::new();

    for line in iter_lines(path) {
        let (pred, alg) = split_at_first(&line, ':');
        let pred = pred.split(',')
            .map(|i| i.parse().unwrap_or_else(|_| panic!("Invalid value for usize: {}" , i)))
            .collect();
        let alg = Alg::from(&alg[1..alg.len() - 1]);
        safe_insert(&mut map, pred, alg);
    }

    map
}


// Data:
fn lazy_get<'a, T>(data: &'a mut Option<T>, load: fn(&'a str) -> T, path: &'a str) -> &'a T {
    if data.is_none() {
        mem::replace(data, Some(load(path)));
    }
    data.as_ref().unwrap()
}

macro_rules! lazy_load {
    ($const:ident, $fn:ident, $load:expr, $path:expr, $type:ty) => {
        static mut $const: Option<$type> = None;
        pub(crate) fn $fn() -> &'static $type {
            unsafe { lazy_get(&mut $const, $load, $path) }
        }
    };
}

type MapToTern<T> = HashMap<T, Tern<Vec<T>, Alg>>;

lazy_load!(CROSS_DATA, cross_data, load1, "src/algorithm_data/data/cross.txt",
    MapToTern<Pos<2>>);

lazy_load!(F2L_DATA, f2l_data, load1, "src/algorithm_data/data/f2l.txt",
    MapToTern<CEPos>);

lazy_load!(OLL_DATA, oll_data, load2, "src/algorithm_data/data/oll.txt",
    HashMap<Vec<usize>, Alg>);

lazy_load!(PLL_DATA, pll_data, load2, "src/algorithm_data/data/pll.txt",
    HashMap<Vec<usize>, Alg>);