# drain_at_sorted_unchecked

[![Crates.io](https://img.shields.io/crates/v/drain_at_sorted_unchecked)](https://crates.io/crates/drain_at_sorted_unchecked)
[![Downloads](https://img.shields.io/crates/d/drain_at_sorted_unchecked.svg)](https://crates.io/crates/drain_at_sorted_unchecked)
[![Documentation](https://docs.rs/drain_at_sorted_unchecked/badge.svg)](https://docs.rs/drain_at_sorted_unchecked)
[![License](https://img.shields.io/crates/l/drain_at_sorted_unchecked)](https://crates.io/crates/drain_at_sorted_unchecked)
[![Dependency Status](https://deps.rs/repo/github/JohnScience/drain_at_sorted_unchecked/status.svg)](https://deps.rs/repo/github/JohnScience/drain_at_sorted_unchecked)

Terribly unsafe but highly efficient function that allows removing items from a vector with few moves.

## Example

```rust
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
```

## Safety

* The indices must be sorted in ascending order.
* The indices must be within bounds of the collection.
* The indices must be unique.
* The items must be trivially [movable].

## Notes

At the moment of writing the algorithm is implemented only for a vector because that's what the author needed. Extending the algorithm to other contiguous collections (e.g. [`heapless::Vec`] or [`arrayvec::ArrayVec`]) should be straightforward.

The library is quite heavily tested but there's still a slim chance that there are some bugs. Please report them if you find any.

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

[movable]: https://doc.rust-lang.org/std/pin/#:~:text=By%20default%2C%20all%20types%20in,can%20use%20mem%3A%3Aswap%20.
[`heapless::Vec`]: https://docs.rs/heapless/latest/heapless/struct.Vec.html
[`arrayvec::ArrayVec`]: https://docs.rs/arrayvec/latest/arrayvec/struct.ArrayVec.html
