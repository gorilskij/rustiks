use std::iter::Map;

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