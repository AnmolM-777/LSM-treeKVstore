use std::sync::atomic::{AtomicU64, Ordering};
pub struct TxnManager { next_seq: AtomicU64 }
impl TxnManager { pub fn new() -> Self { Self { next_seq: AtomicU64::new(1) } } pub fn get_next_seq(&self) -> u64 { self.next_seq.fetch_add(1, Ordering::SeqCst) } }

// Incremental development step #10

// Incremental development step #24

// Incremental development step #38

// Incremental development step #52

// Incremental development step #66

// Incremental development step #80
