use super::file;

#[derive(Debug)]
pub struct ExceptionTableEntry {
    start_pc: u16,
    end_pc: u16,
    handler_pc: u16,
    catch_type: u16,
}

pub fn parse_file(file: &mut file::File) -> ExceptionTableEntry {
    ExceptionTableEntry { start_pc: file.get_u2(), end_pc: file.get_u2(), handler_pc: file.get_u2(), catch_type: file.get_u2() }
}

pub fn parse_range(file: &mut file::File, range: usize) -> Vec<ExceptionTableEntry> {
    (0..range).into_iter().map(|f| {parse_file(file)}).collect()
}