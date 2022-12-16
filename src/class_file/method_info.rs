use super::file;
use super::attribute_info;
use super::super::jvm::ContainsAttributes;

#[derive(Debug)]
pub struct MethodInfo {
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    attributes: Vec<attribute_info::AttributeInfo>,
}

impl MethodInfo {
    pub fn get_name_index(&self) -> u16 {
        self.name_index
    }
}

impl ContainsAttributes for MethodInfo {
    fn get_attributes(&self) -> &Vec<attribute_info:: AttributeInfo> {
        &self.attributes
    }
}


pub fn parse_file(file: &mut file::File) -> MethodInfo {
    let access_flags = file.get_u2();
    let name_index = file.get_u2();
    let descriptor_index = file.get_u2();
    let attributes_length = file.get_u2() as usize;
    MethodInfo { access_flags, name_index, descriptor_index, attributes: attribute_info::parse_range(file, attributes_length) }
}

pub fn parse_range(file: &mut file::File, range: usize) -> Vec<MethodInfo> {
    (0..range).into_iter().map(|f| {parse_file(file)}).collect()
}
