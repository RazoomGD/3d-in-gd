use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use std::io::prelude::*;
use flate2::{read::{GzDecoder, GzEncoder}, Compression};


pub fn dat_to_xml(dat: &String) -> Result<String, &'static str> {
    // decode robtop's .dat format
    let base64_str: String = dat.trim().chars()
        .map(|c| (c as u8 ^ 0xB) as char)
        .collect();

    // decode base 64
    let gzip = match URL_SAFE.decode(base64_str) {
        Ok(res) => res,
        Err(_) => return Err("error while decoding base64"),
    };

    // decompress gzip
    let mut decoder = GzDecoder::new(&gzip[..]);
    let mut xml = String::new();
    if let Err(_) = decoder.read_to_string(&mut xml) {
        return Err("error while decompressing gzip");
    }

    Ok(xml)
}


pub fn xml_to_dat(xml: &String) -> Result<String, &'static str> {
    // to gzip
    let mut gzip = Vec::new();
    let mut encoder = GzEncoder::new(xml.as_bytes(), Compression::fast());
    if let Err(_) = encoder.read_to_end(&mut gzip) {
        return Err("error while compressing to gzip");
    }

    // to base64
    let base64 = URL_SAFE.encode(gzip);

    // to robtop's dat format
    let dat: String = base64.chars()
        .map(|c| (c as u8 ^ 0xB) as char)
        .collect();
    
    Ok(dat)
}


//------------tests----------
#[cfg(test)]
pub mod tests {
    use std::fs;

    use super::{dat_to_xml, xml_to_dat};

    #[test]
    fn dat_to_xml_test() {
        let cc = fs::read_to_string("CCTest_3.dat").unwrap();
        let xml = dat_to_xml(&cc).unwrap();
        fs::write("test3.xml", xml).unwrap();
    }

    #[test]
    fn xml_to_dat_test() {
        let xml = fs::read_to_string("test3.xml").unwrap();
        let dat = xml_to_dat(&xml).unwrap();
        fs::write("test.dat", dat).unwrap();
    }
}

