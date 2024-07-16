//! This is the adapted version of the Blake 2 hashing algos.

use blake2::Digest;
use std::hash::Hasher;

use crate::ExtendedHasher;

#[cfg(test)]
mod tests;

/// Hash type for the [`Blake2sHasher256`] providing 256-bits hashses.
pub struct Blake2s256Hash(pub [u8; 32]);

/// Blake2s 256-bits Hasher
#[derive(Debug, Clone, Default)]
pub struct Blake2sHasher256 {
    state: blake2::Blake2s256,
}

impl ExtendedHasher for Blake2sHasher256 {
    type Hash = Blake2s256Hash;

    #[inline]
    fn finish(self) -> Self::Hash {
        let mut hash = self.state.finalize();
        Blake2s256Hash(*hash.as_mut())
    }
}

impl Hasher for Blake2sHasher256 {
    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        self.state.update(bytes);
    }

    // TODO: Improve this function hashing-combine with a better thing
    #[inline]
    fn finish(&self) -> u64 {
        let mut hash = self.state.clone().finalize();

        let [a0, a1, a2, a3, a4, a5, a6, a7, b0, b1, b2, b3, b4, b5, b6, b7, c0, c1, c2, c3, c4, c5, c6, c7, d0, d1, d2, d3, d4, d5, d6, d7] =
            *hash.as_mut();

        let p0 = u64::from_ne_bytes([a0, a1, a2, a3, a4, a5, a6, a7]);
        let p1 = u64::from_ne_bytes([b0, b1, b2, b3, b4, b5, b6, b7]);
        let p2 = u64::from_ne_bytes([c0, c1, c2, c3, c4, c5, c6, c7]);
        let p3 = u64::from_ne_bytes([d0, d1, d2, d3, d4, d5, d6, d7]);

        p0.wrapping_mul(3)
            .wrapping_add(p1)
            .wrapping_add(p2)
            .wrapping_mul(p3)
            .to_le()
    }
}
