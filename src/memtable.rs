use crossbeam_skiplist::SkipMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
pub type Key = Vec<u8>;
pub type Value = Option<Vec<u8>>;
#[derive(Clone)]
pub struct MemTable { map: Arc<SkipMap<Key, Value>>, size_bytes: Arc<AtomicUsize> }
impl MemTable { pub fn new() -> Self { Self { map: Arc::new(SkipMap::new()), size_bytes: Arc::new(AtomicUsize::new(0)) } } pub fn put(&self, key: Key, value: Value) { let k_len = key.len(); let v_len = value.as_ref().map_or(0, |v| v.len()); self.map.insert(key, value); self.size_bytes.fetch_add(k_len + v_len, Ordering::Relaxed); } pub fn get(&self, key: &[u8]) -> Option<Value> { self.map.get(key).map(|e| e.value().clone()) } pub fn size(&self) -> usize { self.size_bytes.load(Ordering::Relaxed) } }

// Incremental development step #4

// Incremental development step #18

// Incremental development step #32

// Incremental development step #46

// Incremental development step #60

// Incremental development step #74
