use std::fs::File;
use std::io::Result as IoResult;
use std::path::Path;
pub struct SsTableReader { _file: File }
impl SsTableReader { pub fn open<P: AsRef<Path>>(path: P) -> IoResult<Self> { let file = File::open(path)?; Ok(Self { _file: file }) } }
