#[cfg(all(feature = "usize8", any(feature = "usize16", feature = "usize32", feature = "usize64", feature = "usize128")))]
compile_error!("Features 'usize8' and another usize feature cannot be enabled together!");

#[cfg(all(feature = "usize16", any(feature = "usize8", feature = "usize32", feature = "usize64", feature = "usize128")))]
compile_error!("Features 'usize16' and another usize feature cannot be enabled together!");

#[cfg(all(feature = "usize32", any(feature = "usize8", feature = "usize16", feature = "usize64", feature = "usize128")))]
compile_error!("Features 'usize32' and another usize feature cannot be enabled together!");

#[cfg(all(feature = "usize64", any(feature = "usize8", feature = "usize16", feature = "usize32", feature = "usize128")))]
compile_error!("Features 'usize64' and another usize feature cannot be enabled together!");

#[cfg(all(feature = "usize128", any(feature = "usize8", feature = "usize16", feature = "usize32", feature = "usize64")))]
compile_error!("Features 'usize128' and another usize feature cannot be enabled together!");

#[cfg(not(any(feature = "usize8", feature = "usize16", feature = "usize32", feature = "usize64", feature = "usize128")))]
compile_error!("You must enable exactly one of the usize features: usize8, usize16, usize32, usize64, or usize128.");

pub mod to_digits;
pub mod digit_size;