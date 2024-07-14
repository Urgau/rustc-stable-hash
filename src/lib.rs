//! A stable hashing algorithm used by rustc

#![cfg_attr(feature = "nightly", feature(hasher_prefixfree_extras))]
#![deny(clippy::missing_safety_doc)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unreachable_pub)]

#[cfg(feature = "blake3")]
mod blake3;
mod int_overflow;
mod sip128;
mod stable_hasher;

/// Hashers collection
pub mod hashers {
    #[doc(inline)]
    pub use super::sip128::{SipHasher128, SipHasher128Hash};

    #[cfg(feature = "blake3")]
    #[doc(inline)]
    pub use super::blake3::{Blake3Hash, Blake3Hasher};

    /// Stable 128-bits Sip Hasher
    ///
    /// [`StableHasher`] version of [`SipHasher128`].
    ///
    /// [`StableHasher`]: super::StableHasher
    pub type StableSipHasher128 = super::StableHasher<SipHasher128>;

    #[cfg(feature = "blake3")]
    pub type StableBlake3Hasher = super::StableHasher<Blake3Hasher>;
}

#[doc(inline)]
pub use stable_hasher::StableHasher;

#[doc(inline)]
pub use stable_hasher::FromStableHash;

#[doc(inline)]
pub use stable_hasher::ExtendedHasher;

#[doc(inline)]
pub use hashers::{SipHasher128Hash, StableSipHasher128};
