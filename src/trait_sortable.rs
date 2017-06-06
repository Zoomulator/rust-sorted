use std::cmp::Ordering;

pub trait Sortable {
    type Item;

    fn sort<F>(&mut self, f: F)
    where F: FnMut(&Self::Item, &Self::Item) -> Ordering;

    fn search<F>(&self, &Self::Item, F) -> Result<usize, usize>
    where F: FnMut(&Self::Item, &Self::Item) -> Ordering;
}

impl<T> Sortable for Vec<T>
{
    type Item = T;

    fn sort<F>(&mut self, f: F)
    where
        F: FnMut(&Self::Item, &Self::Item) -> Ordering
    {
        self.sort_by(f);
    }

    fn search<F>(&self, a: &T, mut f: F) -> Result<usize, usize>
    where
        F: FnMut(&Self::Item, &Self::Item) -> Ordering
    {
        self.binary_search_by(|b| f(b,a))
    }
}

impl<'a,T> Sortable for &'a mut [T]
{
    type Item = T;

    fn sort<F>(&mut self, f: F)
    where
        F: FnMut(&Self::Item, &Self::Item) -> Ordering
    {
        self.sort_by(f);
    }

    fn search<F>(&self, a: &T, mut f: F) -> Result<usize, usize>
    where
        F: FnMut(&Self::Item, &Self::Item) -> Ordering
    {
        self.binary_search_by(|b| f(a,b))
    }
}

macro_rules! sortable_arr {
    ($($x:expr)*) => {
        $(
            impl<T> Sortable for [T;$x]
            {
                type Item = T;

                fn sort<'a,F>(&'a mut self, f: F)
                where
                    F: FnMut(&Self::Item, &Self::Item) -> Ordering
                {
                    self.sort_by(f);
                }

                fn search<'a,F>(&'a self, a: &'a T, mut f: F) -> Result<usize, usize>
                where
                    F: FnMut(&Self::Item, &Self::Item) -> Ordering
                {
                    self.binary_search_by(|b| f(a,b))
                }
            }
        )*
    }
}

sortable_arr!(
     1  2  3  4  5  6  7  8
     9 10 11 12 13 14 15 16
    17 18 19 20 21 22 23 24
    25 26 27 28 29 30 31 32
);