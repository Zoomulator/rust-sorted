use std::cmp::Ordering;
use super::{SortOrder, SortedIterator};


pub struct Union<I, J>
    where I: SortedIterator,
          I::Ordering: SortOrder<I::Item>,
          J: SortedIterator<Item = I::Item, Ordering = I::Ordering>
{
    i: I,
    j: J,
    a: Option<I::Item>,
    b: Option<J::Item>,
}


impl<I, J> SortedIterator for Union<I, J>
    where I: SortedIterator,
          I::Ordering: SortOrder<I::Item>,
          J: SortedIterator<Item = I::Item, Ordering = I::Ordering>
{
    type Ordering = I::Ordering;
}


impl<I, J> Iterator for Union<I, J>
    where I: SortedIterator,
          I::Ordering: SortOrder<I::Item>,
          J: SortedIterator<Item = I::Item, Ordering = I::Ordering>
{
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        let a = self.a.take().or_else(|| self.i.next());
        let b = self.b.take().or_else(|| self.j.next());

        match (a, b) {
            (a @ Some(_), None) => a,
            (None, b @ Some(_)) => b,
            (Some(a), Some(b)) => {
                if I::Ordering::cmp(&a, &b) == Ordering::Less {
                    self.b = Some(b);
                    Some(a)
                } else {
                    self.a = Some(a);
                    Some(b)
                }
            }
            (None, None) => None,
        }
    }
}


pub trait UnionExt
    where Self: SortedIterator + Sized,
          Self::Ordering: SortOrder<Self::Item>
{
    fn union<J>(self, J) -> Union<Self, J>
        where J: SortedIterator<Ordering = Self::Ordering, Item = Self::Item>;
}

impl<I> UnionExt for I
    where I: SortedIterator,
          I::Ordering: SortOrder<I::Item>
{
    fn union<J>(self, j: J) -> Union<Self, J>
        where J: SortedIterator<Ordering = Self::Ordering, Item = Self::Item>
    {
        Union {
            i: self,
            j,
            a: None,
            b: None,
        }
    }
}
