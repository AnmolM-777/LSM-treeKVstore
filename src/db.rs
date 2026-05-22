use crate::memtable::MemTable;
use crate::wal::Wal;
use crate::mvcc::TxnManager;
use std::sync::Arc;
use parking_lot::RwLock;
pub struct LSMEngine { memtable: Arc<RwLock<MemTable>>, _wal: Arc<RwLock<Wal>>, _txn_mgr: TxnManager }
impl LSMEngine { pub fn open(path: &str) -> std::io::Result<Self> { let wal = Wal::open(format!("{}/wal.log", path))?; Ok(Self { memtable: Arc::new(RwLock::new(MemTable::new())), _wal: Arc::new(RwLock::new(wal)), _txn_mgr: TxnManager::new() }) } pub fn put(&self, key: &[u8], value: &[u8]) -> std::io::Result<()> { self.memtable.write().put(key.to_vec(), Some(value.to_vec())); Ok(()) } pub fn get(&self, key: &[u8]) -> Option<Vec<u8>> { self.memtable.read().get(key).flatten() } }

// Incremental development step #11

// Incremental development step #25

// Incremental development step #39
