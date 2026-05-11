use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Read, Write, Result as IoResult};
use std::path::Path;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
pub struct Wal { file: BufWriter<File> }
impl Wal { pub fn open<P: AsRef<Path>>(path: P) -> IoResult<Self> { let file = OpenOptions::new().create(true).append(true).open(path)?; Ok(Self { file: BufWriter::new(file) }) } pub fn append(&mut self, key: &[u8], value: Option<&[u8]>) -> IoResult<()> { self.file.write_u32::<BigEndian>(key.len() as u32)?; self.file.write_u32::<BigEndian>(value.map_or(0, |v| v.len() as u32))?; self.file.write_u8(value.is_none() as u8)?; self.file.write_all(key)?; if let Some(v) = value { self.file.write_all(v)?; } self.file.flush()?; Ok(()) } pub fn recover<P: AsRef<Path>>(path: P) -> IoResult<Vec<(Vec<u8>, Option<Vec<u8>>)>> { if !path.as_ref().exists() { return Ok(Vec::new()); } let file = File::open(path)?; let mut reader = BufReader::new(file); let mut records = Vec::new(); while let Ok(k_len) = reader.read_u32::<BigEndian>() { let v_len = reader.read_u32::<BigEndian>()?; let tomb = reader.read_u8()? == 1; let mut key = vec![0u8; k_len as usize]; reader.read_exact(&mut key)?; let val = if tomb { None } else { let mut v = vec![0u8; v_len as usize]; reader.read_exact(&mut v)?; Some(v) }; records.push((key, val)); } Ok(records) } }

// Incremental development step #5

// Incremental development step #19
