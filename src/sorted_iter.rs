use super::SortedIterator;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct SortedIter<I, O> {
    pub(crate) inner: I,
    pub(crate) ordering: PhantomData<O>,
}

impl<I, O> Iterator for SortedIter<I, O>
where
    I: Iterator,
{
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<I, O> SortedIterator for SortedIter<I, O>
where
    I: Iterator,
{
    type Ordering = O;
}
