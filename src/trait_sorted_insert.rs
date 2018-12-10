use super::{SearchableByOrder, SortOrder};

pub trait SortedInsert<O>: SearchableByOrder<O>
where
    O: SortOrder<Self::Item>,
{
    fn insert(&mut self, Self::Item);
}
