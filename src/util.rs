use core::fmt;
use std::{mem, error::Error};

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

