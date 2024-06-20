//! A stable hashing algorithm used by rustc

#![cfg_attr(feature = "nightly", feature(hasher_prefixfree_extras))]
#![deny(clippy::arithmetic_side_effects)]

mod int_overflow;
mod sip128;
mod stable_hasher;

#[doc(inline)]
pub use stable_hasher::StableHasher;

#[doc(inline)]
pub use stable_hasher::StableHasherResult;
