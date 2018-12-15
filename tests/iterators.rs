extern crate sorted;
use sorted::*;

#[test]
fn union_iter() {
    let v0 = AscendingOrder::by_sorting(vec![1, 3, 4, 5, 5, 10]);
    let v1 = AscendingOrder::by_sorting(vec![0, 2, 4, 4, 7, 11, 11, 23]);
    let v2 = Sorted::<Vec<u32>, _>::from_iter(v1.into_iter().union(v0.into_iter()));
    assert_eq!(
        [0, 1, 2, 3, 4, 4, 4, 5, 5, 7, 10, 11, 11, 23],
        v2.as_slice()
    );
}

#[test]
fn difference_iter() {
    let v0 = AscendingOrder::by_sorting(vec![1, 2, 2, 3, 4, 4, 5]);
    let v1 = AscendingOrder::by_sorting(vec![0, 1, 3, 3, 4, 6, 7, 7, 9]);
    let v2 = Sorted::<Vec<_>, _>::from_iter(v0.into_iter().difference(v1.into_iter()));
    assert_eq!([2, 2, 5], v2.as_slice());
}

#[test]
fn intersection_iter() {
    let v0 = AscendingOrder::by_sorting(vec![1, 1, 2, 5, 5, 6, 6, 8, 9, 9]);
    let v1 = AscendingOrder::by_sorting(vec![1, 1, 3, 6, 7, 9, 9, 10]);
    let v2 = Sorted::<Vec<_>, _>::from_iter(v0.into_iter().intersection(v1.into_iter()));
    assert_eq!([1, 1, 6, 9, 9], v2.as_slice());
}
