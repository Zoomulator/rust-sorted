
pub trait SortOrder<T> : Clone + Copy {
    fn sort(&mut [T]);
}

#[derive(Debug, Clone, Copy)]
pub struct DefaultOrder;

impl<T> SortOrder<T> for DefaultOrder
where T: Ord {
    fn sort(s: &mut [T]) {
        s.sort();
    }
}