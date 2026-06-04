pub struct BloomFilter { bit_vec: Vec<bool>, num_hashes: usize }
impl BloomFilter { pub fn new(capacity: usize, fp_rate: f64) -> Self { let size = (-(capacity as f64) * fp_rate.ln() / (2.0f64.ln().powi(2))).ceil() as usize; let num_hashes = ((size as f64 / capacity as f64) * 2.0f64.ln()).round() as usize; Self { bit_vec: vec![false; size.max(64)], num_hashes: num_hashes.max(1) } } pub fn insert(&mut self, key: &[u8]) { for i in 0..self.num_hashes { let hash = crc32fast::hash(&[key, &[i as u8]].concat()) as usize; self.bit_vec[hash % self.bit_vec.len()] = true; } } pub fn contains(&self, key: &[u8]) -> bool { for i in 0..self.num_hashes { let hash = crc32fast::hash(&[key, &[i as u8]].concat()) as usize; if !self.bit_vec[hash % self.bit_vec.len()] { return false; } } true } }

// Incremental development step #7

// Incremental development step #21

// Incremental development step #35

// Incremental development step #49

// Incremental development step #63
