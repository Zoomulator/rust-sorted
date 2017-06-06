use std::ops;
use std::marker::PhantomData;

use super::{
    Sortable,
    IsSorted,
    SortOrder,
};


#[derive(Debug)]
pub struct SortedSlice<'a,T,O>
where
    T:'a
{
    inner: &'a[T],
    ordering: PhantomData<*const O>
}

impl<'a,T,O> IsSorted for SortedSlice<'a,T,O> {
    type Ordering = O;
}

impl<'a,T,O> SortedSlice<'a,T,O>
where
    T: Ord,
    O: SortOrder<T>
{
    pub fn by_sorting(mut inner: &'a mut [T], _: O) -> Self {
        Sortable::sort(&mut inner, O::cmp);
        Self {inner, ordering: PhantomData} 
    }
}

impl<'a,T,O> SortedSlice<'a,T,O> {
    pub fn as_slice(&self) -> &[T] {
        &self.inner
    }
}

impl<'a,T,O> PartialEq<&'a [T]> for SortedSlice<'a,T,O>
where T: Ord
{
    fn eq(&self, other: &&[T]) -> bool {
        self.as_slice() == *other
    }

    fn ne(&self, other: &&[T]) -> bool {
        self.as_slice() != *other
    }
}

impl<'a,T,S> ops::Deref for SortedSlice<'a,T,S> 
{
    type Target = [T];
    fn deref(&self) -> &[T] {
        self.inner
    }
}

impl<'a,T,O,U> From<&'a U> for SortedSlice<'a,T,O>
where
    U: 'a + ops::Deref<Target=[T]> + IsSorted<Ordering=O>,
{
    fn from(x: &'a U) -> Self {
        SortedSlice {
            inner: x.deref(),
            ordering: PhantomData
        }
    }
}