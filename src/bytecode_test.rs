#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::{bytecode::{ByteCodeInstruction, InstrIAdd, InstrIShl, InstrIShr, InstrLShl, InstrLShr}, jvm::{JavaInstance, StackFrame}, native_java_classes::NativeObjectInstance};

    fn get_stack_frame() -> StackFrame {
        let var = Rc::new(RefCell::new(NativeObjectInstance {}));
        let variables: [Rc<RefCell<dyn JavaInstance>>; 16] = [var.clone(), var.clone(), var.clone(), var.clone(),
            var.clone(), var.clone(), var.clone(), var.clone(),
            var.clone(), var.clone(), var.clone(), var.clone(),
            var.clone(), var.clone(), var.clone(), var.clone()];

            StackFrame::new(variables)
    }

    #[test]
    fn test_instr_iadd() {
        let mut sf = get_stack_frame();
        sf.push_int(42);
        sf.push_int(3);

        let instr = InstrIAdd {};
        instr.execute(&mut sf);

        let result = sf.pop_int();
        assert_eq!(result, 45);
    }

    #[test]
    fn test_instr_ishl() {
        let mut sf = get_stack_frame();
        sf.push_int(42);
        sf.push_int(3);

        let instr = InstrIShl {};
        instr.execute(&mut sf);
        
        let result = sf.pop_int();
        assert_eq!(result, 336);
    }

    #[test]
    fn test_instr_lshl() {
        let mut sf = get_stack_frame();
        sf.push_long(42);
        sf.push_long(34);

        let instr = InstrLShl {};
        instr.execute(&mut sf);
        
        let result = sf.pop_long();
        assert_eq!(result, 721554505728);
    }

    #[test]
    fn test_instr_ishr() {
        let mut sf = get_stack_frame();
        sf.push_int(336);
        sf.push_int(3);

        let instr = InstrIShr {};
        instr.execute(&mut sf);
        
        let result = sf.pop_int();
        assert_eq!(result, 42);
    }

    #[test]
    fn test_instr_lshr() {
        let mut sf = get_stack_frame();
        sf.push_long(721554505728);
        sf.push_long(34);

        let instr = InstrLShr {};
        instr.execute(&mut sf);
        
        let result = sf.pop_long();
        assert_eq!(result, 42);
    }

}
