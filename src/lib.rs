use std::ops;
use std::cmp::Ordering;
use std::marker::PhantomData;
use std::mem;


// I wonder if the order property of the sorted slices and vecs are
// optimized away.

pub trait Sorted {}

#[derive(Debug)]
pub struct SortedSlice<'a,T,S>
where
    T:'a,
    S: SortOrder<T>
{
    inner: &'a[T],
    order: PhantomData<*const S>
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

impl<'a,T,S> ops::Index<usize> for SortedSlice<'a,T,S>
where
    S: SortOrder<T>
{
    type Output = T;
    fn index(&self, i: usize) -> &Self::Output {
        &self.inner[i]
    }
}

impl<'a,T,S> ops::Deref for SortedSlice<'a,T,S> 
where
    S: SortOrder<T>
{
    type Target = [T];
    fn deref(&self) -> &[T] {
        self.inner
    }
}

impl<'a,T,S> Sorted for SortedSlice<'a,T,S>
where S: SortOrder<T>
{}

#[derive(Debug)]
pub struct SortedVec<T,S: SortOrder<T>> {
    inner: Vec<T>,
    order: PhantomData<*const S>
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

    pub fn as_sorted_slice(&self) -> SortedSlice<T,S> {
        SortedSlice{
            inner: &self.inner,
            order: self.order
        }
    }

    pub fn as_slice(&self) -> &[T] {
        &self.inner
    }
}

impl<'a,T,S> From<&'a SortedVec<T,S>> for SortedSlice<'a,T,S>
where
    S: SortOrder<T>
{
    fn from(v: &'a SortedVec<T,S>) -> Self {
        SortedSlice {
            inner: v.as_slice(),
            order: PhantomData
        }
    }
}

impl<T,S> Into<Vec<T>> for SortedVec<T,S>
where
    S: SortOrder<T>
{
    fn into(self) -> Vec<T> {
        self.inner
    }
}

pub trait SortOrder<T> : Clone + Copy {
    fn sort(&mut [T]);
}

#[derive(Debug, Clone, Copy)]
pub struct DefaultOrder;

impl<T> SortOrder<T> for DefaultOrder
where T: Ord {
    fn sort(s: &mut [T]) {
        s.sort();
    }
}


macro_rules! order_by_key {
    ($name:ident: $(fn ($($gen:tt)*) ($entry:ident: $t:ty) -> $r:ty $blk:block)*) => (
        #[derive(Debug, Clone, Copy)]
        struct $name;

        $(impl<$($gen)*> SortOrder<$t> for $name
        {
            fn sort(s: &mut [$t]) {
                fn key<$($gen)*>($entry: &$t) -> $r $blk
                s.sort_by_key(key)
            }
        })*
    );
}

order_by_key!{ SortByFirst:
    fn (T: Ord + Clone)(entry: (T,T)) -> T { entry.0.clone() }
}

order_by_key!{ SortBySecond:
    fn (T: Ord + Clone)(entry: (T,T)) -> T { entry.1.clone() }
}


#[test]
fn test_sort_by_first() {
    let s = vec![(5,3),(2,7),(3,4)];
    let v = SortedVec::by_sorting(s, SortBySecond);
    assert_eq!(
        &[(5,3),(3,4),(2,7)],
        v.as_slice()
    );
}