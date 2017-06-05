use std::ops;
use std::marker::PhantomData;

use super::{
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
        O::sort(&mut inner);
        Self {inner, ordering: PhantomData}
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

impl<'a,T,O,U> From<U> for SortedVec<T,O>
where
    U: IntoIterator<Item=T> + Sorted<Ordering=O>
{
    fn from(x: U) -> Self {
        SortedVec {
            inner: x.into_iter().collect(),
            ordering: PhantomData,
        }
    }
}

impl<T,O> Into<Vec<T>> for SortedVec<T,O>
{
    fn into(self) -> Vec<T> {
        self.inner
    }
}