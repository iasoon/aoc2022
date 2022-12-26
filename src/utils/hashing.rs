use std::{
    collections::{HashMap, HashSet},
    hash::{BuildHasher, Hasher},
};

const FNV_OFFSET_BASIS: u64 = 0xcbf29ce484222325;
const FNV_PRIME: u64 = 0x100000001b3;

pub type FnvHashMap<K, V> = HashMap<K, V, FnvHash>;
pub type FnvHashSet<K> = HashSet<K, FnvHash>;

pub struct FnvHash;

impl BuildHasher for FnvHash {
    type Hasher = FnvHasher;

    fn build_hasher(&self) -> Self::Hasher {
        FnvHasher::init()
    }
}

/// Implements FNV-1a hashing
pub struct FnvHasher {
    state: u64,
}

impl FnvHasher {
    pub fn init() -> Self {
        FnvHasher {
            state: FNV_OFFSET_BASIS,
        }
    }
}

impl Hasher for FnvHasher {
    fn finish(&self) -> u64 {
        self.state
    }

    fn write(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.state ^= byte as u64;
            self.state = self.state.wrapping_mul(FNV_PRIME);
        }
    }
}
