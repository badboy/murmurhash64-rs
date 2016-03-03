use std::default::Default;
use std::hash::Hasher;
use rand::{self,Rng};
use super::murmur_hash64a;
use std::hash::BuildHasher;

/// MurmurHash2 can also be used as the hash algorithm in a HashMap
/// (or similar).
/// It implements the necessary traits.
///
/// The new hasher traits are only available since Rust 1.7.0
/// To use them, enable the `hasher` feature in your build.
///
/// # Basic Example
///
/// ```rust
/// # use std::collections::HashMap;
/// # use std::default::Default;
/// # use murmurhash64::{MurmurHasher, MurmurState, RandomMurmurState};
/// let mut hashmap : HashMap<_, _, MurmurState> = Default::default();
/// hashmap.insert("abc", 123);
/// hashmap.insert("def", 456);
/// assert_eq!(Some(&123), hashmap.get("abc"));
/// assert_eq!(Some(&456), hashmap.get("def"));
///
/// let mut hashmap : HashMap<_, _, RandomMurmurState> = Default::default();
/// hashmap.insert("abc", 123);
/// hashmap.insert("def", 456);
/// assert_eq!(Some(&123), hashmap.get("abc"));
/// assert_eq!(Some(&456), hashmap.get("def"));
/// ```

pub struct MurmurHasher {
    state: u64
}

impl MurmurHasher {
    pub fn new() -> MurmurHasher {
        MurmurHasher { state: 0 }
    }

    pub fn with_seed(seed: u64) -> MurmurHasher {
        MurmurHasher { state: seed }
    }
}

impl Default for MurmurHasher {
    fn default() -> MurmurHasher {
        MurmurHasher::new()
    }
}

impl Hasher for MurmurHasher {
    fn finish(&self) -> u64 {
        self.state
    }

    fn write(&mut self, buf: &[u8]) {
        self.state = murmur_hash64a(buf, self.state)
    }
}


pub struct MurmurState(u64);

impl MurmurState {
    pub fn new() -> MurmurState {
        MurmurState(0)
    }
}

impl Default for MurmurState {
    fn default() -> MurmurState { MurmurState::new() }
}

impl BuildHasher for MurmurState {
    type Hasher = MurmurHasher;
    fn build_hasher(&self) -> MurmurHasher {
        MurmurHasher::with_seed(self.0)
    }
}

pub struct RandomMurmurState(u64);

impl RandomMurmurState {
    fn new() -> RandomMurmurState {
        RandomMurmurState(rand::thread_rng().gen())
    }
}

impl Default for RandomMurmurState {
    fn default() -> RandomMurmurState { RandomMurmurState::new() }
}

impl BuildHasher for RandomMurmurState {
    type Hasher = MurmurHasher;
    fn build_hasher(&self) -> MurmurHasher {
        MurmurHasher::with_seed(self.0)
    }
}

#[cfg(test)]
mod test {
    use super::{MurmurState,MurmurHasher,RandomMurmurState};
    use std::collections::HashMap;

    #[test]
    fn hashmap_str() {
        let s = MurmurState::new();
        let mut hashmap : HashMap<_, _, MurmurState> = HashMap::with_hasher(s);
        hashmap.insert("abc", 123);
        hashmap.insert("def", 456);
        assert_eq!(Some(&123), hashmap.get("abc"));
        assert_eq!(Some(&456), hashmap.get("def"));
    }

    #[test]
    fn hashmap_uint() {
        let s = MurmurState::new();
        let mut hashmap : HashMap<_, _, MurmurState> = HashMap::with_hasher(s);
        hashmap.insert(123, "abc");
        hashmap.insert(456, "def");
        assert_eq!(Some(&"abc"), hashmap.get(&123));
        assert_eq!(Some(&"def"), hashmap.get(&456));
    }

    #[test]
    fn hashmap_default() {
        use std::hash::BuildHasherDefault;

        let mut hash: HashMap<_, _, BuildHasherDefault<MurmurHasher>> = Default::default();
        hash.insert(42, "the answer");
        assert_eq!(hash.get(&42), Some(&"the answer"));

        let mut hash: HashMap<_, _, RandomMurmurState> = Default::default();
        hash.insert(42, "the answer");
        assert_eq!(hash.get(&42), Some(&"the answer"));
    }

    #[test]
    fn hashmap_build_hasher_default() {
        use std::hash::BuildHasherDefault;
        type MyHasher = BuildHasherDefault<MurmurHasher>;

        let mut map: HashMap<_, _, MyHasher> = HashMap::default();
        map.insert(42, "the answer");
        assert_eq!(map.get(&42), Some(&"the answer"));
    }
}

