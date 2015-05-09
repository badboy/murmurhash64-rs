#![feature(test)]

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

#[cfg(test)]
mod bench {
    extern crate test;

    use std::hash::Hasher;
    use murmurhash64::MurmurHasher;

    fn hasher_bench<H>(b: &mut test::Bencher, mut hasher: H, len: usize)
        where H: Hasher
    {
        let bytes: Vec<_> = (0..100).cycle().take(len).collect();
        b.bytes = bytes.len() as u64;
        b.iter(|| {
            hasher.write(&bytes);
            hasher.finish()
        });
    }

    fn murmurhash_bench(b: &mut test::Bencher, len: usize) {
        hasher_bench(b, MurmurHasher::with_seed(0), len)
    }

    #[bench]
    fn murmurhash_megabyte(b: &mut test::Bencher) { murmurhash_bench(b, 1024*1024) }

    #[bench]
    fn murmurhash_1024_byte(b: &mut test::Bencher) { murmurhash_bench(b, 1024) }

    #[bench]
    fn murmurhash_512_byte(b: &mut test::Bencher) { murmurhash_bench(b, 512) }

    #[bench]
    fn murmurhash_256_byte(b: &mut test::Bencher) { murmurhash_bench(b, 256) }

    #[bench]
    fn murmurhash_128_byte(b: &mut test::Bencher) { murmurhash_bench(b, 128) }

    #[bench]
    fn murmurhash_32_byte(b: &mut test::Bencher) { murmurhash_bench(b, 32) }

    #[bench]
    fn murmurhash_16_byte(b: &mut test::Bencher) { murmurhash_bench(b, 16) }

    #[bench]
    fn murmurhash_4_byte(b: &mut test::Bencher) { murmurhash_bench(b, 4) }

    #[bench]
    fn murmurhash_1_byte(b: &mut test::Bencher) { murmurhash_bench(b, 1) }

    #[bench]
    fn murmurhash_0_byte(b: &mut test::Bencher) { murmurhash_bench(b, 0) }
}
