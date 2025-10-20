#[cfg(feature = "bit_hacks")]
macro_rules! abs_assigner {
    (true, $x:expr, $t:ty) => {{let mask = $x >> (<$t>::BITS - 1); ($x ^ mask) - mask}};
    (false, $x:expr, $t:ty) => { $x };
}

#[cfg(not(feature = "bit_hacks"))]
macro_rules! abs_assigner {
    (true, $x:expr, $t:ty) => { if $x < 0 { -$x } else { $x } };
    (false, $x:expr, $t:ty) => { $x };
}
