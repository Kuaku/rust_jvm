use super::exception_table_entry;
use super::attribute_info;
use super::file;

#[derive(Debug)]
pub struct CodeAttribute {
    max_stack: u16,
    max_locals: u16,
    code: Vec<u8>,
    exception_table: Vec<exception_table_entry::ExceptionTableEntry>,
    attributes: Vec<attribute_info::AttributeInfo>,
}

impl CodeAttribute {
    pub fn get_code(&self) -> Vec<u8> {
        self.code.clone()
    }

    pub fn get_max_locals(&self) -> u16 {
        self.max_locals
    }
}

pub fn parse_file(file: &mut file::File) -> CodeAttribute {
    let max_stack = file.get_u2();
    let max_locals = file.get_u2();
    let code_length = file.get_u4() as usize;
    let code = file.get_range(code_length as isize);
    let execption_table_length = file.get_u2() as usize;
    let exception_table = exception_table_entry::parse_range(file, execption_table_length);
    let attributes_count = file.get_u2() as usize;
    let attributes = attribute_info::parse_range(file, attributes_count);

    CodeAttribute { max_stack, max_locals, code, exception_table, attributes }
}