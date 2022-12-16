use super::file;
use super::code_attribute;

#[derive(Debug)]
pub struct AttributeInfo {
    attribute_name_index: u16,
    info: Vec<u8>,
}


impl AttributeInfo {
    pub fn get_attribute_name_index(&self) -> u16 {
        self.attribute_name_index
    }

    pub fn to_code_attribute(&self) -> code_attribute::CodeAttribute {
        code_attribute::parse_file(&mut file::File::new(self.info.clone()))
    }
}

pub fn parse_file(file: &mut file::File) -> AttributeInfo {
    let attribute_name_index = file.get_u2();
    let info_length = file.get_u4() as isize;
    AttributeInfo { attribute_name_index, info: file.get_range(info_length) }
}

pub fn parse_range(file: &mut file::File, range: usize) -> Vec<AttributeInfo> {
    (0..range).into_iter().map(|f| {parse_file(file)}).collect()
}