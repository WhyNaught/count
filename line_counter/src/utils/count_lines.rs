use std::{fs, io};
use std::fs::{Metadata, metadata};

use crate::utils::count_file::count_lines_in_file; 

pub fn count_lines(path: String, predicates: &Vec<Box<dyn Fn(String, Metadata) -> bool>>, root: &str) -> io::Result<i64> {
    let md = metadata(path.clone()).unwrap(); 
    let relative_path = path.strip_prefix(root).unwrap_or(&path).to_string();

    for predicate in predicates {
        if !predicate(relative_path.clone(), md.clone()) {
            return Ok(0); 
        } 
    }

    if md.is_file() {
        let result = count_lines_in_file(path.clone())?; 
        return Ok(result); 
    }

    let mut num_lines: i64 = 0; 
    let children = fs::read_dir(path.clone()).unwrap(); 

    for child in children {
        let child_path = child.unwrap().path().into_os_string().into_string().unwrap();

        num_lines += count_lines(child_path, predicates, root)?;  
    }

    return Ok(num_lines); 
}