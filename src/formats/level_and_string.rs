use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use std::io::prelude::*;
use flate2::{read::{GzDecoder, GzEncoder}, Compression};


pub fn level_to_string(level: &String, add_preamble: bool) -> Result<String, &str> {
    let level = match add_preamble {
        true => {
            let mut lvl = default_preamble().to_owned();
            lvl.push_str(level);
            lvl
        },
        false => level.to_string(),
    };
    //compress to gzip
    let mut gzip = Vec::new();
    let mut encoder = GzEncoder::new(level.as_bytes(), Compression::fast());
    if let Err(_) = encoder.read_to_end(&mut gzip) {
        return Err("error while compressing to gzip");
    }
    // apply base64
    let base64 = URL_SAFE.encode(gzip);
    Ok(base64)
}


pub fn string_to_level(string: &String, remove_preamble: bool) -> Result<String, &'static str> {
    //decode base64
    let gzip = match URL_SAFE.decode(string) {
        Ok(res) => res,
        Err(_) => return Err("error while decoding base64"),
    };

    // decompress gzip
    let mut decoder = GzDecoder::new(&gzip[..]);
    let mut level = String::new();
    if let Err(_) = decoder.read_to_string(&mut level) {
        return Err("error while decompressing gzip");
    }

    if remove_preamble && level.contains("|") { //silly check if level has preamble
        match level.split_once(";") {
            Some((_preamble, lvl)) => Ok(lvl.to_string()),
            None => Ok(level),
        }
    } else {
        Ok(level)
    }
}


fn default_preamble() -> &'static str {
    "kS38,1_40_2_125_3_255_11_255_12_255_13_255_4_-1_6_1000_7_1_15_1_18_0_8_1|1_0_2_102_3_255_11_255_12_255_13_255_4_-1_6_1001_7_1_15_1_18_0_8_1|1_0_2_102_3_255_11_255_12_255_13_255_4_-1_6_1009_7_1_15_1_18_0_8_1|1_255_2_255_3_255_11_255_12_255_13_255_4_-1_6_1002_5_1_7_1_15_1_18_0_8_1|1_125_2_0_3_255_11_255_12_255_13_255_4_-1_6_1005_5_1_7_1_15_1_18_0_8_1|1_0_2_255_3_255_11_255_12_255_13_255_4_-1_6_1006_5_1_7_1_15_1_18_0_8_1|,kA13,0,kA15,0,kA16,0,kA14,,kA6,0,kA7,0,kA17,0,kA18,0,kS39,0,kA2,0,kA3,0,kA8,0,kA4,0,kA9,0,kA10,0,kA11,0;"
}




// --------- tests -----------
#[cfg(test)]
mod tests {
    use crate::formats::level_and_string::{level_to_string, string_to_level};
    #[test]
    fn str_to_lvl_test() {
        let str = "H4sIAAAAAAAAC6WQ0Q3DIAxEF3Iln8GEqF-ZIQPcAFmhwxdw-lMlElV_7rgDP1kce6oCZqUR5kw0dwJhFhZl5gMshKpyIQjvUqmsxAscCLU5BP5HrJeI_iYGpiDGPn8F6r9hLU1h_Bajv2xTbjBybEii3TyshGVpGuclmtNqtz2tI9nQAIyLLQ-NW2gYRJ-QKiZYXZLAz2gWUQzSlsyf2r_qNzPPAaFMAgAA";
        println!("{}", string_to_level(&str.to_string(), true).unwrap());
    }

    #[test]
    fn lvl_to_str_test() {
        let lvl = "1,8,2,195,3,15;1,8,2,225,3,15,21,1004;1,8,2,255,3,15,21,1004;";
        println!("{}", level_to_string(&lvl.to_string(), true).unwrap())
    }


}