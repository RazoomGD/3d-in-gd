mod formats;

use std::{fs::{self, File}, io::Write, path::Path};

use text_io::read;

use crate::formats::{gd_and_string::{gd_to_string, string_to_gd}, level_and_string::{level_to_string, string_to_level}};

// formats info:
// - Level - readable format with preamble and object info
// - String - compressed and decoded "Level" format. Unreadable
// - GD - when level is in "CCLocalLevels.dat"

#[derive(Debug)]
pub enum InputFormats {
    GD, LEVEL,
}

#[derive(Debug)]
pub enum OutputFormats {
    GD, LEVEL,
}

#[derive(Debug)]
pub struct Config {
    pub src_path: String,
    pub input_format: InputFormats,
    pub level_name: String,
    pub target_path: String,
    pub output_format: OutputFormats,
    pub cc_local_levels_path: String,
}


pub fn run() -> Result<(), &'static str> {
    // set configuration
    println!("Enter path to source file (leave empty to export directly from gd files):");
    let src_path = loop {
        let mut path: String = read!("{}\n");
        path = path.trim().to_string();
        if path == "" || Path::new(&path).exists() {
            break path
        }
        println!("File \"{}\" not exists! Try again: ", path); 
    };
    println!("--------------------");
    println!("Enter path to target file (leave empty to insert directly into gd files):");
    let target_path = loop {
        let mut path: String = read!("{}\n");
        path = path.trim().to_string();
        if path == "" || !Path::new(&path).exists() {
            break path
        }
        println!("File \"{}\" already exists! Try again: ", path); 
    };
    println!("--------------------");
    let cc_local_levels_path = if src_path == "" || target_path == "" {
        println!("To insert/export directly into/from gd enter path to \"CCLocalLevels.dat\" file:");
        loop {
            let mut path: String = read!("{}\n");
            path = path.trim().to_string();
            if Path::new(&path).exists() {
                break path
            }
            println!("File \"{}\" not exists! Try again: ", path); 
        }
    } else {
        "".to_string()
    };
    let level_name = if src_path == "" || target_path == "" {
        println!("Enter non-empty level name, that is specified (or will be secified) in gd:");
        let level_name: String = read!("{}\n");
        println!("--------------------");
        level_name
    } else {
        "".to_string()
    };

    let input_format = if src_path == "" || src_path.contains("CCLocalLevels") {
        InputFormats::GD
    } else {
        InputFormats::LEVEL
        // let res = loop {
        //     println!("Select input format:\n1-level, 2-gmd (enter 1 or 2):");
        //     let answ: String = read!("{}\n");
        //     if answ.trim() == "1" {
        //         break InputFormats::LEVEL
        //     }
        //     if answ.trim() == "2" {
        //         break InputFormats::GMD
        //     }
        //     println!("Not valid number!");
        // };
        // println!("--------------------");
        // res
    };

    let output_format = if target_path == "" || target_path.contains("CCLocalLevels") {
        OutputFormats::GD
    } else {
        OutputFormats::LEVEL
        // let res = loop {
        //     println!("Select output format:\n1-level, 2-gmd (enter 1 or 2):");
        //     let answ: String = read!("{}\n");
        //     if answ.trim() == "1" {
        //         break OutputFormats::LEVEL
        //     }
        //     if answ.trim() == "2" {
        //         break OutputFormats::GMD
        //     }
        //     println!("Not valid number!");
        // };
        // println!("--------------------");
        // res
    };

    let config = Config{
        output_format,
        src_path,
        level_name,
        input_format,
        target_path,
        cc_local_levels_path,
    };


    // convert to level
    let level = match config.input_format {
        InputFormats::GD => {
            let string = gd_to_string(&config.level_name, &config.cc_local_levels_path)?;
            string_to_level(&string, false)?
        }

        InputFormats::LEVEL => {
            match fs::read_to_string(config.src_path) {
                Ok(s) => s,
                Err(_) => {return Err("error while reading source file");}
            }.replace("\n", "").replace(" ", "")
        }
    };

    
    // --------------- so here it is in simple level format ---------------
     

    // convert to output format
    match config.output_format {
        OutputFormats::GD => {
            let level_string = level_to_string(&level, true)?; //anyway it checks preamble
            string_to_gd(&config.level_name, &level_string, &config.cc_local_levels_path)?;
        }

        OutputFormats::LEVEL => {
            let mut file = match File::create(&config.target_path) {
                Ok(f) => f,
                Err(_) => {return Err("error while creating target file")}
            };
            match file.write(level.replace(";", ";\n").as_bytes()) {
                Ok(_) => {},
                Err(_) => {return Err("error while writing to target file")}
            }
        }
    }

    Ok(())
}

