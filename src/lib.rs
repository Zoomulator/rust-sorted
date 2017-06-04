use std::ops;
use std::cmp::Ordering;


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
    order: S
}

impl<'a,T,S> SortedSlice<'a,T,S>
where
    T: Ord,
    S: SortOrder<T>
{
    pub fn by_sorting(slice: &'a mut [T], order: S) -> Self {
        S::sort(slice);
        Self {inner: slice, order} 
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
    order: S,
}

impl<T,S> SortedVec<T,S>
where
    S: SortOrder<T>
{
    pub fn by_sorting<I>(unsorted: I, order: S) -> Self
    where
        I: IntoIterator<Item=T>
    {
        let mut inner: Vec<T> = unsorted.into_iter().collect();
        S::sort(&mut inner);
        Self {inner, order}
    }

    pub fn slice(&self) -> SortedSlice<T,S> {
        SortedSlice{
            inner: &self.inner[..],
            order: self.order
        }
    }
}

// Can't deref to SortedSlice from SortedVec. SortOrder must be made completely
// static.

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
    fn (T: Ord + Clone)(entry: (T,T,T)) -> T { entry.0.clone() }
}


#[test]
fn test_sort_by_first() {
    let s: [(i32,i32);3] = [(5,3),(2,7),(3,4)];
    let v = SortedVec::by_sorting(s.iter().cloned(), SortByFirst);
    println!("{:?}", v);
}