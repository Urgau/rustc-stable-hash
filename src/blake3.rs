//! This is the adapted version of the official Blake 3 implementation providing 256 bit hashes.

use std::hash::Hasher;

use crate::ExtendedHasher;

/// Hash type for the [`Blake3Hash`], providing 256 bit hashses.
pub type Blake3Hash = blake3::Hash;

/// Blake 3 implementation
#[derive(Debug, Clone)]
pub struct Blake3Hasher {
    state: blake3::Hasher,
}

impl Default for Blake3Hasher {
    fn default() -> Blake3Hasher {
        Blake3Hasher {
            state: Default::default(),
        }
    }
}

impl ExtendedHasher for Blake3Hasher {
    type Hash = Blake3Hash;

    #[inline]
    fn finish(self) -> Self::Hash {
        self.state.finalize()
    }
}

impl Hasher for Blake3Hasher {
    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        self.state.update(bytes);
    }

    #[inline]
    fn finish(&self) -> u64 {
        let hash = self.state.finalize();

        // todo: godbolt
        let [a0, a1, a2, a3, a4, a5, a6, a7, b0, b1, b2, b3, b4, b5, b6, b7, c0, c1, c2, c3, c4, c5, c6, c7, d0, d1, d2, d3, d4, d5, d6, d7] =
            *hash.as_bytes();

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
