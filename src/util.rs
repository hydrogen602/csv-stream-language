use core::fmt;
use std::{mem, error::Error, fmt::Debug};

#[derive(Debug, Clone)]
pub struct ParseError(String);

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ParseError: {}", self.0)
    }
}

impl Error for ParseError {}

/// TODO: how to get accumulator
pub struct MapFold<I, F, G, C>
{
    iter: I,
    map_f: F,
    fold_f: G,
    acc: C,
}

impl<B, I: Iterator<Item=impl Clone>, F, G, C> Iterator for MapFold<I, F, G, C> where 
    F: FnMut(I::Item) -> B, 
    G: FnMut(C, I::Item) -> C,
    C: Default
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

pub fn to_1_tuple<T: Debug>(v: Vec<T>) -> (T,) {
    if v.len() == 1 {
        let mut it = v.into_iter();
        (it.next().unwrap(),)
    }
    else {
        panic!("Wrong number of arguments, expected 1: {:?}", v);
    }
}

pub fn to_2_tuple<T: Debug>(v: Vec<T>) -> (T,T) {
    if v.len() == 2 {
        let mut it = v.into_iter();
        (it.next().unwrap(), it.next().unwrap())
    }
    else {
        panic!("Wrong number of arguments, expected 2: {:?}", v);
    }
}

#[allow(dead_code)]
pub fn to_3_tuple<T: Debug>(v: Vec<T>) -> (T,T,T) {
    if v.len() == 3 {
        let mut it = v.into_iter();
        (it.next().unwrap(), it.next().unwrap(), it.next().unwrap())
    }
    else {
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

