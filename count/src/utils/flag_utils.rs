use std::{fs::{Metadata, read_to_string}, io::{self}};
use glob::Pattern; 

pub fn get_flag_argument(args: &Vec<String>, flag_name: &str) -> io::Result<String> {
    let found_position = args.iter().position(|arg| arg == flag_name);

    if found_position.is_some() {
        let result = args.get(found_position.unwrap() + 1)
            .ok_or_else(|| io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("flag '{}' requires an argument", flag_name)
            ))?; 

        if result.starts_with("--") {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("flag '{}' requires an argument", flag_name)
            )); 
        }

        return Ok(result.clone()); 
    } else {
        return Ok(String::new()); 
    }
}

pub fn get_is_valid_suffix_function(suffix: String) -> io::Result<Box<dyn Fn(String, Metadata) -> bool>> {
    let result = Box::new(move |path: String, md: Metadata| -> bool {
        if suffix.is_empty() || md.is_dir() {
            return true; 
        }
        
        return path.ends_with(&suffix); 
    }); 

    return Ok(result); 
}

pub fn get_is_valid_ignore_function(ignore_path: String) -> io::Result<Box<dyn Fn(String, Metadata) -> bool>> {
    let patterns_to_ignore: Vec<glob::Pattern> = if ignore_path.is_empty() {
        Vec::new()
    } else {
        read_to_string(&ignore_path)
            .map_err(|_| io::Error::new(io::ErrorKind::NotFound, format!("Invalid file path for --ignore: {}", ignore_path)))?
            .lines()
            .filter(|x| !x.is_empty() && !x.starts_with('#'))
            .filter_map(|x| glob::Pattern::new(x).ok())
            .collect()
    };

    let result = Box::new(move |path: String, _md: Metadata| -> bool {
        !patterns_to_ignore.iter().any(|p: &Pattern| p.matches(&path))
    });

    Ok(result)
}