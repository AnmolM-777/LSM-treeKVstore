use super::bloom::BloomFilter;
use std::fs::File;
use std::io::{BufWriter, Write, Result as IoResult};
use byteorder::{BigEndian, WriteBytesExt};
pub struct SsTableBuilder { writer: BufWriter<File>, bloom: BloomFilter, index: Vec<(Vec<u8>, u64)>, current_offset: u64 }
impl SsTableBuilder { pub fn new(file: File, expected_keys: usize) -> Self { Self { writer: BufWriter::new(file), bloom: BloomFilter::new(expected_keys, 0.01), index: Vec::new(), current_offset: 0 } } pub fn add(&mut self, key: &[u8], value: Option<&[u8]>) -> IoResult<()> { self.bloom.insert(key); self.index.push((key.to_vec(), self.current_offset)); self.writer.write_u32::<BigEndian>(key.len() as u32)?; self.writer.write_u32::<BigEndian>(value.map_or(0, |v| v.len() as u32))?; self.writer.write_all(key)?; if let Some(v) = value { self.writer.write_all(v)?; } self.current_offset += 8 + key.len() as u64 + value.map_or(0, |v| v.len() as u64); Ok(()) } pub fn finish(mut self) -> IoResult<()> { self.writer.flush()?; Ok(()) } }

// Incremental development step #8

// Incremental development step #22

// Incremental development step #36
