use std::fs;

use xml::{reader::XmlEvent, EventReader};
use xml_doc::{Document, Element};

use super::xml_and_dat::{dat_to_xml, xml_to_dat};

// tskes string-level, inserts it into gd
pub fn string_to_gd(level_name: &String, level_string: &String, cc_local_levels_path: &String) -> Result<(), &'static str> {
    let cc_local_levels_content = match fs::read_to_string(cc_local_levels_path) {
        Ok(s) => s,
        Err(_) => {return Err("can't read \"CCLocalLevels.dat\" file");},
    };
    let xml = dat_to_xml(&cc_local_levels_content)?;
    let updated_xml = insert_level(&xml, level_name, level_string)?;
    let updated_cc_local_levels = xml_to_dat(&updated_xml)?;
    match fs::write(cc_local_levels_path, updated_cc_local_levels) {
        Ok(_) => Ok(()),
        Err(_) => Err("can't write \"CCLocalLevels.dat\" file")
    }
}


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


fn insert_level(xml: &String, level_name: &String, level_string: &String, ) -> Result<String, &'static str> {

    // cc local levels
    let mut doc = match Document::parse_str(xml) {
        Ok(s) => s,
        Err(_) => return Err("can't read xml")
    };
    
    let d1 = match (|| doc.root_element()?.find(&doc, "dict")?.find(&doc, "d"))() { 
        Some(s) => s,
        None => return Err("can't parse xml"),
    };

    let elements: Vec<Element> = d1.child_elements(&doc);
    
    // increment all keys
    for el in elements {
        if el.name(&doc) == "k" && 
            el.text_content(&doc).len() >= 3 && 
            el.text_content(&doc)[0..2] == "k_".to_string() 
        {
            let next_num = match el.text_content(&doc)[2..].parse::<i32>() {
                Ok(num) => num + 1,
                _ => continue, //shouldn't happen
            };
            el.set_text_content(&mut doc, format!("k_{}", next_num));
        }
    }

    let _future_level = Element::build(&mut doc, "future_level")
        .push_to(d1); //todo: rewrite it later

    let new_xml = doc.write_str().unwrap();

    Ok(new_xml.replace("<future_level/>", &default_level_settings(level_name, level_string)))
}


//from 2.1 (STARTS FROM <k>k_0</k> line!!!)
fn default_level_settings(level_name: &String, level_string: &String) -> String {
    format!("<k>k_0</k><d><k>kCEK</k><i>4</i><k>k2</k><s>{}</s><k>k3</k><s>dGVzdCBkZXNjcmlwdGlvbg==</s><k>k4</k><s>{}</s><k>k5</k><s>Player</s><k>k13</k><t/><k>k21</k><i>2</i><k>k16</k><i>1</i><k>k80</k><i>23</i><k>k50</k><i>35</i><k>k47</k><t/><k>k48</k><i>3</i><k>kI1</k><r>-3.05176e-005</r><k>kI2</k><r>84.6</r><k>kI3</k><r>0.6</r><k>kI5</k><i>4</i><k>kI6</k>
<d><k>0</k><s>0</s><k>1</k><s>0</s><k>2</k><s>0</s><k>3</k><s>0</s><k>4</k><s>0</s><k>5</k><s>0</s><k>6</k><s>0</s><k>7</k><s>0</s><k>8</k><s>0</s><k>9</k><s>0</s><k>10</k><s>0</s><k>11</k><s>0</s><k>12</k><s>0</s></d></d>", 
    level_name, level_string).replace("\n", "")
}


//----------tests-----------
#[cfg(test)]
mod tests {
    use std::fs;

    use super::{get_level_by_name, insert_level};

    #[test]
    fn test_get_from_xml() {
        let xml = fs::read_to_string("test2.xml").unwrap();
        println!("{}", get_level_by_name(&xml, &"test level".to_string()).unwrap());
    }

    #[test]
    fn test_put_to_xml() {
        let src = fs::read_to_string("test2.xml").unwrap();
        let new_xml =  insert_level(&src, &String::from("hallo here"), &String::from("inserted level")).unwrap();
        fs::write("test4_changed_xml.xml", new_xml).unwrap();
    }
}

