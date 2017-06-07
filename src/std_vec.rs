use std::cmp::Ordering;
use super::{Sortable, SortedInsert, SortOrder};

impl<T> Sortable for Vec<T> {
    type Item = T;

    fn sort<F>(&mut self, f: F)
        where F: FnMut(&Self::Item, &Self::Item) -> Ordering
    {
        self.sort_by(f);
    }

    fn search<F>(&self, a: &T, mut f: F) -> Result<usize, usize>
        where F: FnMut(&Self::Item, &Self::Item) -> Ordering
    {
        self.binary_search_by(|b| f(b, a))
    }
}

impl<T> SortedInsert<T> for Vec<T>
    where Self: Sortable<Item = T>,
          T: Ord
{
    fn insert<O>(&mut self, x: Self::Item)
        where O: SortOrder<T>
    {
        let i = match Sortable::search(self, &x, O::cmp) {
            Ok(i) => i,
            Err(i) => i,
        };
        self.insert(i, x);
    }
}