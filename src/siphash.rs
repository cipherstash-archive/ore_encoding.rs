use std::hash::Hasher;
use siphasher::sip128::SipHasher;

/// Generate a siphash from an arbitrary length array of bytes.
pub fn siphash(bytes: &[u8]) -> u64 {
  const KEY: [u8; 16] = [0xde, 0xad, 0xbe, 0xef, 0xca, 0xfe, 0xba, 0xbe, 0x8b, 0xad, 0xf0, 0x0d, 0x1b, 0xad, 0xb0, 0x02];
  let mut hasher = SipHasher::new_with_key(&KEY);
  hasher.write(bytes);
  hasher.finish()
}