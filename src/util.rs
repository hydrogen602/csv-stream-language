use std::mem;

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

