use core::fmt;
use std::{error::Error, fmt::Debug, mem};

use either::Either;

#[derive(Debug, Clone)]
pub struct ParseError(String);

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ParseError: {}", self.0)
    }
}

impl Error for ParseError {}

/// TODO: how to get accumulator
pub struct MapFold<I, F, G, C> {
    iter: I,
    map_f: F,
    fold_f: G,
    acc: C,
}

impl<B, I: Iterator<Item = impl Clone>, F, G, C> Iterator for MapFold<I, F, G, C>
where
    F: FnMut(I::Item) -> B,
    G: FnMut(C, I::Item) -> C,
    C: Default,
{
    type Item = B;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            None => None,
            Some(e) => {
                let b = (self.map_f)(e.clone());

                let acc = mem::take(&mut self.acc);
                self.acc = (self.fold_f)(acc, e);

                Some(b)
            }
        }
    }
}

pub fn get_args<const C: usize, T: Debug>(v: Vec<T>) -> [T; C] {
    if v.len() != C {
        panic!("Wrong number of arguments, expected {}: {:?}", C, v);
    }

    let mut it = v.into_iter();

    [0u8; C].map(|_| it.next().unwrap())
}

pub fn get_args_2_sizes<const C1: usize, const C2: usize, T: Debug>(
    v: Vec<T>,
) -> Either<[T; C1], [T; C2]> {
    if v.len() == C1 {
        let mut it = v.into_iter();
        Either::Left([0u8; C1].map(|_| it.next().unwrap()))
    } else if v.len() == C2 {
        let mut it = v.into_iter();
        Either::Right([0u8; C2].map(|_| it.next().unwrap()))
    } else {
        panic!(
            "Wrong number of arguments, expected {} or {}: {:?}",
            C1, C2, v
        )
    }
}

pub fn to_1_tuple<T: Debug>(v: Vec<T>) -> (T,) {
    if v.len() == 1 {
        let mut it = v.into_iter();
        (it.next().unwrap(),)
    } else {
        panic!("Wrong number of arguments, expected 1: {:?}", v);
    }
}

pub fn to_2_tuple<T: Debug>(v: Vec<T>) -> (T, T) {
    if v.len() == 2 {
        let mut it = v.into_iter();
        (it.next().unwrap(), it.next().unwrap())
    } else {
        panic!("Wrong number of arguments, expected 2: {:?}", v);
    }
}

#[allow(dead_code)]
pub fn to_3_tuple<T: Debug>(v: Vec<T>) -> (T, T, T) {
    if v.len() == 3 {
        let mut it = v.into_iter();
        (it.next().unwrap(), it.next().unwrap(), it.next().unwrap())
    } else {
        panic!("Wrong number of arguments, expected 3: {:?}", v);
    }
}

// pub trait ConsumeToTuple<T> {
//     fn to_tuple(self: Self) -> T;
// }

// impl<T: Debug> ConsumeToTuple<(T,)> for Vec<T> {
//     fn to_tuple(self: Self) -> (T,) {
//         if self.len() == 1 {
//             let mut it = self.into_iter();
//             (it.next().unwrap(),)
//         }
//         else {
//             panic!("Wrong arguments: {:?}", self);
//         }
//     }
// }

// impl<T: Debug> ConsumeToTuple<(T, T)> for Vec<T> {
//     fn to_tuple(self: Self) -> (T, T) {
//         if self.len() == 2 {
//             let mut it = self.into_iter();
//             (it.next().unwrap(), it.next().unwrap())
//         }
//         else {
//             panic!("Wrong arguments: {:?}", self);
//         }
//     }
// }

// impl<T: Debug> ConsumeToTuple<(T, T, T)> for Vec<T> {
//     fn to_tuple(self: Self) -> (T, T, T) {
//         if self.len() == 3 {
//             let mut it = self.into_iter();
//             (it.next().unwrap(), it.next().unwrap(), it.next().unwrap())
//         }
//         else {
//             panic!("Wrong arguments: {:?}", self);
//         }
//     }
// }

pub struct OwnedStringRead {
    pub data: String,
    idx: usize,
}

impl OwnedStringRead {
    fn bytes_left(&self) -> usize {
        usize::max(self.data.len() - self.idx, 0)
    }

    pub fn new(data: String) -> Self {
        Self { data, idx: 0 }
    }
}

impl std::io::Read for OwnedStringRead {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        // TODO: improve this hack fix
        let bytes_to_read = usize::min(buf.len(), self.bytes_left());

        for i in 0..bytes_to_read {
            buf[i] = self.data.as_bytes()[i + self.idx];
        }

        self.idx += bytes_to_read;

        Ok(bytes_to_read)
    }
}
