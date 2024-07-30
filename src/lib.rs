//! A stable hashing algorithm used by rustc

#![cfg_attr(feature = "nightly", feature(hasher_prefixfree_extras))]
#![deny(clippy::missing_safety_doc)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unreachable_pub)]

#[cfg(feature = "blake2")]
mod blake2;
mod int_overflow;
mod sip128;
mod stable_hasher;

/// Hashers collection
pub mod hashers {
    #[doc(inline)]
    pub use super::sip128::{SipHasher128, SipHasher128Hash};

    #[cfg(feature = "blake2")]
    #[doc(inline)]
    pub use super::blake2::{Blake2s256Hash, Blake2sHasher256};

    /// Stable 128-bits Sip Hasher
    ///
    /// [`StableHasher`] version of [`SipHasher128`].
    ///
    /// [`StableHasher`]: super::StableHasher
    pub type StableSipHasher128 = super::StableHasher<SipHasher128>;

    /// Stable 256-bits Blake2s Hasher
    #[cfg(feature = "blake2")]
    pub type StableBlake2sHasher256 = super::StableHasher<Blake2sHasher256>;
}

#[doc(inline)]
pub use stable_hasher::StableHasher;

#[doc(inline)]
pub use stable_hasher::{FromStableHash, IntoStableHash};

#[doc(inline)]
pub use stable_hasher::ExtendedHasher;

#[doc(inline)]
pub use hashers::{SipHasher128Hash, StableSipHasher128};
