use std::{fs, io};
use std::fs::metadata;

use crate::utils::count_file::count_lines_in_file; 

pub fn count_lines(path: String) -> io::Result<i64> {
    let md = metadata(path.clone()).unwrap(); 

    if md.is_file() {
        let result = count_lines_in_file(path.clone())?; 
        return Ok(result); 
    }

    let mut num_lines: i64 = 0; 
    let children = fs::read_dir(path.clone()).unwrap(); 

    for child in children {
        let child_path = child.unwrap().path().into_os_string().into_string().unwrap();

        num_lines += count_lines(child_path)?;  
    }

    return Ok(num_lines); 
}