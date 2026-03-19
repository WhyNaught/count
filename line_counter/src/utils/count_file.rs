use std::{fs::File, io};
use std::io::BufRead; 

pub fn count_lines_in_file(path: String) -> io::Result<i64> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let count = reader.lines().count();  

    return Ok(count as i64);
}