use std::vec;
use std::ops;
use std::marker::PhantomData;

use super::{
    Sortable,
    SortedIter,
    Sorted,
    SortOrder,
    SortedSlice,
};

#[derive(Debug)]
pub struct SortedVec<T,O> {
    inner: Vec<T>,
    ordering: PhantomData<*const O>
}

impl<T,O> Sorted for SortedVec<T,O> {
    type Ordering = O;
}

impl<T,O> SortedVec<T,O>
where
    O: SortOrder<T>
{
    pub fn by_sorting<I>(unsorted: I, _: O) -> Self
    where
        I: IntoIterator<Item=T>
    {
        let mut inner: Vec<T> = unsorted.into_iter().collect();
        Sortable::sort(&mut inner, O::cmp);
        Self {inner, ordering: PhantomData}
    }

    pub fn insert(&mut self, x: T) {
        let i = match Sortable::search(&self.inner, &x, O::cmp) {
            Ok(i) => i, Err(i) => i
        };
        self.inner.insert(i, x);
    }
}

impl<T,O> SortedVec<T,O> {
    pub fn as_slice(&self) -> &[T] {
        &self.inner
    }

    pub fn as_vec(&self) -> &Vec<T> {
        &self.inner
    }
}

impl<T,O> ops::Deref for SortedVec<T,O> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        &self.inner
    }
}

// The std::vec::Vec implements a From<&[T]>, so this makes sense.
impl<'a,T,O> From<SortedSlice<'a,T,O>> for SortedVec<T,O>
where
    T: Clone
{
    fn from(s: SortedSlice<'a,T,O>) -> Self {
        SortedVec {
            inner: s.as_slice().to_vec(),
            ordering: PhantomData
        }
    }
}

// Can't match FromIterator when adding Sorted bound.
// Define a new FromSortedIterator trait?
/*
impl<'a,T,O> std::iter::FromIterator<T> for SortedVec<T,O>
{
    fn from_iter<I>(x: I) -> Self
    where I: IntoIterator<Item=T> + Sorted<Ordering=O> {
        SortedVec {
            inner: x.into_iter().collect(),
            ordering: PhantomData,
        }
    }
}
*/

impl<T,O> Into<Vec<T>> for SortedVec<T,O>
{
    fn into(self) -> Vec<T> {
        self.inner
    }
}

impl<O,T> IntoIterator for SortedVec<T,O>
{
    type Item = T;
    type IntoIter = SortedIter<vec::IntoIter<T>,O>;
    fn into_iter(self) -> Self::IntoIter {
        SortedIter {
            inner: self.inner.into_iter(),
            ordering: PhantomData
        }
    }
}