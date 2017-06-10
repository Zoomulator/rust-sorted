extern crate sorted;
use sorted::*;



#[test]
fn union_iter() {
    let v0 = AscendingOrder::by_sorting(vec![1,3,4,5,10]);
    let v1 = AscendingOrder::by_sorting(vec![0,2,4,4,7,11,23]);
    let v2 = Sorted::<Vec<u32>,_>::from_iter(v1.into_iter().union(v0.into_iter()));
    assert_eq!(
        [0,1,2,3,4,4,4,5,7,10,11,23],
        v2.as_slice()
    );
}

#[test]
fn difference_iter() {
    let v0 = AscendingOrder::by_sorting(vec![1,1,2,2,3,4,4,5]);
    let v1 = AscendingOrder::by_sorting(vec![0,1,3,3,4,7,9]);
    let v2 = Sorted::<Vec<_>,_>::from_iter(
        v0.into_iter().difference(v1.into_iter())
    );
    assert_eq!(
        [2,2,5],
        v2.as_slice()
    );
}