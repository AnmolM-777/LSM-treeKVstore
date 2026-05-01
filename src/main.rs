use lsm_tree_kvstore::db::LSMEngine;
use std::fs;
use std::path::Path;
fn main() -> std::io::Result<()> { println!("=== LSM Engine Demo ==="); let path = "./db_demo_data"; if Path::new(path).exists() { fs::remove_dir_all(path)?; } fs::create_dir_all(path)?; let db = LSMEngine::open(path)?; db.put(b"user:1001", b"Alice")?; println!("PUT completed."); if let Some(v) = db.get(b"user:1001") { println!("GET: {}", String::from_utf8_lossy(&v)); } Ok(()) }
