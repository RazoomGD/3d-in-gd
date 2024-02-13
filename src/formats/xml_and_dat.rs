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


pub fn xml_to_dat(xml: &String) -> Result<String, &str> {
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

