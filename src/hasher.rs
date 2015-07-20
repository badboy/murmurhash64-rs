use std::default::Default;
use std::hash::Hasher;
use std::collections::hash_state::HashState;
use rand::{self,Rng};
use murmurhash64::murmur_hash64a;

/// MurmurHash2 can also be used as the hash algorithm in a HashMap
/// (or similar). For this it implements the std::hash::Hasher trait.
///
/// # Basic Example
///
/// ```rust
/// # #![feature(hashmap_hasher)]
/// # use std::collections::HashMap;
/// # use std::default::Default;
/// # use murmurhash64::{MurmurHasher,RandomMurmurState};
/// # use std::collections::hash_state::DefaultState;
/// let mut hashmap : HashMap<_, _, DefaultState<MurmurHasher>> = Default::default();
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

impl HashState for MurmurState {
    type Hasher = MurmurHasher;
    fn hasher(&self) -> MurmurHasher {
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

impl HashState for RandomMurmurState {
    type Hasher = MurmurHasher;
    fn hasher(&self) -> MurmurHasher {
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

    #[test]
    fn hashmap_default() {
        use std::collections::hash_state::DefaultState;

        let mut hash: HashMap<_, _, DefaultState<MurmurHasher>> = Default::default();
        hash.insert(42, "the answer");
        assert_eq!(hash.get(&42), Some(&"the answer"));

        let mut hash: HashMap<_, _, RandomMurmurState> = Default::default();
        hash.insert(42, "the answer");
        assert_eq!(hash.get(&42), Some(&"the answer"));
    }
}

