use std::default::Default;
use std::hash::Hasher;
use std::collections::hash_state::HashState;
use murmurhash64::murmur_hash64a;

/// MurmurHash2 can also be used as the hash algorithm in a HashMap
/// (or similar). For this it implements the std::hash::Hasher trait.
///
/// # Basic Example
///
/// ```rust
/// # #![feature(std_misc)]
/// # use std::collections::HashMap;
/// # use murmurhash64::{MurmurHasher,MurmurState};
/// let s = MurmurState::new();
/// let mut hashmap : HashMap<&'static str, usize, MurmurState> = HashMap::with_hash_state(s);
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


pub struct MurmurState {
    seed: u64
}

impl MurmurState {
    pub fn new() -> MurmurState {
        MurmurState { seed: 0 }
    }
}

impl HashState for MurmurState {
    type Hasher = MurmurHasher;
    fn hasher(&self) -> MurmurHasher {
        MurmurHasher::with_seed(self.seed)
    }
}

#[cfg(test)]
mod test {
    use super::MurmurState;
    use std::collections::HashMap;

    #[test]
    fn hashmap_str() {
        let s = MurmurState::new();
        let mut hashmap : HashMap<&'static str, isize, MurmurState> = HashMap::with_hash_state(s);
        hashmap.insert("abc", 123);
        hashmap.insert("def", 456);
        assert_eq!(Some(&123), hashmap.get("abc"));
        assert_eq!(Some(&456), hashmap.get("def"));
    }

    #[test]
    fn hashmap_uint() {
        let s = MurmurState::new();
        let mut hashmap : HashMap<usize, &'static str, MurmurState> = HashMap::with_hash_state(s);
        hashmap.insert(123, "abc");
        hashmap.insert(456, "def");
        assert_eq!(Some(&"abc"), hashmap.get(&123));
        assert_eq!(Some(&"def"), hashmap.get(&456));
    }
}
