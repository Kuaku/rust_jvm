use super::file;
use super::attribute_info;

#[derive(Debug)]
pub struct FieldInfo {
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    attributes: Vec<attribute_info::AttributeInfo>,
}

impl FieldInfo {

    #[allow(dead_code)]
    pub fn get_access_flags(&self) -> u16 {
        self.access_flags
    }

    #[allow(dead_code)]
    pub fn get_name_index(&self) -> u16 {
        self.name_index
    }

    #[allow(dead_code)]
    pub fn get_descriptor_index(&self) -> u16 {
        self.descriptor_index
    }

    #[allow(dead_code)]
    pub fn get_attributes(&self) -> Vec<attribute_info::AttributeInfo> {
        self.attributes.clone()
    }
}

pub fn parse_file(file: &mut file::File) -> FieldInfo {
    let access_flags = file.get_u2();
    let name_index = file.get_u2();
    let descriptor_index = file.get_u2();
    let attributes_length = file.get_u2() as usize;
    FieldInfo { access_flags, name_index, descriptor_index, attributes: attribute_info::parse_range(file, attributes_length) }
}

pub fn parse_range(file: &mut file::File, range: usize) -> Vec<FieldInfo> {
    (0..range).into_iter().map(|_f| {parse_file(file)}).collect()
}