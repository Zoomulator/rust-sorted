use std::cmp::Ordering;
use super::{Collection, SearchableByOrder, RetainsOrder, SortOrder, Sortable};

// Impl traits for &mut [T].

impl<'a, T> Collection for &'a mut [T] {
    type Item = T;
}

impl<'a, T> Sortable for &'a mut [T] {
    fn sort<F>(&mut self, f: F)
        where F: FnMut(&Self::Item, &Self::Item) -> Ordering
    {
        self.sort_by(f);
    }
}

impl<'a, T, O> SearchableByOrder<O> for &'a mut [T] where
    O: SortOrder<T>
{
    fn search(&self, a: &T) -> Result<usize, usize>
    {
        self.binary_search_by(|b| O::cmp(a, b))
    }
}

impl<'a, T> RetainsOrder for &'a mut [T] {}

// Impl traits for &[T].

impl<'a, T> Collection for &'a [T] {
    type Item = T;
}

impl<'a, T, O> SearchableByOrder<O> for &'a [T]
    where O: SortOrder<T>
{
    fn search(&self, a: &T) -> Result<usize, usize>
    {
        self.binary_search_by(|b| O::cmp(a, b))        
    }
}

impl<'a, T> RetainsOrder for &'a [T] {}

// Impl traits for [T;n]

macro_rules! arr_traits {
    ($($x:expr)*) => {
        $(
            impl<T> Collection for [T;$x] {
                type Item = T;
            }

            impl<T> Sortable for [T;$x]
            {
                fn sort<F>(&mut self, f: F)
                where
                    F: FnMut(&Self::Item, &Self::Item) -> Ordering
                {
                    self.sort_by(f);
                }
            }

            impl<T,O> SearchableByOrder<O> for [T;$x]
                where O: SortOrder<T>
            {
                fn search(&self, a: &T) -> Result<usize, usize>
                {
                    self.binary_search_by(|b| O::cmp(a,b))
                }
            }

            impl<T> RetainsOrder for [T;$x] {}
        )*
    }
}

arr_traits!(
     1  2  3  4  5  6  7  8
     9 10 11 12 13 14 15 16
    17 18 19 20 21 22 23 24
    25 26 27 28 29 30 31 32
);