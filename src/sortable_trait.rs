use std::cmp::Ordering;

pub trait Sortable {
    type Item;

    fn sort<'a,F>(&'a mut self, f: F)
    where F: FnMut(&Self::Item, &Self::Item) -> Ordering;

    fn search<'a,F>(&'a self, &'a Self::Item, F) -> Result<usize, usize>
    where F: FnMut(&Self::Item, &Self::Item) -> Ordering;
}

impl<T> Sortable for Vec<T>
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
        self.binary_search_by(|b| f(b,a))
    }
}


impl<T> Sortable for [T]
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