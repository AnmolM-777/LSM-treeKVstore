use std::sync::Arc;
use std::thread;
pub struct Compactor { _max_levels: usize }
impl Compactor { pub fn new(max_levels: usize) -> Self { Self { _max_levels: max_levels } } pub fn start(self: Arc<Self>) { thread::spawn(move || { loop { thread::sleep(std::time::Duration::from_secs(10)); } }); } }

// Incremental development step #9

// Incremental development step #23

// Incremental development step #37

// Incremental development step #51
