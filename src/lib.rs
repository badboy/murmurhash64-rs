//! Implementation of MurmurHash2 (64bit version)
//!
//! Based on the implementation for Redis
//! ([antirez/redis src/hyperloglog.c](https://github.com/antirez/redis/blob/93eed9ae0163e328c33b16ab9ea3c4fbe0f98674/src/hyperloglog.c#L390-L439))
//!
//! More info and different implementations available at:
//! <https://sites.google.com/site/murmurhash/>
//!
//! # Basic Example
//!
//! ```rust
//! # use murmurhash64::murmur_hash64a;
//! let key = "Pizza & Mandolino";
//! let seed = 2915580697;
//!
//! let hash = murmur_hash64a(key.as_bytes(), seed);
//! ```

extern crate rand;

#[cfg(not(feature = "no-hasher"))]
pub use hasher::MurmurHasher;
#[cfg(not(feature = "no-hasher"))]
pub use hasher::MurmurState;
#[cfg(not(feature = "no-hasher"))]
pub use hasher::RandomMurmurState;

#[cfg(not(feature = "no-hasher"))]
mod hasher;

/// Hash the given key and the given seed.
///
/// Returns the resulting 64bit hash.
///
/// Example:
///
/// ```rust
/// # use murmurhash64::murmur_hash64a;
/// let key = "Pizza & Mandolino";
/// let seed = 2915580697;
///
/// let hash = murmur_hash64a(key.as_bytes(), seed);
/// ```
pub const fn murmur_hash64a(key: &[u8], seed: u64) -> u64 {
    let m : u64 = 0xc6a4a7935bd1e995;
    let r : u8 = 47;

    let len = key.len();
    let mut h : u64 = seed ^ ((len as u64).wrapping_mul(m));

    let endpos = len-(len&7);
    let mut i = 0;
    while i != endpos {
        let mut k : u64;

        k  = key[i+0] as u64;
        k |= (key[i+1] as u64) << 8;
        k |= (key[i+2] as u64) << 16;
        k |= (key[i+3] as u64) << 24;
        k |= (key[i+4] as u64) << 32;
        k |= (key[i+5] as u64) << 40;
        k |= (key[i+6] as u64) << 48;
        k |= (key[i+7] as u64) << 56;

        k = k.wrapping_mul(m);
        k ^= k >> r;
        k = k.wrapping_mul(m);
        h ^= k;
        h = h.wrapping_mul(m);

        i += 8;
    };

    let over = len & 7;
    if over == 7 { h ^= (key[i+6] as u64) << 48; }
    if over >= 6 { h ^= (key[i+5] as u64) << 40; }
    if over >= 5 { h ^= (key[i+4] as u64) << 32; }
    if over >= 4 { h ^= (key[i+3] as u64) << 24; }
    if over >= 3 { h ^= (key[i+2] as u64) << 16; }
    if over >= 2 { h ^= (key[i+1] as u64) << 8; }
    if over >= 1 { h ^= key[i+0] as u64; }
    if over >  0 { h = h.wrapping_mul(m); }

    h ^= h >> r;
    h = h.wrapping_mul(m);
    h ^= h >> r;
    h
}

#[test]
fn test_mumurhash() {
    assert_eq!(0, murmur_hash64a("".as_bytes(), 0));
    assert_eq!(0xc26e8bc196329b0f, murmur_hash64a("".as_bytes(), 10));
    assert_eq!(0x472ff7d324321dfe,
               murmur_hash64a("Pizza & Mandolino".as_bytes(), 2915580697));
}
