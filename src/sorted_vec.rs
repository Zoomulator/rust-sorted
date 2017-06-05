use std::ops;
use std::marker::PhantomData;

use super::{
    Sorted,
    SortOrder,
    SortedSlice,
};

#[derive(Debug)]
pub struct SortedVec<T,S> {
    inner: Vec<T>,
    order: PhantomData<*const S>
}

impl<T,S> Sorted for SortedVec<T,S> {
    type Ordering = S;
}

impl<T,S> SortedVec<T,S>
where
    S: SortOrder<T>
{
    pub fn by_sorting<I>(unsorted: I, _: S) -> Self
    where
        I: IntoIterator<Item=T>
    {
        let mut inner: Vec<T> = unsorted.into_iter().collect();
        S::sort(&mut inner);
        Self {inner, order: PhantomData}
    }
}

impl<T,S> SortedVec<T,S> {
    pub fn as_slice(&self) -> &[T] {
        &self.inner
    }

    pub fn as_vec(&self) -> &Vec<T> {
        &self.inner
    }
}

impl<T,S> ops::Deref for SortedVec<T,S> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        &self.inner
    }
}

// The std::vec::Vec implements a From<&[T]>, so this makes sense.
impl<'a,T,S> From<SortedSlice<'a,T,S>> for SortedVec<T,S>
where
    T: Clone
{
    fn from(s: SortedSlice<'a,T,S>) -> Self {
        SortedVec {
            inner: s.as_slice().to_vec(),
            order: PhantomData
        }
    }
}

impl<'a,T,S,U> From<U> for SortedVec<T,S>
where
    U: IntoIterator<Item=T> + Sorted<Ordering=S>
{
    fn from(x: U) -> Self {
        SortedVec {
            inner: x.into_iter().collect(),
            order: PhantomData,
        }
    }
}

impl<T,S> Into<Vec<T>> for SortedVec<T,S>
{
    fn into(self) -> Vec<T> {
        self.inner
    }
}