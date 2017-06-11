use std::cmp::Ordering;
use super::{SortOrder, IsSorted};


pub struct Union<I, J>
    where I: Iterator + IsSorted,
          J: Iterator + IsSorted
{
    i: I,
    j: J,
    a: Option<I::Item>,
    b: Option<J::Item>,
}


impl<I, J> IsSorted for Union<I, J>
    where I: Iterator + IsSorted,
          I::Ordering: SortOrder<I::Item>,
          J: Iterator + IsSorted<Ordering = I::Ordering>
{
    type Ordering = I::Ordering;
}


impl<I, J> Iterator for Union<I, J>
    where I: Iterator + IsSorted,
          J: Iterator<Item = I::Item> + IsSorted,
          I::Item: Ord,
          I::Ordering: SortOrder<I::Item>
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
    where Self: Iterator + Sized + IsSorted
{
    fn union<J>(self, J) -> Union<Self, J>
        where J: IsSorted<Ordering = Self::Ordering> + Iterator<Item = Self::Item>;
}

impl<I> UnionExt for I
    where I: IsSorted + Iterator
{
    fn union<J>(self, j: J) -> Union<Self, J>
        where J: IsSorted<Ordering = Self::Ordering> + Iterator<Item = Self::Item>
    {
        Union {
            i: self,
            j,
            a: None,
            b: None,
        }
    }
}
