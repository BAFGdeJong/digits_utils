/// Provides the maximum number of digits for a type.
pub trait DigitSize {
    /// The maximum number of decimal digits this type can represent.
    const MAX_DIGITS: usize;
}

impl DigitSize for i8    { const MAX_DIGITS: usize = 3; }
impl DigitSize for u8    { const MAX_DIGITS: usize = 3; }
impl DigitSize for i16   { const MAX_DIGITS: usize = 5; }
impl DigitSize for u16   { const MAX_DIGITS: usize = 5; }
impl DigitSize for i32   { const MAX_DIGITS: usize = 10; }
impl DigitSize for u32   { const MAX_DIGITS: usize = 10; }
impl DigitSize for i64   { const MAX_DIGITS: usize = 20; }
impl DigitSize for u64   { const MAX_DIGITS: usize = 20; }
impl DigitSize for i128  { const MAX_DIGITS: usize = 39; }
impl DigitSize for u128  { const MAX_DIGITS: usize = 39; }
impl DigitSize for usize { const MAX_DIGITS: usize = 20; }
