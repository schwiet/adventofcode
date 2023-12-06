use std::fs::File;
use std::io::{self, BufReader};
use std::path::Path;

pub fn open_file_as_bufreader(file_path: &str) -> io::Result<BufReader<File>> {
    let path = Path::new(file_path);
    let file = File::open(&path)?;
    Ok(BufReader::new(file))
}
