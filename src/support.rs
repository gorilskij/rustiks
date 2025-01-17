use std::iter::Map;
use serde::Deserialize;

// TODO: implement this better
#[macro_export]
macro_rules! array_collect {
    ($iter:expr, [$type:ty; $len:expr]) => {{
        let mut iter = $iter;

        let mut array: [$type; $len] = unsafe {
            std::mem::transmute([std::mem::MaybeUninit::<$type>::uninit(); $len])
        };

        for i in 0..$len { array[i] = iter.next().unwrap(); }
        array
    }}
}

#[macro_export]
macro_rules! tuple_map {
    ($f:expr; $( $elem:expr ),*) => {
        ( $( $f($elem) ),* )
    };
}

#[allow(dead_code)]
pub trait IterDeref<'a, T: 'a + Copy> where Self: Iterator<Item=&'a T> + Sized {
    fn d(self) -> Map<Self, fn(&T) -> T>;
}

#[allow(dead_code)]
impl<'a, T: 'a + Copy, I> IterDeref<'a, T> for I where I: Iterator<Item=&'a T> {
    fn d(self) -> Map<Self, fn(&T) -> T> {
        self.map(|x: &T| *x)
    }
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub enum Tern<C: PartialEq, R> {
    End(R),
    Con(C, R, Box<Tern<C, R>>),
}

#[allow(dead_code)]
#[macro_export]
macro_rules! tern {
    ($fin:expr) => {
        Tern::End($fin)
    };
    ($con:expr, $res:expr, $($rest:tt),*) => {
        Tern::Con($con, $res, Box::new(tern!($($rest),*)))
    };
}

#[allow(dead_code)]
impl<C: PartialEq, R> Tern<C, R> {
    fn create(mut conditions: Vec<(C, R)>, last: Self) -> Self {
        if conditions.is_empty() { return last }
        let c = conditions.pop().unwrap();
        Self::create(conditions, Self::Con(c.0, c.1, Box::new(last)))
    }

    pub fn new(conditions: Vec<(C, R)>, last: R) -> Self {
        Self::create(conditions, Self::End(last))
    }

    pub fn eval(&self, input: &C) -> &R {
        match self {
            Tern::End(r) => r,
            Tern::Con(c, r, b) => if c == input {
                r
            } else {
                b.eval(input)
            }
        }
    }
}

#[allow(dead_code)]
impl<C: PartialEq + Copy, R: Copy> Tern<C, R> {
    pub fn eval_owned(self, input: C) -> R {
        *self.eval(&input)
    }
}

pub trait IndexOf {
    fn index_of(&self, c: char, skip: usize) -> Option<usize>;
}

impl IndexOf for String {
    fn index_of(&self, c: char, skip: usize) -> Option<usize> {
        Some(self[skip..].find(c)? + skip)
    }
}

impl IndexOf for str {
    fn index_of(&self, c: char, skip: usize) -> Option<usize> {
        self.to_string().index_of(c, skip)
    }
}