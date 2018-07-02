# divide_range

Split a range evenly into an iterator of smaller ranges.

```rust
extern crate divide_range;
use divide_range::RangeDivisions;

let range = 1..18;
let mut iter = range.divide_evenly_into(5);

assert_eq!(Some(1..4), iter.next());
assert_eq!(Some(4..7), iter.next());
assert_eq!(Some(7..10), iter.next());
assert_eq!(Some(10..14), iter.next());
assert_eq!(Some(14..18), iter.next());
assert_eq!(None, iter.next());
```
