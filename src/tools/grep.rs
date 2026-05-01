use std::fs;

pub fn search(needle:&str,haystack:&str) {
    match fs::read_to_string(haystack){
        Ok(contents) => {
            for line in contents.lines(){
                if line.to_lowercase().contains(needle){
                    println!("{}",line);
                }
            }
        },
        Err(e)=> {
            eprintln!("{}",e);
        }
    }
    }
