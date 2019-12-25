use crate::bidir_iterator::BidirIterator;

pub trait BidirIterate {
    type Item;
    type BidirIter: BidirIterator<Item = Self::Item>;

    fn bidir_iter(&self) -> Self::BidirIter;
}

pub struct BiIter<'a, T> {
    next: usize,
    slice: &'a [T],
}

impl<'a, T> BidirIterator for BiIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        let x = self.slice.get(self.next);
        self.next += 1;
        x
    }

    fn prev(&mut self) -> Option<&'a T> {
        if self.next <= 1 {
            self.next = 0;
            None
        } else {
            self.next -= 1;
            self.slice.get(self.next - 1)
        }
    }
}

impl<'a, T> BidirIterate for &'a [T] {
    type Item = &'a T;
    type BidirIter = BiIter<'a, T>;

    fn bidir_iter(&self) -> Self::BidirIter {
        BiIter {
            next: 0,
            slice: self,
        }
    }
}
