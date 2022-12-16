use super::{InstructionModule, Frame, OperandFrame};
use crate::class_file::const_type;

pub struct StaticModule;

impl InstructionModule for StaticModule {
    fn get_instructions(&self) -> Vec<(u8 , super::Instruction)> {
       vec![(178, super::Instruction {
           name: String::from("get_static"),
           handler: |frame, class_file| {
               let static_index = frame.get_u2() as usize;
               let field_ref = class_file.get_constant(static_index);
               match field_ref {
                   &const_type::ConstType::CONSTANT_Fieldref(class_index, name_and_type_index) => {
                       let class_name = class_file.get_name_of_class(class_index as usize);
                       let name_of_member = class_file.get_name_of_member(name_and_type_index as usize);
                       if class_name == String::from("java/lang/System") && name_of_member == String::from("out") {
                           frame.push_operand(OperandFrame::StdOut);
                       } else {
                           panic!("The Classname({})/Membername({}) is not supported for instruction get_static!", class_name, name_of_member);
                       }
                   }
                   _ => {panic!("Get static is not implemented for {:?}", field_ref)}
               }
               super::JVMEvent::None
           }
       }),
       (18, super::Instruction {
           name: String::from("ldc"),
           handler: |frame, class_file| {
               let constant_index = frame.get_u1() as usize;
               frame.push_operand(OperandFrame::Constant(class_file.get_constant(constant_index).clone()));
               super::JVMEvent::None
           }
       }),
       (182, super::Instruction {
           name: String::from("invokevirtual"),
           handler: |frame, class_file| {
               let invoke_index = frame.get_u2() as usize;
               let method_ref = class_file.get_constant(invoke_index);
               match method_ref {
                   &const_type::ConstType::CONSTANT_Methodref(class_index, name_and_type_index) => {
                       let class_name = class_file.get_name_of_class(class_index as usize);
                       let name_of_member = class_file.get_name_of_member(name_and_type_index as usize);
                       if class_name == String::from("java/io/PrintStream") && name_of_member == String::from("println") {
                           if frame.stack_len() < 2 {
                               panic!("The Classname({})/Membername({}) expects 2 arguments, but provided {}!", class_name, name_of_member, frame.stack_len());
                           }
                           let param1 = frame.pop_operand();
                           let param2 = frame.pop_operand();
                           match param2 {
                               OperandFrame::StdOut => {}
                               _ => {
                                   panic!("Unsupported stream type {:?}!", param2);
                               }
                           }
                           match param1 {
                               OperandFrame::Constant(constant) => {
                                   match constant {
                                       const_type::ConstType::CONSTANT_String(string_index) => {
                                           let utf8_const = &class_file.get_constant(string_index as usize);
                                           match utf8_const {
                                               const_type::ConstType::CONSTANT_Utf8(bytes)  => {
                                                   println!("{}", String::from_utf8(bytes.clone()).unwrap());
                                               }
                                               _ => {panic!("String constant is not pointing to a utf8 constant!")}
                                           }
                                       }
                                       _ => {panic!("Println for {:?} is not implemented for that constant type!", constant)}
                                   }
                               }
                               OperandFrame::Int(integer) => {
                                   println!("{}", integer);
                               }
                               _ => {panic!("Println for {:?} is not implemented for that operand type!", param1)}
                           }
                       } else {
                           panic!("The Classname({})/Membername({}) is not supported for instruction invokevirtual!", class_name, name_of_member);
                       }
                   }
                   _=> {panic!("The instruction invokevirtual is not implemented for {:?}", method_ref)}
               }
               super::JVMEvent::None
           }
       }),
       (177, super::Instruction {
           name: String::from("return (void)"),
           handler: |_frame, _class_file| {
               super::JVMEvent::Return(None)
           }
       }),
       (167, super::Instruction {
           name: String::from("goto"),
           handler: |frame, _class_file| {
               let offset = frame.get_u2() as i16;
               frame.offset_code((offset-3) as isize);
               super::JVMEvent::Return(None)
           }
       }),]
    }
}