/// Provides the maximum number of digits for a type.
pub trait DigitSize {
    /// The maximum number of decimal digits this type can represent.
    const MAX_DIGITS: usize;
}

#[cfg(feature = "enable_8_bit")]
impl DigitSize for i8    { const MAX_DIGITS: usize = 3; }
#[cfg(feature = "enable_8_bit")]
impl DigitSize for u8    { const MAX_DIGITS: usize = 3; }
#[cfg(feature = "usize8")]
impl DigitSize for usize { const MAX_DIGITS: usize = 3; }
#[cfg(feature = "usize8")]
impl DigitSize for isize { const MAX_DIGITS: usize = 3; }

#[cfg(feature = "enable_16_bit")]
impl DigitSize for i16   { const MAX_DIGITS: usize = 5; }
#[cfg(feature = "enable_16_bit")]
impl DigitSize for u16   { const MAX_DIGITS: usize = 5; }
#[cfg(feature = "usize16")]
impl DigitSize for usize { const MAX_DIGITS: usize = 5; }
#[cfg(feature = "usize16")]
impl DigitSize for isize { const MAX_DIGITS: usize = 5; }

#[cfg(feature = "enable_32_bit")]
impl DigitSize for i32   { const MAX_DIGITS: usize = 10; }
#[cfg(feature = "enable_32_bit")]
impl DigitSize for u32   { const MAX_DIGITS: usize = 10; }
#[cfg(feature = "usize32")]
impl DigitSize for usize { const MAX_DIGITS: usize = 10; }
#[cfg(feature = "usize32")]
impl DigitSize for isize { const MAX_DIGITS: usize = 10; }

#[cfg(feature = "enable_64_bit")]
impl DigitSize for i64   { const MAX_DIGITS: usize = 20; }
#[cfg(feature = "enable_64_bit")]
impl DigitSize for u64   { const MAX_DIGITS: usize = 20; }
#[cfg(feature = "usize64")]
impl DigitSize for usize { const MAX_DIGITS: usize = 20; }
#[cfg(feature = "usize64")]
impl DigitSize for isize { const MAX_DIGITS: usize = 20; }

#[cfg(feature = "enable_128_bit")]
impl DigitSize for i128  { const MAX_DIGITS: usize = 39; }
#[cfg(feature = "enable_128_bit")]
impl DigitSize for u128  { const MAX_DIGITS: usize = 39; }
#[cfg(feature = "usize128")]
impl DigitSize for usize { const MAX_DIGITS: usize = 39; }
#[cfg(feature = "usize128")]
impl DigitSize for isize { const MAX_DIGITS: usize = 39; }
