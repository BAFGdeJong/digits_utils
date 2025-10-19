Easy-to-use methods for getting the array or vector of digits from integer types, with optional no_std and Vec support.

```
features = ["no_std", "bit_hacks"]

# Using the default std features (default)
[dependencies]
to_digits = "0.2.2"

# Using no_std
[dependencies]
to_digits = { version = "0.2.2", features = ["no_std"] }
```

Example:
```rust
use digits_utils::to_digits::ToDigits;
use digits_utils::to_digits::ToDigitsCollection;
use digits_utils::to_digits::DigitSize

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
assert_eq!(reversed, vec![1, 2, 3]);

let x: u8 = 123;
let reversed = x.to_digits_vec_reversed();
assert_eq!(reversed, [3, 2, 1]);