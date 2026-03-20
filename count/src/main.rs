use std::{env, fs::{self, Metadata, metadata}, io}; 
mod utils;
use crate::utils::{count_lines::count_lines, flag_utils::{get_flag_argument, get_is_valid_ignore_function, get_is_valid_suffix_function}}; 

fn main() -> io::Result<()>{ 
    let args: Vec<String> = env::args().collect(); 

    if args.len() < 2 {
        eprintln!("Error: File/directory path must be provided."); 
        println!("Use --help flag for instructions on usage."); 
        return Ok(()); 
    }

    if args.iter().any(|arg| arg == "--help") {
        println!("\"count\" command usage: count <directory/file path> [--flag1] [--flag2]...");
        println!("--help: instructions on how to use this tool"); 
        println!("--ignore <file_path>: will ignore any files that match the patterns written within this file (common example would be count --ignore .gitignore)"); 
        println!("--suffix <suffix>: will only count files that end with the suffix (.rs, .c, .txt, etc.)");
        return Ok(()); 
    }

    let path = args[1].clone();
    let md = metadata(path.clone()); 
    if md.is_err() {
        eprintln!("Error: Invalid file/directory path provided: {}", path); 
        return Ok(()); 
    }

    let root = fs::canonicalize(".")?.into_os_string().into_string().unwrap();

    // each function here should return true if a given path is valid 
    // should return false if they are invalid
    let mut predicates: Vec<Box<dyn Fn(String, Metadata) -> bool>> = Vec::new();
    predicates.push(get_is_valid_ignore_function(get_flag_argument(&args, "--ignore")?)?);
    predicates.push(get_is_valid_suffix_function(get_flag_argument(&args, "--suffix")?)?); 

    let num_lines = count_lines(path.clone(), &predicates, &root)?; 

    println!("{} has {} lines of code.", path, num_lines); 
    return Ok(()); 
}
