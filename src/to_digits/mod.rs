#[macro_use]
mod abs_assigner;
mod to_digits;

/// Converts a number into an array of its digits.
///
/// `SIZE` specifies the length of the array. Leading zeros are used if the
/// number has fewer digits than `SIZE`.
pub trait ToDigits<const SIZE: usize> {
    /// Returns the digits in **normal order** (most significant first).
    /// # Examples
    ///
    /// ```
    /// use digits_utils::digit_size::DigitSize;
    /// use digits_utils::to_digits::ToDigits;
    ///
    /// let x: u8 = 123;
    /// let digits: [u8; u8::MAX_DIGITS] = x.to_digits();
    /// assert_eq!(digits, [1, 2, 3]);
    ///
    /// let x: u8 = 123;
    /// let digits: [u8; 2] = x.to_digits();
    /// assert_eq!(digits, [2, 3]);
    /// ```
    fn to_digits(&self) -> [u8; SIZE];

    /// Returns the digits in **reversed order** (the least significant first).
    /// # Examples
    ///
    /// ```
    /// use digits_utils::digit_size::DigitSize;
    /// use digits_utils::to_digits::ToDigits;
    ///
    /// let x: u8 = 123;
    /// let reversed: [u8; u8::MAX_DIGITS] = x.to_digits_reversed();
    /// assert_eq!(reversed, [3, 2, 1]);
    ///
    /// let x: u8 = 123;
    /// let reversed: [u8; 2] = x.to_digits_reversed();
    /// assert_eq!(reversed, [3, 2]);
    /// ```
    fn to_digits_reversed(&self) -> [u8; SIZE];
}

#[cfg(not(feature = "no_std"))]
use std::iter::FromIterator;
#[cfg(not(feature = "no_std"))]
mod to_digits_collection;

#[cfg(not(feature = "no_std"))]
/// Converts a number into a `FromIterator` of its digits.
pub trait ToDigitsCollection {
    /// Returns the digits in **normal order** (most significant first) as a `Vec<u8>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use digits_utils::to_digits::ToDigitsCollection;
    ///
    /// let x: u8 = 123;
    /// let digits = x.to_digits_into_vec();
    /// assert_eq!(reversed, vec![1, 2, 3]);
    /// ```
    fn to_digits_into_vec(&self) -> Vec<u8>;

    /// Returns the digits of the integer in **normal order** (most significant first)
    /// as a collection of type `T`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::LinkedList;
    /// use digits_utils::to_digits::ToDigitsCollection;
    ///
    /// let x: u8 = 123;
    /// let digits: LinkedList<u8> = x.to_digits_into();
    /// let digits_vec: Vec<u8> = digits.into_iter().collect();
    /// assert_eq!(digits_vec, vec![1, 2, 3]);
    /// ```
    fn to_digits_into<T>(&self) -> T
    where
        T: FromIterator<u8>;

    /// Returns the digits in **reversed order** (the least significant first) as a `Vec<u8>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use digits_utils::to_digits::ToDigitsCollection;
    ///
    /// let x: u8 = 123;
    /// let reversed = x.to_digits_reversed_into_vec();
    /// assert_eq!(reversed, [3, 2, 1]);
    /// ```
    fn to_digits_reversed_into_vec(&self) -> Vec<u8>;

    /// Returns the digits of the integer in **reversed order** (the least significant first)
    /// as a collection of type `T`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::LinkedList;
    ///
    /// let x: u8 = 123;
    /// let digits: LinkedList<u8> = x.to_digits_reversed_into();
    /// let digits_vec: Vec<u8> = digits.into_iter().collect();
    /// assert_eq!(digits_vec, vec![3, 2, 1]);
    /// ```
    fn to_digits_reversed_into<T>(&self) -> T
    where
        T: FromIterator<u8>;
}
