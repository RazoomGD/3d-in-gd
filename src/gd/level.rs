use crate::formats::xml_and_dat;

pub struct GdConfig {
    pub xml: String, //cclocallevels content
    pub level_name: String, //in gd
    pub file_path: String,
}

pub fn insert_level(config: GdConfig) -> Result<(), &'static str> {
    Ok(())
}


pub fn extract_level(congig: GdConfig) -> Result<(), &'static str> {
    Ok(())
}