use std::fs::File;
use std::error::Error;
use std::io::prelude::*;


pub fn search(search_pattern : &str, file_name : &str) -> Result<Vec<String>,Box<dyn Error>> {
    let mut file = check_file_exists(file_name)?;
    let mut content = String::new();
    let mut found = Vec::new();
    file.read_to_string(&mut content)?;
    for line in content.lines(){
        if line.contains(search_pattern){
        found.push(line.to_string());
        }
    }


    Ok(found)
}

fn check_file_exists(file_name : &str) -> Result<File,Box<dyn Error>> {
    
    let file = File::open(file_name)?;

    Ok(file)
}
