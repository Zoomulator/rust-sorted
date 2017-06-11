use std::cmp::Ordering;
use super::{Collection, RetainsOrder, Sortable, SortedInsert, SortOrder, SearchableByOrder};

impl<T> Collection for Vec<T> {
    type Item = T;
}

impl<T> RetainsOrder for Vec<T> {}

impl<T> Sortable for Vec<T> {
    fn sort<F>(&mut self, f: F)
        where F: FnMut(&Self::Item, &Self::Item) -> Ordering
    {
        self.sort_by(f);
    }
}

impl<T, O> SearchableByOrder<O> for Vec<T>
    where O: SortOrder<T>
{
    fn search(&self, a: &T) -> Result<usize, usize> {
        self.binary_search_by(|b| O::cmp(b, a))
    }
}

impl<T, O> SortedInsert<O> for Vec<T>
    where O: SortOrder<T>,
          Self: Collection<Item = T> + SearchableByOrder<O>
{
    fn insert(&mut self, x: T) {
        let i = match self.search(&x) {
            Ok(i) => i,
            Err(i) => i,
        };
        self.insert(i, x);
    }
}
