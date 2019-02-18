# Bidrectional iterators

```rust
use bidir_iter::*;

let a: &[i64] = &[1, 2, 3];
let mut iter = a.bidir_iter();

assert_eq!(iter.next(), Some(&1));
assert_eq!(iter.next(), Some(&2));
assert_eq!(iter.next(), Some(&3));
assert_eq!(iter.prev(), Some(&2));
assert_eq!(iter.prev(), Some(&1));
assert_eq!(iter.prev(), None);
assert_eq!(iter.next(), Some(&1));
```
