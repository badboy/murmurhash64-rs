# MurmurHash2 (64bit) implementation

Based on the implementation for Redis
([antirez/redis src/hyperloglog.c](https://github.com/antirez/redis/blob/93eed9ae0163e328c33b16ab9ea3c4fbe0f98674/src/hyperloglog.c#L390-L439))

More info and different implementations available at:
<https://sites.google.com/site/murmurhash/>

[Documentation](http://badboy.github.io/murmurhash64-rs/murmurhash64/)

## Build

```
cargo build --release
```

## Usage

```rust
use murmurhash64::murmur_hash64a;

fn main() {
    let key = "Pizza & Mandolino";
    let seed = 2915580697;

    let hash = murmur_hash64a(key.as_bytes(), seed);
}

```

As a `Hasher`

```rust
use std::collections::HashMap;
use murmurhash64::{MurmurHasher,RandomMurmurState};
use std::default::Default;

fn main() {
    let mut hashmap : HashMap<_, _, RandomMurmurState> = Default::default();
    hashmap.insert("abc", 123);
    hashmap.insert("def", 456);
    assert_eq!(Some(&123), hashmap.get("abc"));
    assert_eq!(Some(&456), hashmap.get("def"));
}
```

## Tests

Run tests with:

```
cargo test
```

## Contribute

If you find bugs or want to help otherwise, please [open an issue](https://github.com/badboy/murmurhash64-rs/issues).

## License

BSD. See [LICENSE](LICENSE).  
