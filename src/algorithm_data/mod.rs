//Moves = 'UDFBRL'.split('').flat_map {|i| [i, i + '2', i + "'"]}
extern crate serde;
extern crate serde_json;

use std::collections::HashMap;
use crate::cube::piece::face::Face;
use crate::support::Tern;
use crate::cube::piece::position::EdgePosition;
use crate::cube::algorithm::Algorithm;
use serde::Deserialize;

// NOTE: All algorithms for down_front = [0, 5]

#[derive(Deserialize)]
struct CrossAlg {
    piece: EdgePosition,
    select: Face, // TODO: understand what this is and rename it
    algorithms: HashMap<EdgePosition, Tern<Vec<EdgePosition>, Algorithm>>,
}