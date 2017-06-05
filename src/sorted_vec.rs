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
}

impl<T,S> ops::Deref for SortedVec<T,S> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        &self.inner
    }
}

/*
impl<'a,T,S> From<&'a SortedVec<T,S>> for SortedSlice<'a,T,S>
{
    fn from(v: &'a SortedVec<T,S>) -> Self {
        SortedSlice {
            inner: v.as_slice(),
            order: PhantomData
        }
    }
}

impl<T,S> Into<Vec<T>> for SortedVec<T,S>
{
    fn into(self) -> Vec<T> {
        self.inner
    }
}
*/