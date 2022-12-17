pub mod static_module;
pub mod integer_module;
use super::class_file::{attribute_info, method_info, const_type, ClassFile};
use std::collections::HashMap;
use super::util::file;


pub struct JVM {
    instructions: HashMap<u8, Instruction>,
    debug: bool
}

impl JVM {
    pub fn new() -> JVM {
        JVM { instructions: HashMap::new(), debug: true }
    }

    #[allow(dead_code)]
    pub fn print_whole_instruction_set(&self) {
        println!("Instructions: ");
        for (opcode, instruction) in &self.instructions {
            println!("{:#x}: {}", opcode, instruction.name);
        }
    }

    pub fn register(mut self, module: Box<dyn InstructionModule>) -> JVM {
        let instructions = module.get_instructions();
        for (opcode, handler) in instructions {
            self.instructions.insert(opcode, handler);
        }
        self
    }

    pub fn execute_main_method(&mut self, class_file: &ClassFile, main_method: &method_info::MethodInfo) {
        let code_attribute = find_attributes_by_name(class_file, Box::new(main_method), "Code".as_bytes().to_vec())[0].to_code_attribute();
        println!("Code: {:?}", code_attribute.get_code());
        let mut locale_variables = Vec::new();
        for _i in 0..code_attribute.get_max_locals() {
            locale_variables.push(LocalFrame::None);
        }
        self.execute_method(class_file, Frame { code: file::File::new(code_attribute.get_code()), operand_stack: vec![], locale_variables });
    }

    pub fn execute_method(&mut self, class_file: &ClassFile, mut frame: Frame) -> Option<OperandFrame>{
        while frame.code.has_next() {
            let opcode = frame.code.get_u1();
            if self.instructions.contains_key(&opcode) {
                let instruction = self.instructions.get(&opcode).unwrap();
                if self.debug {
                    println!("Instruction: {}({})", instruction.name, opcode);
                }
                let result = instruction.get_handler()(&mut frame, class_file);
                match result {
                    JVMEvent::InvokeMethod(invoke_index) => {
                        let method_ref = class_file.get_constant(invoke_index);
                        match method_ref {
                            const_type::ConstType::ConstantMethodref(class_index, type_and_name_index) => {
                                let method_name = class_file.get_name_of_member(*type_and_name_index as usize);
                                let _class_name = class_file.get_name_of_class(*class_index as usize);
                                let descriptor = class_file.get_description_of_member(*type_and_name_index as usize);
                                let method = find_methods_by_name(class_file, Box::new(class_file), method_name.into_bytes())[0];
                                let code_attribute = find_attributes_by_name(class_file, Box::new(method), "Code".as_bytes().to_vec())[0].to_code_attribute();
                                let mut locale_variables = Vec::new();
                                for _i in 0..code_attribute.get_max_locals() {
                                    locale_variables.push(LocalFrame::None);
                                }
                                let mut param_index = 1;
                                let mut arg_index = 0;
                                let mut d = 0;
                                while &descriptor[param_index..param_index+1] != ")" {
                                    let c = &descriptor[param_index..param_index+1];
                                    if c == "[" {
                                        d += 1;
                                    } else {
                                        let param = frame.pop_operand();
                                        if d == 0 {
                                            match c {
                                                "I" => {
                                                    match param {
                                                        OperandFrame::Int(integer) => {
                                                            locale_variables[arg_index] = LocalFrame::Int(integer);
                                                        }
                                                        _ => {panic!("The top Operand({:?}) was no Int!", param);}
                                                    }
                                                }
                                                _ => {panic!("The descriptor {} is no implemented!", &descriptor[param_index..param_index+1])}
                                            }
                                        } else {
                                            panic!("Can't handle arrays in decoraters!");
                                        }
                                        d = 0;
                                        arg_index += 1;
                                    }
                                    param_index += 1;
                                }
                                let return_value = self.execute_method(&class_file, Frame { code: file::File::new(code_attribute.get_code()), operand_stack: vec![], locale_variables });
                                if return_value.is_some() {
                                    frame.push_operand(return_value.unwrap());
                                }
                            }
                            _ => {panic!("The constant {:?} is not implemented for cannot be staticly invoked!", method_ref)}
                        }
                    }
                    JVMEvent::Return(return_value) => {
                        return return_value;
                    }
                    _ => {}
                }
            } else {
                panic!("The opcode({}) is not implemented!", opcode);
            }
        }
        None
    }
}

#[derive(Debug)]
pub enum OperandFrame {
    StdOut,
    Constant(const_type::ConstType),
    Int(i32),
}

#[derive(Debug, Clone)]
pub enum LocalFrame {
    Int(i32),
    None
}
#[derive(Debug)]
pub struct Frame {
    code: file::File,
    operand_stack: Vec<OperandFrame>,
    locale_variables: Vec<LocalFrame>,
}

impl Frame {
    pub fn get_u1(&mut self) -> u8 {
        self.code.get_u1()
    }
    pub fn get_u2(&mut self) -> u16 {
        self.code.get_u2()
    }
    pub fn push_operand(&mut self, operand: OperandFrame) {
        self.operand_stack.push(operand);
    }
    pub fn pop_operand(&mut self) -> OperandFrame {
        self.operand_stack.pop().unwrap()
    }
    pub fn stack_len(&self) -> usize {
        self.operand_stack.len()
    }
    pub fn store_locale_variable(&mut self, index: usize, value: LocalFrame) {
        self.locale_variables[index] = value;
    }
    pub fn load_locale_variable(&mut self, index: usize) -> LocalFrame {
        self.locale_variables[index].clone()
    }
    pub fn offset_code(&mut self, offset: isize) {
        self.code.offset_pointer(offset);
    }
}

pub enum JVMEvent {
    Return(Option<OperandFrame>),
    InvokeMethod(usize),
    None,
}

pub struct Instruction {
    name: String,
    handler: fn(&mut Frame, &ClassFile) -> JVMEvent
}

impl Instruction {
    pub fn get_handler(&self) -> fn(&mut Frame, &ClassFile) -> JVMEvent {
        self.handler
    }
}

pub trait InstructionModule {
    fn get_instructions(&self) -> Vec<(u8 , Instruction)>;
}

pub trait ContainsMethods {
    fn get_methods(&self) -> &Vec<method_info::MethodInfo>;
}

pub trait ContainsAttributes {
    fn get_attributes(&self) -> &Vec<attribute_info:: AttributeInfo>;
}


pub fn find_methods_by_name<'a>(class_file: &ClassFile, data: Box<&'a dyn ContainsMethods>, name: Vec<u8>) -> Vec<&'a method_info::MethodInfo> {
    data.get_methods().iter().filter(|f| {
        match &class_file.get_constant(f.get_name_index() as usize) {
            const_type::ConstType::ConstantUtf8(bytes) => {
                name == *bytes
            }
            _ => {false}
        }
    }).collect()
}


pub fn find_attributes_by_name<'a>(class_file: &ClassFile, data: Box<&'a dyn ContainsAttributes>, name: Vec<u8>) -> Vec<&'a attribute_info::AttributeInfo> {
    data.get_attributes().iter().filter(|f| {
        match &class_file.get_constant(f.get_attribute_name_index() as usize) {
            &const_type::ConstType::ConstantUtf8(bytes) => {
                name == *bytes
            }
            _ => {false}
        }
    }).collect()
}