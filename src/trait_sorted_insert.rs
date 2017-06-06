use super::{
    Sortable,
    SortOrder,
};


pub trait SortedInsert<T>: Sortable {
    fn insert<O>(&mut self, Self::Item)
    where O: SortOrder<T>;
}


impl<T> SortedInsert<T> for Vec<T>
where
    Self: Sortable<Item=T>,
    T: Ord
{
    fn insert<O>(&mut self, x: Self::Item)
    where
        O: SortOrder<T>
    {
        let i = match Sortable::search(self, &x, O::cmp) {
            Ok(i) => i, Err(i) => i
        };
        self.insert(i, x);
    }
}