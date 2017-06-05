
pub trait SortOrder<T> : Clone + Copy {
    type Key;
    fn key(&T) -> Self::Key;
    fn sort(&mut [T]);
    fn search(&[T], &Self::Key) -> Result<usize, usize>;
}

#[derive(Debug, Clone, Copy)]
pub struct DefaultOrder;

impl<T> SortOrder<T> for DefaultOrder
where T: Ord + Clone {
    type Key = T;

    fn key(k: &T) -> Self::Key {
        k.clone()
    }

    fn sort(s: &mut [T]) {
        s.sort();
    }

    fn search(s: &[T], x: &T) -> Result<usize,usize> {
        s.binary_search(x)
    }
}