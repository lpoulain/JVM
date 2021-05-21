use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use crate::java_class::ConstantField;
use crate::java_class::ConstantString;
use crate::java_class::ConstantStringRef;
use crate::java_class::ConstantClass;
use crate::java_class::ConstantMethod;
use crate::java_class::ConstantNameType;
use crate::JVM;
use crate::java_class::Blob;
use crate::java_class::BytecodeClass;
use crate::jvm::JavaObject;
use crate::Classes;

pub trait ByteCodeInstruction {
    fn execute(&self, class: &BytecodeClass, jvm: &mut JVM, classes: &Classes) -> InstrNextAction;
    fn print(&self);
    fn set_branch(&mut self, _address_map: &HashMap<usize, usize>) {}
}

pub enum InstrNextAction {
    NEXT,
    RETURN,
    GOTO(usize)
}

////////////////////////////////////////////////////////////////////////////////////

pub struct InstrALoad0 {}
impl ByteCodeInstruction for InstrALoad0 {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        jvm.push(jvm.var0.clone());
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      aload0"); }
}

pub struct InstrInvokeSpecial { class_name: String, method_name: String, type_desc: String }
impl ByteCodeInstruction for InstrInvokeSpecial {
    fn execute(&self, _class: &BytecodeClass, _jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      invokespecial {} {} {}", self.class_name, self.method_name, self.type_desc); }
}

pub struct InstrReturn {}
impl ByteCodeInstruction for InstrReturn {
    fn execute(&self, _class: &BytecodeClass, _jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        return InstrNextAction::RETURN;
    }
    fn print(&self) { println!("      return"); }
}

pub struct InstrGetStatic { class_name: String, field_name: String, type_desc: String }
impl ByteCodeInstruction for InstrGetStatic {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, classes: &Classes) -> InstrNextAction {
        let class = classes.get_class(&self.class_name);
        jvm.push(Rc::new(class.get_static_object(&self.field_name)));
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      getstatic {} {} {}", self.class_name, self.field_name, self.type_desc); }
}

pub struct InstrLdc { string: String }
impl ByteCodeInstruction for InstrLdc {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        jvm.push(Rc::new(JavaObject::STRING(self.string.clone())));
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      ldc \"{}\"", self.string); }
}

pub struct InstrInvokeVirtual { class_name: String, method_name: String, type_desc: String }
impl ByteCodeInstruction for InstrInvokeVirtual {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, classes: &Classes) -> InstrNextAction {
        let class = classes.get_class(&self.class_name);
        class.execute_method(jvm, classes, &self.method_name);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      invokevirtual {} {} {}", self.class_name, self.method_name, self.type_desc); }
}

pub struct InstrIConst0 {}
impl ByteCodeInstruction for InstrIConst0 {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        jvm.push(Rc::new(JavaObject::INTEGER(0)));
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      iconst_0"); }
}

pub struct InstrIConst1 {}
impl ByteCodeInstruction for InstrIConst1 {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        jvm.push(Rc::new(JavaObject::INTEGER(1)));
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      iconst_1"); }
}

pub struct InstrIConst2 {}
impl ByteCodeInstruction for InstrIConst2 {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        jvm.push(Rc::new(JavaObject::INTEGER(2)));
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      iconst_2"); }
}

pub struct InstrAALoad {}
impl ByteCodeInstruction for InstrAALoad {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        let idx = jvm.pop_int();
        let val = jvm.pop();
        let array = match &*val {
            JavaObject::ARRAY(array) => array.borrow(),
            _ => panic!("Unknown object in the stack. Was expecting an array")
        };
        let object: &Rc<JavaObject> = match array.get(idx as usize) {
            Some(obj) => obj,
            _ => panic!("No object in the array at index {}", idx)
        };
        jvm.push(object.clone());
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      aaload"); }
}

pub struct InstrAAStore {}
impl ByteCodeInstruction for InstrAAStore {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        let object = jvm.pop();
        let idx = jvm.pop_int();
        let arg = jvm.pop();
        let array = match &*arg {
            JavaObject::ARRAY(array) => array,
            _ => panic!("Excepted array in the stack")
        };
        array.borrow_mut()[idx as usize] = object;
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      aastore"); }
}

pub struct InstrInvokeStatic { class_name: String, method_name: String, type_desc: String }
impl ByteCodeInstruction for InstrInvokeStatic {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, classes: &Classes) -> InstrNextAction {
        let class = classes.get_class(&self.class_name);
        class.execute_static_method(jvm, classes, &self.method_name);
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      invokestatic {} {} {}", self.class_name, self.method_name, self.type_desc); }
}

pub struct InstrIStore1 {}
impl ByteCodeInstruction for InstrIStore1 {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        jvm.var1 = jvm.pop().clone();
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      istore_1"); }
}

pub struct InstrIStore2 {}
impl ByteCodeInstruction for InstrIStore2 {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        jvm.var2 = jvm.pop().clone();
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      istore_2"); }
}

pub struct InstrISub {}
impl ByteCodeInstruction for InstrISub {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        let nb1 = jvm.pop_int();
        let nb2 = jvm.pop_int();
        jvm.push(Rc::new(JavaObject::INTEGER(nb2 - nb1)));
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      isub"); }
}

pub struct InstrIDiv {}
impl ByteCodeInstruction for InstrIDiv {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        let nb1 = jvm.pop_int();
        let nb2 = jvm.pop_int();
        jvm.push(Rc::new(JavaObject::INTEGER(nb2 / nb1)));
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      idiv"); }
}

pub struct InstrILoad1 {}
impl ByteCodeInstruction for InstrILoad1 {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        jvm.push(jvm.var1.clone());
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      iload_1"); }
}

pub struct InstrILoad2 {}
impl ByteCodeInstruction for InstrILoad2 {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        jvm.push(jvm.var2.clone());
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      iload_2"); }
}

pub struct InstrIRem {}
impl ByteCodeInstruction for InstrIRem {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        let nb1 = jvm.pop_int();
        let nb2 = jvm.pop_int();
        jvm.push(Rc::new(JavaObject::INTEGER(nb2 % nb1)));
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      irem"); }
}

pub struct InstrIfne { branch: usize }
impl ByteCodeInstruction for InstrIfne {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        let arg = jvm.pop();
        match &*arg {
            JavaObject::INTEGER(int) => {
                if *int != 0 {
                    return InstrNextAction::GOTO(self.branch);
                }
            },
            _ => panic!("ifne expects an integer in the stack")
        };
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      ifne {}", self.branch); }
    fn set_branch(&mut self, address_map: &HashMap<usize, usize>) {
        match address_map.get(&self.branch) {
            Some(instr_idx) => { self.branch = *instr_idx; },
            _ => panic!("Unknown branch position {}", self.branch)
        }
    }
}

pub struct InstrGoto { branch: usize }
impl ByteCodeInstruction for InstrGoto {
    fn execute(&self, _class: &BytecodeClass, _jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        return InstrNextAction::GOTO(self.branch);
    }
    fn print(&self) { println!("      goto {}", self.branch); }
    fn set_branch(&mut self, address_map: &HashMap<usize, usize>) {
        match address_map.get(&self.branch) {
            Some(instr_idx) => { self.branch = *instr_idx; },
            _ => panic!("Unknown branch position {}", self.branch)
        }
    }
}

pub struct InstrDup { }
impl ByteCodeInstruction for InstrDup {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        let arg = jvm.pop();
        jvm.push(arg.clone());
        jvm.push(arg.clone());
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      dup"); }
}

pub struct InstrANewArray { class_name: String }
impl ByteCodeInstruction for InstrANewArray {
    fn execute(&self, _class: &BytecodeClass, jvm: &mut JVM, _classes: &Classes) -> InstrNextAction {
        let count = jvm.pop_int();
        let mut array: Vec<Rc<JavaObject>> = Vec::with_capacity(count as usize);
        for _i in 0..count {
            array.push(Rc::new(JavaObject::INTEGER(0)));
        }
        jvm.push(Rc::new(JavaObject::ARRAY(RefCell::new(array))));
        return InstrNextAction::NEXT;
    }
    fn print(&self) { println!("      anewarray {}", self.class_name); }
}

////////////////////////////////////////////////////////////////////////////////////

pub struct ByteCode {
    pub instructions: Vec<Box<dyn ByteCodeInstruction>>
}

impl ByteCode {
    pub fn new(data: &mut Blob, constants_class: &HashMap<usize, ConstantClass>,
        _constants_string: &HashMap<usize, ConstantString>,
        constants_string_ref: &HashMap<usize, ConstantStringRef>,
        constants_method: &HashMap<usize, ConstantMethod>,
        constants_field: &HashMap<usize, ConstantField>,
        _constants_name_type: &HashMap<usize, ConstantNameType>, debug:u8) -> ByteCode {

        let mut instructions: Vec<Box<dyn ByteCodeInstruction>> = Vec::new();
        data.rewind();

        let mut data_offset: usize;
        let mut address_map: HashMap<usize, usize> = HashMap::new();
        let mut instr_idx: usize = 0;

        while data.has_more_data() {
            data_offset = data.get_offset();
            address_map.insert(data_offset, instr_idx);

            let opcode = data.get_u8();
            let instr: Box<dyn ByteCodeInstruction> = match opcode {
                0x03 => Box::new(InstrIConst0 {}),
                0x04 => Box::new(InstrIConst1 {}),
                0x05 => Box::new(InstrIConst2 {}),
                0x12 => Box::new(InstrLdc { string: match constants_string_ref.get(&(data.get_u8() as usize)) {
                    Some(string) => string.value.clone(),
                    _ => panic!("Unknown string")
                } }),
                0x1b => Box::new(InstrILoad1 {}),
                0x1c => Box::new(InstrILoad2 {}),
                0x2a => Box::new(InstrALoad0 {}),
                0x32 => Box::new(InstrAALoad {}),
                0x3c => Box::new(InstrIStore1 {}),
                0x3d => Box::new(InstrIStore2 {}),
                0x53 => Box::new(InstrAAStore {}),
                0x59 => Box::new(InstrDup {}),
                0x64 => Box::new(InstrISub {}),
                0x6c => Box::new(InstrIDiv {}),
                0x70 => Box::new(InstrIRem {}),
                0x9a => Box::new(InstrIfne { branch: data_offset + data.get_u16size() }),
                0xa7 => Box::new(InstrGoto { branch: data_offset + data.get_u16size() }),
                0xb1 => Box::new(InstrReturn {}),
                0xb2 => match constants_field.get(&data.get_u16size()) {
                    Some(method) => Box::new(InstrGetStatic {
                        class_name: method.class_name.clone(),
                        field_name: method.field_name.clone(),
                        type_desc: method.type_name.clone()
                    }),
                    _ => panic!("Unknown field")
                },
                0xb6 => match constants_method.get(&data.get_u16size()) {
                    Some(method) => Box::new(InstrInvokeVirtual {
                        class_name: method.class_name.clone(),
                        method_name: method.method_name.clone(),
                        type_desc: method.type_name.clone()
                    }),
                    _ => panic!("Unknown method")
                },
                0xb7 => match constants_method.get(&data.get_u16size()) {
                    Some(method) => Box::new(InstrInvokeSpecial {
                        class_name: method.class_name.clone(),
                        method_name: method.method_name.clone(),
                        type_desc: method.type_name.clone()
                    }),
                    _ => panic!("Unknown method")
                },
                0xb8 => match constants_method.get(&data.get_u16size()) {
                    Some(method) => Box::new(InstrInvokeStatic {
                        class_name: method.class_name.clone(),
                        method_name: method.method_name.clone(),
                        type_desc: method.type_name.clone()
                    }),
                    _ => panic!("Unknown method")
                },
                0xbd => match constants_class.get(&data.get_u16size()) {
                    Some(class) => Box::new(InstrANewArray {
                        class_name: class.name.clone()
                    }),
                    _ => panic!("Unknown class")
                },
                _ => panic!("Unknown opcode {:#02x}", opcode)
            };

            if debug >= 2 { instr.print(); }
            instructions.push(instr);

            instr_idx += 1;
        }

        for instr in instructions.iter_mut() {
            instr.set_branch(&address_map);
        }

        ByteCode {
            instructions: instructions
        }
    }
}