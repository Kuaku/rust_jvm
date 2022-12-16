use super::file;
use super::attribute_info;

#[derive(Debug)]
pub struct FieldInfo {
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    attributes: Vec<attribute_info::AttributeInfo>,
}

pub fn parse_file(file: &mut file::File) -> FieldInfo {
    let access_flags = file.get_u2();
    let name_index = file.get_u2();
    let descriptor_index = file.get_u2();
    let attributes_length = file.get_u2() as usize;
    FieldInfo { access_flags, name_index, descriptor_index, attributes: attribute_info::parse_range(file, attributes_length) }
}

pub fn parse_range(file: &mut file::File, range: usize) -> Vec<FieldInfo> {
    (0..range).into_iter().map(|f| {parse_file(file)}).collect()
}