use super::{Collection, SortOrder};

// New name SearchableByOrder

/// Defines an interface for collections that can be searched when sorted.
pub trait SearchableByOrder<O>: Collection
    where O: SortOrder<Self::Item>
{
    fn search(&self, &Self::Item) -> Result<usize, usize>;
}
