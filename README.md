Easy-to-use methods for getting the array or vector of digits from integer types, with optional no_std and Vec support.

```
# Using the default std features (default)
[dependencies]
to_digits = "0.2.2"

# Using no_std
[dependencies]
to_digits = { version = "0.2.2", features = ["no_std"] }

[features]
default = ["usize64", "enable_8_bit", "enable_16_bit", "enable_32_bit", "enable_64_bit", "enable_128_bit"]

# Use 8-bit platform usize and isize
usize8
# Use 16-bit platform usize and isize
usize16
# Use 32-bit platform usize and isize (for 32-bit systems)
usize32
# Use 64-bit platform usize and isize (default for most targets)
usize64
# Use 128-bit platform usize and isize (for experimental wide-pointer architectures)
usize128

# Enables use for 8-bit integers.
enable_8_bit
# Enables use for 16-bit integers.
enable_16_bit
# Enables use for 32-bit integers.
enable_32_bit
# Enables use for 64-bit integers.
enable_64_bit
# Enables use for 128-bit integers.
enable_128_bit

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