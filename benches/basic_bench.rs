extern crate test;
extern crate murmurhash64;

use test::Bencher;
use murmurhash64::murmur_hash64a;

#[bench]
fn benchmark_murmur(b: &mut Bencher) {
    let key = "lorem ipsum dolor sit amet".as_bytes();
    let seed = 0xadc83b19;
    b.iter(|| {
        murmur_hash64a(key, seed);
    })
}
