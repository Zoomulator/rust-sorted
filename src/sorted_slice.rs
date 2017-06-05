use std::ops;
use std::marker::PhantomData;

use super::{
    Sorted,
    SortOrder,
};


#[derive(Debug)]
pub struct SortedSlice<'a,T,S>
where
    T:'a
{
    inner: &'a[T],
    order: PhantomData<*const S>
}

impl<'a,T,S> Sorted for SortedSlice<'a,T,S> {
    type Ordering = S;
}

impl<'a,T,S> SortedSlice<'a,T,S>
where
    T: Ord,
    S: SortOrder<T>
{
    pub fn by_sorting(slice: &'a mut [T], _: S) -> Self {
        S::sort(slice);
        Self {inner: slice, order: PhantomData} 
    }
}

impl<'a,T,S> SortedSlice<'a,T,S> {
    pub fn as_slice(&self) -> &[T] {
        &self.inner
    }
}

impl<'a,T,S> PartialEq<&'a [T]> for SortedSlice<'a,T,S>
where T: Ord
{
    fn eq(&self, other: &&[T]) -> bool {
        self.as_slice() == *other
    }

    fn ne(&self, other: &&[T]) -> bool {
        self.as_slice() != *other
    }
}

impl<'a,T,S> ops::Index<usize> for SortedSlice<'a,T,S>
{
    type Output = T;
    fn index(&self, i: usize) -> &Self::Output {
        &self.inner[i]
    }
}

/*
impl<'a,T,S> ops::Deref for SortedSlice<'a,T,S> 
{
    type Target = [T];
    fn deref(&self) -> &[T] {
        self.inner
    }
}
*/

impl<'a,T,S,U> From<&'a U> for SortedSlice<'a,T,S>
where
    U: 'a + ops::Deref<Target=[T]> + Sorted<Ordering=S>,
{
    fn from(x: &'a U) -> Self {
        SortedSlice {
            inner: x.deref(),
            order: PhantomData
        }
    }
}