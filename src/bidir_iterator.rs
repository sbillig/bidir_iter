pub trait BidirIterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

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
    /// assert_eq!(iter.next(), Some(&1));
    /// ```
    fn prev(&mut self) -> Option<Self::Item>;

    /// Create a forward-moving Iterator,
    /// starting at the current position.
    /// The forward iterator borrows the underlying
    /// bidirectional iterator.
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
    ///
    /// for i in iter.backward() {
    ///     sum += i;
    /// }
    /// assert_eq!(sum, 20);
    /// ```
    fn forward(&mut self) -> Forward<&mut Self>
    where
        Self: Sized,
    {
        Forward { iter: self }
    }

    /// Create a forward-moving Iterator,
    /// starting at the current position.
    /// Like `forward()`, but the resulting iterator
    /// owns the underlying bidirectional iterator.
    ///
    /// # Examples
    /// ```
    /// use bidir_iter::*;
    ///
    /// let a: &[i64] = &[1, 2, 3, 4];
    ///
    /// let mut iter = a.bidir_iter().forward_owned();
    /// let mut sum = 0;
    /// for i in iter {
    ///     sum += i;
    /// }
    /// assert_eq!(sum, 10);
    /// ```
    fn forward_owned(self) -> Forward<Self>
    where
        Self: Sized,
    {
        Forward { iter: self }
    }

    /// Create a backward-moving Iterator,
    /// starting at the current position.
    fn backward(&mut self) -> Backward<&mut Self>
    where
        Self: Sized,
    {
        Backward { iter: self }
    }

    /// Create a backward-moving Iterator,
    /// starting at the current position.
    /// The resulting iterator owns the underlying
    /// bidir iterator.
    ///
    /// # Examples
    /// ```
    /// use bidir_iter::*;
    ///
    /// let a: &[i64] = &[1, 2, 3, 4];
    /// let mut iter = a.bidir_iter();
    /// // .forward() and .backward() borrow the bidir iter
    /// assert_eq!(iter.forward().count(), 4);
    /// let mut b = iter.backward_owned();
    /// // iter has been moved into b
    /// assert_eq!(b.count(), 4);
    /// ```
    fn backward_owned(self) -> Backward<Self>
    where
        Self: Sized,
    {
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
    fn filter<P>(self, predicate: P) -> Filter<Self, P>
    where
        Self: Sized,
        P: FnMut(&Self::Item) -> bool,
    {
        Filter {
            iter: self,
            predicate,
        }
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
    fn map<B, F>(self, f: F) -> Map<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> B,
    {
        Map { iter: self, f }
    }

    /// # Examples
    /// ```
    /// use bidir_iter::*;
    ///
    /// let a: &[i64] = &[0, 1, 2, 0, 3];
    /// let mut iter = a.bidir_iter().filter_map(|i| if *i == 0 { None } else { Some(1 / i) });
    ///
    /// assert_eq!(iter.next(), Some(1/1));
    /// assert_eq!(iter.next(), Some(1/2));
    /// assert_eq!(iter.next(), Some(1/3));
    /// assert_eq!(iter.next(), None);
    /// ```
    fn filter_map<B, F>(self, f: F) -> FilterMap<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> Option<B>,
    {
        FilterMap { iter: self, f }
    }
}

impl<T> BidirIterator for &mut T
where
    T: BidirIterator,
{
    type Item = T::Item;
    fn next(&mut self) -> Option<Self::Item> {
        (*self).next()
    }
    fn prev(&mut self) -> Option<Self::Item> {
        (*self).prev()
    }
}

pub struct Forward<T> {
    iter: T,
}

impl<B> Iterator for Forward<B>
where
    B: BidirIterator,
{
    type Item = B::Item;

    fn next(&mut self) -> Option<B::Item> {
        self.iter.next()
    }
}

pub struct Backward<B> {
    iter: B,
}

impl<B> Iterator for Backward<B>
where
    B: BidirIterator,
{
    type Item = B::Item;

    fn next(&mut self) -> Option<B::Item> {
        self.iter.prev()
    }
}

pub struct Filter<B, P> {
    iter: B,
    predicate: P,
}

impl<B: BidirIterator, P> BidirIterator for Filter<B, P>
where
    P: FnMut(&B::Item) -> bool,
{
    type Item = B::Item;

    fn next(&mut self) -> Option<B::Item> {
        while let Some(i) = self.iter.next() {
            if (self.predicate)(&i) {
                return Some(i);
            }
        }
        None
    }

    fn prev(&mut self) -> Option<B::Item> {
        while let Some(i) = self.iter.prev() {
            if (self.predicate)(&i) {
                return Some(i);
            }
        }
        None
    }
}

pub struct Map<I, F> {
    iter: I,
    f: F,
}

impl<B, I: BidirIterator, F> BidirIterator for Map<I, F>
where
    F: FnMut(I::Item) -> B,
{
    type Item = B;

    fn next(&mut self) -> Option<B> {
        self.iter.next().map(&mut self.f)
    }

    fn prev(&mut self) -> Option<B> {
        self.iter.prev().map(&mut self.f)
    }
}

pub struct FilterMap<I, F> {
    iter: I,
    f: F,
}

impl<B, I: BidirIterator, F> BidirIterator for FilterMap<I, F>
where
    F: FnMut(I::Item) -> Option<B>,
{
    type Item = B;

    fn next(&mut self) -> Option<B> {
        while let Some(a) = self.iter.next() {
            if let Some(b) = (self.f)(a) {
                return Some(b);
            }
        }
        None
    }

    fn prev(&mut self) -> Option<B> {
        while let Some(a) = self.iter.prev() {
            if let Some(b) = (self.f)(a) {
                return Some(b);
            }
        }
        None
    }
}
