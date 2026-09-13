use std::any::TypeId;
use std::collections::HashMap;
use std::hash::{BuildHasherDefault, Hasher};

#[derive(Default)]
pub struct IdentityHasher(u64);

impl Hasher for IdentityHasher {
    fn finish(&self) -> u64 {
        self.0
    }
    fn write_u64(&mut self, i: u64) {
        self.0 = i;
    }
    fn write(&mut self, bytes: &[u8]) {
        self.0 = u64::from_ne_bytes(bytes[..8].try_into().unwrap_or([0; 8]));
    }
}

pub type TypeIdMap<V> = HashMap<TypeId, V, BuildHasherDefault<IdentityHasher>>;
