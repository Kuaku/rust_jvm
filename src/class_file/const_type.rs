use super::file;

#[derive(Debug, Clone)]
pub enum ConstType {
    ConstantClass(u16),
    ConstantFieldref(u16, u16),
    ConstantMethodref(u16, u16),
    ConstantInterfaceMethodref(u16, u16),
    ConstantString(u16),
    ConstantInteger(u32),
    ConstantFloat(u32),
    ConstantLong(u32, u32),
    ConstantDouble(u32, u32),
    ConstantNameAndType(u16, u16),
    ConstantUtf8(Vec<u8>),
    ConstantMethodHandle(u8, u16),
    ConstantMethodType(u16),
    ConstantInvokeDynamic(u16, u16),
}

pub fn parse_file(file: &mut file::File) -> ConstType {
    let tag = file.get_u1();
    match tag {
        7 => {ConstType::ConstantClass(file.get_u2())}
        9 => {ConstType::ConstantFieldref(file.get_u2(), file.get_u2())}
        10 => {ConstType::ConstantMethodref(file.get_u2(), file.get_u2())}
        11 => {ConstType::ConstantInterfaceMethodref(file.get_u2(), file.get_u2())}
        8 => {ConstType::ConstantString(file.get_u2())}
        3 => {ConstType::ConstantInteger(file.get_u4())}
        4 => {ConstType::ConstantFloat(file.get_u4())}
        5 => {ConstType::ConstantLong(file.get_u4(), file.get_u4())}
        6 => {ConstType::ConstantDouble(file.get_u4(), file.get_u4())}
        12 => {ConstType::ConstantNameAndType(file.get_u2(), file.get_u2())}
        1 => {
            let string_length = file.get_u2() as isize;
            ConstType::ConstantUtf8(file.get_range(string_length))
        }
        15 => {ConstType::ConstantMethodHandle(file.get_u1(), file.get_u2())}
        16 => {ConstType::ConstantMethodType(file.get_u2())}
        18 => {ConstType::ConstantInvokeDynamic(file.get_u2(), file.get_u2())}
        _ => {panic!();}
    }
}

pub fn parse_range(file: &mut file::File, range: usize) -> Vec<ConstType> {
    (1..range).into_iter().map(|_f| {parse_file(file)}).collect()
}