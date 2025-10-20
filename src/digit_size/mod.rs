mod digit_size;


/// Provides the maximum number of digits for a type.
pub trait DigitSize {
    /// The maximum number of decimal digits this type can represent.
    const MAX_DIGITS: usize;
}
