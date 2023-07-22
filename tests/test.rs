use drain_at_sorted_unchecked::drain_at_sorted_unchecked;

fn main() {
    let mut v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
    // Safety:
    // 
    // [x] The indices are sorted in ascending order.
    // [x] The indices are within bounds of the vector.
    // [x] The indices are unique.
    // [x] Items of type i32 are trivially movable.
    unsafe { drain_at_sorted_unchecked(&mut v, [2,4,6]); }
    assert_eq!(v, [0, 1, 3, 5, 7, 8]);
}
