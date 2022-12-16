pub mod static_module;
pub mod integer_module;
use super::class_file::{attribute_info, method_info, const_type, ClassFile};
use std::collections::HashMap;
use super::util::file;


pub struct JVM {
    instructions: HashMap<u8, Instruction>
}

impl JVM {
    pub fn new() -> JVM {
        JVM { instructions: HashMap::new() }
    }

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
        let mut locale_variables = Vec::new();
        for i in 0..code_attribute.get_max_locals() {
            locale_variables.push(LocalFrame::None);
        }
        self.execute_method(class_file, Frame { code: file::File::new(code_attribute.get_code()), operand_stack: vec![], locale_variables });
    }

    pub fn execute_method(&mut self, class_file: &ClassFile, mut frame: Frame) -> Option<OperandFrame>{
        while frame.code.has_next() {
            let opcode = frame.code.get_u1();
            if self.instructions.contains_key(&opcode) {
                self.instructions.get(&opcode).unwrap().get_handler()(&mut frame, class_file);
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
            const_type::ConstType::CONSTANT_Utf8(bytes) => {
                name == *bytes
            }
            _ => {false}
        }
    }).collect()
}


pub fn find_attributes_by_name<'a>(class_file: &ClassFile, data: Box<&'a dyn ContainsAttributes>, name: Vec<u8>) -> Vec<&'a attribute_info::AttributeInfo> {
    data.get_attributes().iter().filter(|f| {
        match &class_file.get_constant(f.get_attribute_name_index() as usize) {
            &const_type::ConstType::CONSTANT_Utf8(bytes) => {
                name == *bytes
            }
            _ => {false}
        }
    }).collect()
}