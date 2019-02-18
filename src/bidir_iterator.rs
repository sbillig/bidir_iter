
pub trait BidirIterator {
    type Item;

    /// # Examples
    /// ```
    /// use bidir_iter::*;
    ///
    /// let a: &[i64] = &[1, 2, 3];
    /// let mut iter = a.bidir_iter();
    ///
    /// assert_eq!(iter.next(), Some(&1));
    /// assert_eq!(iter.next(), Some(&2));
    /// assert_eq!(iter.next(), Some(&3));
    /// assert_eq!(iter.next(), None);
    /// assert_eq!(iter.prev(), Some(&3));
    /// assert_eq!(iter.prev(), Some(&2));
    /// assert_eq!(iter.prev(), Some(&1));
    /// assert_eq!(iter.prev(), None);
    /// ```
    fn next(&mut self) -> Option<Self::Item>;
    fn prev(&mut self) -> Option<Self::Item>;

    /// Returns a forward-moving Iterator.
    ///
    /// # Examples
    /// ```
    /// use bidir_iter::*;
    ///
    /// let a: &[i64] = &[1, 2, 3, 4];
    /// let mut iter = a.bidir_iter();
    /// let mut sum = 0;
    /// for i in iter.forward() {
    ///     sum += i;
    /// }
    /// assert_eq!(sum, 10);
    /// ```
    fn forward(self) -> Forward<Self> where Self: Sized {
        Forward { iter: self }
    }

    /// Returns a backward-moving Iterator; ie. an Iterator that calls prev().
    fn backward(self) -> Backward<Self> where Self: Sized {
        Backward { iter: self }
    }

    /// # Examples
    /// ```
    /// use bidir_iter::*;
    ///
    /// let a: &[i64] = &[1, 2, 3, 4, 5, 6, 7];
    /// let mut iter = a.bidir_iter().filter(|n| *n % 2 == 0);
    ///
    /// assert_eq!(iter.next(), Some(&2));
    /// assert_eq!(iter.next(), Some(&4));
    /// assert_eq!(iter.next(), Some(&6));
    /// assert_eq!(iter.next(), None);
    /// assert_eq!(iter.prev(), Some(&6));
    /// assert_eq!(iter.prev(), Some(&4));
    /// assert_eq!(iter.prev(), Some(&2));
    /// assert_eq!(iter.prev(), None);
    /// ```
    fn filter<P>(self, predicate: P) -> Filter<Self, P> where
        Self: Sized, P: FnMut(&Self::Item) -> bool,
    {
        Filter { iter: self, predicate }
    }

    /// # Examples
    /// ```
    /// use bidir_iter::*;
    ///
    /// let a: &[i64] = &[1, 2, 3];
    /// let mut iter = a.bidir_iter().map(|n| *n * 10);
    ///
    /// assert_eq!(iter.next(), Some(10));
    /// assert_eq!(iter.next(), Some(20));
    /// assert_eq!(iter.next(), Some(30));
    /// assert_eq!(iter.next(), None);
    /// assert_eq!(iter.prev(), Some(30));
    /// ```
    fn map<B, F>(self, f: F) -> Map<Self, F> where
        Self: Sized, F: FnMut(Self::Item) -> B,
    {
        Map { iter: self, f }
    }
}

pub struct Forward<B> {
    iter: B
}

impl<B: BidirIterator> Iterator for Forward<B> {
    type Item = B::Item;

    fn next(&mut self) -> Option<B::Item> {
        self.iter.next()
    }
}

pub struct Backward<B> {
    iter: B
}

impl<B: BidirIterator> Iterator for Backward<B> {
    type Item = B::Item;

    fn next(&mut self) -> Option<B::Item> {
        self.iter.prev()
    }
}

pub struct Filter<B, P> {
    iter: B,
    predicate: P
}

impl<B: BidirIterator, P> BidirIterator for Filter<B, P>
where P: FnMut(&B::Item) -> bool
{
    type Item = B::Item;

    fn next(&mut self) -> Option<B::Item> {
        loop {
            match self.iter.next() {
                Some(i) => if (self.predicate)(&i) { return Some(i); },
                None => break
            }
        }
        None
    }

    fn prev(&mut self) -> Option<B::Item> {
        loop {
            match self.iter.prev() {
                Some(i) => if (self.predicate)(&i) { return Some(i); },
                None => break
            }
        }
        None
    }
}

pub struct Map<I, F> {
    iter: I,
    f: F
}

impl<B, I: BidirIterator, F> BidirIterator for Map<I, F> where
    F: FnMut(I::Item) -> B {

    type Item = B;

    fn next(&mut self) -> Option<B> {
        self.iter.next().map(&mut self.f)
    }

    fn prev(&mut self) -> Option<B> {
        self.iter.prev().map(&mut self.f)
    }
}
