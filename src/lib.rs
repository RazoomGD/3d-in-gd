// facade

mod formats;
mod gd;
mod utils;

use std::{fs, io::{self, stdout, Write}, path::Path, result};

use utils::file::request_file_path;

use crate::{formats::xml_and_dat::{dat_to_xml, xml_to_dat}, utils::file::get_file_content};


pub fn run() {
    
    let s = get_file_content(request_file_path(
        "enter file name: "
    )).unwrap();
    
    let tmp = dat_to_xml(&s).unwrap();
    fs::write("test3.xml", tmp);
    
}



