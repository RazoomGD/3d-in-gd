use core::panic;
use std::{fs, io, path::Path};

use crate::utils::file;


pub fn request_file_path(request: &str) -> String {
    println!("{request}");
    let mut file_name = String::new();
    loop {              
        if let Err(_) = io::stdin().read_line(&mut file_name) {
            panic!(); //we should be able to read stdin
        }  
        file_name = file_name.trim().to_string();

        if Path::new(&file_name).exists() {
            break;
        } else {
            println!("File \"{}\" not exists! Try again: ", file_name);
            file_name.clear(); 
        }
    }
    file_name
}


pub fn get_file_content(file_path: String) -> Result<String, &'static str> {
    let content = fs::read_to_string(file_path);
    match content {
        Ok(s) => Ok(s),
        Err(_) => Err("can't read this file"),
    }
}