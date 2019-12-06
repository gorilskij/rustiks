use crate::cube::Cube;
use std::fmt::{Debug, Formatter, Error, Display};
use colored::{Colorize, ColoredString};
use itertools::Itertools;

fn to_color(f: u8) -> ColoredString {
    let s = "  ";
    match f {
        0 => s.on_white(),
        1 => s.on_red(),
        2 => s.on_blue(),
        4 => s.on_magenta(),
        5 => s.on_green(),
        3 => s.on_yellow(),
        _ => unreachable!()
    }
}


// TODO: maybe find a better way to do this
#[derive(Copy, Clone)]
pub struct ColoredCube<'a>(&'a Cube);

fn color_cube(uncolored: String) -> String {
    // TODO: fix ugly replaces
    uncolored
        .replace("\n\n", "nn")
        .replace("\n", "\n\n")
        .replace("nn", "\n\n\n")
        .replace(" ", "   ")
        .replace("     ", "    ")
        .chars()
        .map(|c| {
            if c.is_numeric() {
                to_color(c.to_string()
                    .parse()
                    .expect(&format!("couldn't parse '{}' as u8", c))
                )
            } else {
                c.to_string().normal()
            }
        })
        .join("")
}

impl<'a> Display for ColoredCube<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", color_cube(format!("{:?}", self.0)))
    }
}

// just for convenience, TODO: remove
impl<'a> Debug for ColoredCube<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        Display::fmt(self, f)
    }
}

#[allow(dead_code)]
impl Cube {
    pub fn colored(&self) -> ColoredCube {
        ColoredCube(self)
    }
}