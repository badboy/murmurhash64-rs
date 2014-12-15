use std::hash::Hash;
use std::hash::Hasher;
use std::hash::Writer;

use murmurhash64::murmur_hash64a;

pub struct MurmurState {
    bytes: Vec<u8>
}

impl MurmurState {
    #[inline]
    fn new() -> MurmurState {
        MurmurState { bytes: Vec::new() }
    }
}

impl Writer for MurmurState {
    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        self.bytes.push_all(bytes);
    }
}

#[deriving(Copy)]
pub struct MurmurHasher {
    seed: u64
}

impl MurmurHasher {
    #[inline]
    pub fn new() -> MurmurHasher {
        MurmurHasher { seed: 0 }
    }

    #[inline]
    pub fn with_seed(seed: u64) -> MurmurHasher {
        MurmurHasher { seed: seed }
    }
}

impl Hasher<MurmurState> for MurmurHasher {
    #[inline]
    fn hash<Sized? T: Hash<MurmurState>>(&self, value: &T) -> u64 {
        let mut state = MurmurState::new();
        value.hash(&mut state);
        murmur_hash64a(state.bytes.as_slice(), self.seed)
    }
}

#[cfg(test)]
mod test {
    use super::MurmurHasher;

    #[test]
    fn hashmap_str() {
        use std::collections::HashMap;
        let mut hashmap: HashMap<&str, uint, MurmurHasher> = HashMap::with_hasher(MurmurHasher::new());
        hashmap.insert("abc", 123);
        hashmap.insert("def", 456);
        assert_eq!(Some(&123), hashmap.get("abc"));
        assert_eq!(Some(&456), hashmap.get("def"));
    }

    #[test]
    fn hashmap_uint() {
        use std::collections::HashMap;
        let mut hashmap: HashMap<uint, &str, MurmurHasher> = HashMap::with_hasher(MurmurHasher::new());
        hashmap.insert(123, "abc");
        hashmap.insert(456, "def");
        assert_eq!(Some(&"abc"), hashmap.get(&123));
        assert_eq!(Some(&"def"), hashmap.get(&456));
    }
}
