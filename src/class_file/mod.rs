use super::util::file;
use super::jvm::ContainsMethods;
pub mod const_type;
pub mod attribute_info;
pub mod field_info;
pub mod method_info;
pub mod exception_table_entry;
pub mod code_attribute;

#[derive(Debug)]
#[allow(dead_code)]
pub struct ClassFile {
    magic: u32,
    minor_version: u16,
    major_version: u16,
    constant_pool: Vec<const_type::ConstType>,
    access_flags: u16,
    this_class: u16,
    super_class: u16,
    interfaces: Vec<u16>,
    fields: Vec<field_info::FieldInfo>,
    methods: Vec<method_info::MethodInfo>,
    attributes: Vec<attribute_info::AttributeInfo>,
}

impl ClassFile {
    pub fn get_constant(&self, constant_index: usize) -> &const_type::ConstType {
        &self.constant_pool[constant_index - 1]
    }

    pub fn get_name_of_class(&self, class_index: usize) -> String {
        let class = &self.constant_pool[class_index-1];
        match class {
            const_type::ConstType::ConstantClass(name_index) => {
                let class_name = &self.constant_pool[(*name_index-1) as usize];
                match class_name {
                    const_type::ConstType::ConstantUtf8(bytes) => {
                        String::from_utf8(bytes.clone()).unwrap()
                    }
                    _ => panic!("Given class name is no utf8 constant!")
                }
            }
            _ => {
                panic!("Given index is no class constant!");
            }
        }
    }

    pub fn get_name_of_member(&self, name_and_type_index: usize) -> String {
        let name_and_type = &self.constant_pool[name_and_type_index-1];
        match name_and_type {
            const_type::ConstType::ConstantNameAndType(name_index, _descriptior_index) => {
                let member_name = &self.constant_pool[(*name_index-1) as  usize];
                match member_name {
                    const_type::ConstType::ConstantUtf8(bytes) => {
                        String::from_utf8(bytes.clone()).unwrap()
                    }
                    _ => {panic!("Given index is no utf8 constant!")}
                }
            }
            _ => {panic!("Given index is no name and type constant!");}
        }
    }

    pub fn get_description_of_member(&self, name_and_type_index: usize) -> String {
        let name_and_type = &self.constant_pool[name_and_type_index-1];
        match name_and_type {
            const_type::ConstType::ConstantNameAndType(_name_index, descriptior_index) => {
                let member_name = &self.constant_pool[(*descriptior_index-1) as  usize];
                match member_name {
                    const_type::ConstType::ConstantUtf8(bytes) => {
                        String::from_utf8(bytes.clone()).unwrap()
                    }
                    _ => {panic!("Given index is no utf8 constant!")}
                }
            }
            _ => {panic!("Given index is no name and type constant!");}
        }
    }
}



impl ContainsMethods for ClassFile {
    fn get_methods(&self) -> &Vec<method_info::MethodInfo> {
        &self.methods
    }
}

pub fn parse_file(file: &mut file::File) -> ClassFile {
    let magic =file.get_u4();
    let minor_version = file.get_u2();
    let major_version = file.get_u2();
    let constant_count = file.get_u2() as usize;
    let constant_pool = const_type::parse_range(file, constant_count);
    let access_flags = file.get_u2();
    let this_class = file.get_u2();
    let super_class = file.get_u2();
    let interface_count = file.get_u2() as usize;
    let interfaces = file.get_range_u2(interface_count);
    let field_count = file.get_u2() as usize;
    let fields = field_info::parse_range(file, field_count);
    let methods_count = file.get_u2() as usize;
    let methods = method_info::parse_range(file, methods_count);
    let attributes_count = file.get_u2() as usize;
    let attributes = attribute_info::parse_range(file, attributes_count);
    ClassFile {
        magic,
        minor_version,
        major_version,
        constant_pool,
        access_flags,
        this_class,
        super_class,
        interfaces,
        fields,
        methods,
        attributes }
}