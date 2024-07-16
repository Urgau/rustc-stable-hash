use super::*;

#[test]
fn simple_blake2s() {
    let mut hasher = Blake2sHasher256::default();
    hasher.write_u16(0xFF);
    hasher.write_u32(0xFF);
    let _h = hasher.finish();
}
