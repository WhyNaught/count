use std::{env, io}; 
mod utils;

fn main() -> io::Result<()>{ 
    let args: Vec<String> = env::args().collect(); 

    if args.len() < 2 {
        eprintln!("Error: File/directory path must be provided."); 
        return Ok(()); 
    }

    let path = args[1].clone();
    let num_lines = utils::count_lines::count_lines(path.clone())?; 

    println!("{} has {} lines of code.", path, num_lines); 
    return Ok(()); 
}
