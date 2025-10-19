Easy-to-use methods for getting the array or vector of digits from integer types, with optional no_std and Vec support.

```
# Using the default std features (default)
[dependencies]
to_digits = "0.1.5"

# Using no_std
[dependencies]
to_digits = { version = "0.1.5", default-features = false, features = ["no_std"] }
```

Example:
```rust
use to_digits::ToDigits;
use to_digits::ToDigitsVec;
use to_digits::DigitSize;

// Can be of any signed / unsigned type
// Array
let x: u8 = 123;
let digits: [u8; u8::MAX_DIGITS] = x.to_digits();
assert_eq!(digits, [1, 2, 3]);

let x: u8 = 123;
let digits: [u8; 2] = x.to_digits();
assert_eq!(digits, [2, 3]);

let x: u8 = 123;
let reversed: [u8; u8::MAX_DIGITS] = x.to_digits_reversed();
assert_eq!(reversed, [3, 2, 1]);

let x: u8 = 123;
let reversed: [u8; 2] = x.to_digits_reversed();
assert_eq!(reversed, [3, 2]);

// Vector; not included in no_std
let x: u8 = 123;
let digits = x.to_digits_vec();
assert_eq!(reversed, vec![2, 5, 5]);

let x: u8 = 123;
let reversed = x.to_digits_reversed_vec();
assert_eq!(reversed, [5, 2, 2]);