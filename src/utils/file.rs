use std::fs;




//todo delete it
pub fn get_file_content(file_path: String) -> Result<String, &'static str> {
    let content = fs::read_to_string(file_path);
    match content {
        Ok(s) => Ok(s),
        Err(_) => Err("can't read this file"),
    }
}