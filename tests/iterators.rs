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