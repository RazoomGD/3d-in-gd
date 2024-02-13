use std::fs;

use xml::{reader::XmlEvent, EventReader};

use super::xml_and_dat::dat_to_xml;

// pub fn string_to_gd(config: &mut GdConfig) -> Result<(), &str> {
//     Ok(())
// }


// returns string format
pub fn gd_to_string(level_name: &String, cc_local_levels_path: &String) -> Result<String, &'static str> {
    let cc_local_levels_content = match fs::read_to_string(cc_local_levels_path) {
        Ok(s) => s,
        Err(_) => {return Err("can't read \"CCLocalLevels.dat\" file");},
    };
    let xml = dat_to_xml(&cc_local_levels_content)?;
    get_level_by_name(&xml, &level_name)
}


// returns string format
fn get_level_by_name(xml: &String, level_name: &String) -> Result<String, &'static str> {
    let parser = EventReader::new(xml.as_bytes());

    let mut tree: Vec<String> = Vec::new();
    let mut last_key = String::new();
    let mut flag = false;

    for el in parser {
        match el {
            Ok(XmlEvent::StartElement {name, ..}) => {
                tree.push(name.to_string());
            },
            Ok(XmlEvent::EndElement {name: _ }) => {
                tree.pop();
            },
            Ok(XmlEvent::Characters(s)) => {
                if let [_, k2, k3, k4, k5] = tree.as_slice() {
                    if k2 == "dict" && k3 == "d" && k4 == "d" { 

                        if k5 == "k" {
                            last_key = s.trim().to_string();

                        } else if k5 == "i" || k5 == "s" {

                            if last_key == "k2" && s.trim() == level_name.trim() { // k2 - level name
                                flag = true;
                            } else if last_key == "k4" && flag { // k4 - level string
                                return Ok(s.trim().to_string());
                            }
                        } 
                        
                        if k5 != "k" {
                            last_key.clear();
                        }

                    } else {
                        flag = false;
                    }
                } 
            }
            Ok(_) | Err(_) => {},
        }
    }
    Err("level wasn't found!")
}




//----------tests-----------
#[cfg(test)]
mod tests {
    use std::fs;

    use super::get_level_by_name;

    #[test]
    fn test_get_from_xml() {
        let xml = fs::read_to_string("test2.xml").unwrap();
        println!("{}", get_level_by_name(&xml, &"test level".to_string()).unwrap());
    }
}
