use super::file;

#[derive(Debug, Clone)]
pub enum ConstType {
    CONSTANT_Class(u16),
    CONSTANT_Fieldref(u16, u16),
    CONSTANT_Methodref(u16, u16),
    CONSTANT_InterfaceMethodref(u16, u16),
    CONSTANT_String(u16),
    CONSTANT_Integer(u32),
    CONSTANT_Float(u32),
    CONSTANT_Long(u32, u32),
    CONSTANT_Double(u32, u32),
    CONSTANT_NameAndType(u16, u16),
    CONSTANT_Utf8(Vec<u8>),
    CONSTANT_MethodHandle(u8, u16),
    CONSTANT_MethodType(u16),
    CONSTANT_InvokeDynamic(u16, u16),
}

pub fn parse_file(file: &mut file::File) -> ConstType {
    let tag = file.get_u1();
    match tag {
        7 => {ConstType::CONSTANT_Class(file.get_u2())}
        9 => {ConstType::CONSTANT_Fieldref(file.get_u2(), file.get_u2())}
        10 => {ConstType::CONSTANT_Methodref(file.get_u2(), file.get_u2())}
        11 => {ConstType::CONSTANT_InterfaceMethodref(file.get_u2(), file.get_u2())}
        8 => {ConstType::CONSTANT_String(file.get_u2())}
        3 => {ConstType::CONSTANT_Integer(file.get_u4())}
        4 => {ConstType::CONSTANT_Float(file.get_u4())}
        5 => {ConstType::CONSTANT_Long(file.get_u4(), file.get_u4())}
        6 => {ConstType::CONSTANT_Double(file.get_u4(), file.get_u4())}
        12 => {ConstType::CONSTANT_NameAndType(file.get_u2(), file.get_u2())}
        1 => {
            let string_length = file.get_u2() as isize;
            ConstType::CONSTANT_Utf8(file.get_range(string_length))
        }
        15 => {ConstType::CONSTANT_MethodHandle(file.get_u1(), file.get_u2())}
        16 => {ConstType::CONSTANT_MethodType(file.get_u2())}
        18 => {ConstType::CONSTANT_InvokeDynamic(file.get_u2(), file.get_u2())}
        _ => {panic!();}
    }
}

pub fn parse_range(file: &mut file::File, range: usize) -> Vec<ConstType> {
    (1..range).into_iter().map(|f| {parse_file(file)}).collect()
}