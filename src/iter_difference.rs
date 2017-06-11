use std::cmp::Ordering;
use super::{SortOrder, IsSorted};


pub struct Difference<I, J>
    where I: Iterator + IsSorted,
          J: Iterator + IsSorted
{
    i: I,
    j: J,
    a: Option<I::Item>,
    b: Option<J::Item>,
}

impl<I, J> IsSorted for Difference<I, J>
    where I: Iterator + IsSorted,
          I::Ordering: SortOrder<I::Item>,
          J: Iterator<Item = I::Item> + IsSorted<Ordering = I::Ordering>
{
    type Ordering = I::Ordering;
}

impl<I, J> Iterator for Difference<I, J>
    where I: Iterator + IsSorted,
          I::Item: Ord,
          I::Ordering: SortOrder<I::Item>,
          J: Iterator<Item = I::Item> + IsSorted<Ordering = I::Ordering>
{
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let a = self.a.take().or_else(|| self.i.next());
            let b = self.b.take().or_else(|| self.j.next());
            match (a, b) {
                (Some(a), Some(b)) => {
                    match I::Ordering::cmp(&a, &b) {
                        Ordering::Less => {
                            self.b = Some(b);
                            return Some(a);
                        }
                        Ordering::Greater => {
                            self.a = Some(a);
                        }
                        Ordering::Equal => {
                            self.b = Some(b);
                        }
                    }
                }
                (None, None) => return None,
                (a, None) => return a,
                (None, _) => (),
            }
        }
    }
}


pub trait DifferenceExt
    where Self: Sized + Iterator + IsSorted
{
    fn difference<J>(self, j: J) -> Difference<Self, J>
        where J: Sized + Iterator<Item = Self::Item> + IsSorted<Ordering = Self::Ordering>;
}

impl<I> DifferenceExt for I
    where I: Sized + Iterator + IsSorted
{
    fn difference<J>(self, j: J) -> Difference<Self, J>
        where J: Sized + Iterator<Item = Self::Item> + IsSorted<Ordering = Self::Ordering>
    {
        Difference {
            i: self,
            j,
            a: None,
            b: None,
        }
    }
}
