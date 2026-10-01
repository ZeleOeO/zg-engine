use std::any::Any;
use std::any::TypeId;
use std::collections::HashMap;
use std::hash::{BuildHasherDefault, Hash, Hasher};

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

#[allow(private_interfaces)]
pub type TypeIdMap<V> = HashMap<TypeId, V, BuildHasherDefault<IdentityHasher>>;

pub fn load_binary(location: &str) -> anyhow::Result<Vec<u8>> {
    let data = std::fs::read(location)?;
    Ok(data)
}

pub trait DynHash: Any {
    fn as_any(&self) -> &dyn Any;
    fn dyn_hash(&self, state: &mut dyn Hasher);
    fn dyn_eq(&self, other: &dyn DynHash) -> bool;
}

impl<T> DynHash for T
where
    T: Hash + Eq + Any,
{
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn dyn_hash(&self, mut state: &mut dyn Hasher) {
        self.hash(&mut state);
    }
    fn dyn_eq(&self, other: &dyn DynHash) -> bool {
        other
            .as_any()
            .downcast_ref::<T>()
            .map_or(false, |o| self == o)
    }
}

impl Hash for dyn DynHash {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.dyn_hash(state);
    }
}

impl PartialEq for dyn DynHash {
    fn eq(&self, other: &Self) -> bool {
        self.dyn_eq(other)
    }
}

impl Eq for dyn DynHash {}
