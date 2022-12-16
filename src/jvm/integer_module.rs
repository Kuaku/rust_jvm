use super::{InstructionModule, Frame, OperandFrame, LocalFrame};

pub struct IntegerModule;

impl IntegerModule {
    pub fn store_to_local(frame: &mut Frame, index: usize) {
        let operand = frame.pop_operand();
        match operand {
            OperandFrame::Int(integer) => {
                frame.store_locale_variable(index, LocalFrame::Int(integer));
            }
            _ => {panic!("The top Operand({:?}) was no Int!", operand);}
        }
    }
    pub fn load_to_stack(frame: &mut Frame, index: usize) {
        let local_value = frame.load_locale_variable(index);
        match local_value {
            LocalFrame::Int(integer) => {
                frame.push_operand(OperandFrame::Int(integer));
            }
            _ => {panic!("The local variable ({:?}) was no Int!", local_value);}
        }
    }
    pub fn load_local_variable(frame: &mut Frame, index: usize) -> i32{
        let local_value = frame.load_locale_variable(index);
        match local_value {
            LocalFrame::Int(integer) => {
                integer
            }
            _ => {panic!("The local variable ({:?}) was no Int!", local_value);}
        }
    }
    pub fn try_to_pop_integer(frame: &mut Frame) -> i32 {
        let param = frame.pop_operand();
        match param {
            OperandFrame::Int(integer) => {
                integer
            }
        _ => {panic!("The top Operand({:?}) was no Int!", param);}
        }
    }
}

impl InstructionModule for IntegerModule {
    fn get_instructions(&self) -> Vec<(u8 , super::Instruction)> {
        vec![(16, super::Instruction {
            name: String::from("bipush"),
            handler: |frame, _class_file| {
                let integer = frame.get_u1() as i32;
                frame.push_operand(OperandFrame::Int(integer));
                super::JVMEvent::None
            }
        }),
        (54, super::Instruction {
            name: String::from("istore"),
            handler: |frame, _class_file| {
                let index = frame.get_u2() as usize;
                IntegerModule::store_to_local(frame, index);
                super::JVMEvent::None
            }
        }),
        (59, super::Instruction {
            name: String::from("istore_n(n=0)"),
            handler: |frame, _class_file| {
                IntegerModule::store_to_local(frame, 0);
                super::JVMEvent::None
            }
        }),
        (60, super::Instruction {
            name: String::from("istore_n(n=1)"),
            handler: |frame, _class_file| {
                IntegerModule::store_to_local(frame, 1);
                super::JVMEvent::None
            }
        }),
        (61, super::Instruction {
            name: String::from("istore_n(n=2)"),
            handler: |frame, _class_file| {
                IntegerModule::store_to_local(frame, 2);
                super::JVMEvent::None
            }
        }),
        (62, super::Instruction {
            name: String::from("istore_n(n=3)"),
            handler: |frame, _class_file| {
                IntegerModule::store_to_local(frame, 3);
                super::JVMEvent::None
            }
        }),
        (21, super::Instruction {
            name: String::from("iload"),
            handler: |frame, _class_file| {
                let index = frame.get_u2() as usize;
                IntegerModule::load_to_stack(frame, index);
                super::JVMEvent::None
            }
        }),
        (26, super::Instruction {
            name: String::from("iload_n(n=0)"),
            handler: |frame, _class_file| {
                IntegerModule::load_to_stack(frame, 0);
                super::JVMEvent::None
            }
        }),
        (27, super::Instruction {
            name: String::from("iload_n(n=1)"),
            handler: |frame, _class_file| {
                IntegerModule::load_to_stack(frame, 1);
                super::JVMEvent::None
            }
        }),
        (28, super::Instruction {
            name: String::from("iload_n(n=2)"),
            handler: |frame, _class_file| {
                IntegerModule::load_to_stack(frame, 2);
                super::JVMEvent::None
            }
        }),
        (29, super::Instruction {
            name: String::from("iload_n(n=3)"),
            handler: |frame, _class_file| {
                IntegerModule::load_to_stack(frame, 3);
                super::JVMEvent::None
            }
        }),
        (2, super::Instruction {
            name: String::from("iconst_i(-1)"),
            handler: |frame, _class_file| {
                frame.push_operand(OperandFrame::Int(-1));
                super::JVMEvent::None
            }
        }),
        (3, super::Instruction {
            name: String::from("iconst_i(0)"),
            handler: |frame, _class_file| {
                frame.push_operand(OperandFrame::Int(0));
                super::JVMEvent::None
            }
        }),
        (4, super::Instruction {
            name: String::from("iconst_i(1)"),
            handler: |frame, _class_file| {
                frame.push_operand(OperandFrame::Int(1));
                super::JVMEvent::None
            }
        }),
        (5, super::Instruction {
            name: String::from("iconst_i(2)"),
            handler: |frame, _class_file| {
                frame.push_operand(OperandFrame::Int(2));
                super::JVMEvent::None
            }
        }),
        (6, super::Instruction {
            name: String::from("iconst_i(3)"),
            handler: |frame, _class_file| {
                frame.push_operand(OperandFrame::Int(3));
                super::JVMEvent::None
            }
        }),
        (7, super::Instruction {
            name: String::from("iconst_i(4)"),
            handler: |frame, _class_file| {
                frame.push_operand(OperandFrame::Int(4));
                super::JVMEvent::None
            }
        }),
        (8, super::Instruction {
            name: String::from("iconst_i(5)"),
            handler: |frame, _class_file| {
                frame.push_operand(OperandFrame::Int(5));
                super::JVMEvent::None
            }
        }),
        (96, super::Instruction {
            name: String::from("iadd"),
            handler: |frame, _class_file| {
                let param1 = IntegerModule::try_to_pop_integer(frame);
                let param2 = IntegerModule::try_to_pop_integer(frame);
                frame.push_operand(OperandFrame::Int(param1+param2));
                super::JVMEvent::None
            }
        }),
        (104, super::Instruction {
            name: String::from("imul"),
            handler: |frame, _class_file| {
                let param1 = IntegerModule::try_to_pop_integer(frame);
                let param2 = IntegerModule::try_to_pop_integer(frame);
                frame.push_operand(OperandFrame::Int(param1*param2));
                super::JVMEvent::None
            }
        }),
        (100, super::Instruction {
            name: String::from("isub"),
            handler: |frame, _class_file| {
                let param1 = IntegerModule::try_to_pop_integer(frame);
                let param2 = IntegerModule::try_to_pop_integer(frame);
                frame.push_operand(OperandFrame::Int(param2-param1));
                super::JVMEvent::None
            }
        }),
        (108, super::Instruction {
            name: String::from("idiv"),
            handler: |frame, _class_file| {
                let param1 = IntegerModule::try_to_pop_integer(frame);
                let param2 = IntegerModule::try_to_pop_integer(frame);
                frame.push_operand(OperandFrame::Int(param2/param1));
                super::JVMEvent::None
            }
        }),
        (112, super::Instruction {
            name: String::from("imod"),
            handler: |frame, _class_file| {
                let param1 = IntegerModule::try_to_pop_integer(frame);
                let param2 = IntegerModule::try_to_pop_integer(frame);
                frame.push_operand(OperandFrame::Int(param2%param1));
                super::JVMEvent::None
            }
        }),
        (154, super::Instruction {
            name: String::from("ifne"),
            handler: |frame, _class_file| {
                let param1 = IntegerModule::try_to_pop_integer(frame);
                let offset = frame.get_u2();
                if param1 != 0 {
                    frame.offset_code((offset-3) as isize);
                }
                super::JVMEvent::None
            }
        }),
        (159, super::Instruction {
            name: String::from("if_icmpeq"),
            handler: |frame, _class_file| {
                let param1 = IntegerModule::try_to_pop_integer(frame);
                let param2 = IntegerModule::try_to_pop_integer(frame);
                let offset = frame.get_u2();
                if param1 == param2 {
                    frame.offset_code((offset-3) as isize);
                }
                super::JVMEvent::None
            }
        }),
        (160, super::Instruction {
            name: String::from("if_icmpne"),
            handler: |frame, _class_file| {
                let param1 = IntegerModule::try_to_pop_integer(frame);
                let param2 = IntegerModule::try_to_pop_integer(frame);
                let offset = frame.get_u2();
                if param1 != param2 {
                    frame.offset_code((offset-3) as isize);
                }
                super::JVMEvent::None
            }
        }),
        (161, super::Instruction {
            name: String::from("if_icmplt"),
            handler: |frame, _class_file| {
                let param1 = IntegerModule::try_to_pop_integer(frame);
                let param2 = IntegerModule::try_to_pop_integer(frame);
                let offset = frame.get_u2();
                if  param2 < param1 {
                    frame.offset_code((offset-3) as isize);
                }
                super::JVMEvent::None
            }
        }),
        (162, super::Instruction {
            name: String::from("if_icmpge"),
            handler: |frame, _class_file| {
                let param1 = IntegerModule::try_to_pop_integer(frame);
                let param2 = IntegerModule::try_to_pop_integer(frame);
                let offset = frame.get_u2();
                if  param2 >= param1 {
                    frame.offset_code((offset-3) as isize);
                }
                super::JVMEvent::None
            }
        }),
        (132, super::Instruction {
            name: String::from("iinc"),
            handler: |frame, _class_file| {
                let index = frame.get_u1() as usize;
                let constant = frame.get_u1() as i8;
                let mom_value = IntegerModule::load_local_variable(frame, index);
                frame.store_locale_variable(index, LocalFrame::Int(mom_value+i32::from(constant)));
                super::JVMEvent::None
            }
        }),
        (172, super::Instruction {
            name: String::from("ireturn"),
            handler: |frame, _class_file| {
                //TODO: Type checking
                super::JVMEvent::Return(Some(frame.pop_operand()))
            }
        })]
    }
}
