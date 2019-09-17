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

#[allow(dead_code)]
pub struct Lazy<T>(Option<T>);

#[allow(dead_code)]
impl<T> Lazy<T> {
    pub const fn new() -> Self {
        Self(None)
    }

    pub fn set(&mut self, value: T) {
        match self.0 {
            Some(_) => panic!("Lazy already has a value"),
            None => self.0 = Some(value)
        }
    }

    pub fn get(&self) -> &T {
        match self.0 {
            None => panic!("Lazy doesn't have a value"),
            Some(ref value) => value
        }
    }
}