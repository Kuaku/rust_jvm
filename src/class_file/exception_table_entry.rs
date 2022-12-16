use super::file;

#[derive(Debug, Clone)]
pub struct ExceptionTableEntry {
    start_pc: u16,
    end_pc: u16,
    handler_pc: u16,
    catch_type: u16,
}

impl ExceptionTableEntry {

    #[allow(dead_code)]
    fn get_start_pc(&self) -> u16 {
        self.start_pc
    }

    #[allow(dead_code)]
    fn get_end_pc(&self) -> u16 {
        self.end_pc
    }

    #[allow(dead_code)]
    fn get_handler_pc(&self) -> u16 {
        self.handler_pc
    }

    #[allow(dead_code)]
    fn get_catch_type(&self) -> u16 {
        self.catch_type
    }
}

pub fn parse_file(file: &mut file::File) -> ExceptionTableEntry {
    ExceptionTableEntry { start_pc: file.get_u2(), end_pc: file.get_u2(), handler_pc: file.get_u2(), catch_type: file.get_u2() }
}

pub fn parse_range(file: &mut file::File, range: usize) -> Vec<ExceptionTableEntry> {
    (0..range).into_iter().map(|_f| {parse_file(file)}).collect()
}