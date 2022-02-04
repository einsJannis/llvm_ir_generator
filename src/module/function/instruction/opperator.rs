use std::fmt::{Display, Formatter};
use std::rc::Rc;
use crate::module::function::instruction::{Instruction, InstructionBlock};
use crate::{Element, Value, WithReturnType};

#[derive(Clone, Debug)]
struct Add {
    block: Rc<InstructionBlock>,
    no_unsigned_wrap: bool,
    no_signed_wrap: bool,
    lhs: Box<dyn Value>,
    rhs: Box<dyn Value>
}

impl Display for Add {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("add ")?;
        if self.no_unsigned_wrap {
            f.write_str("nuw ")?;
        }
        if self.no_signed_wrap {
            f.write_str("nsw ")?;
        }
        f.write_fmt(format_args!("{} {}, {}", self.lhs.return_type(), self.lhs, self.rhs))
    }
}

impl Instruction for Add {
    fn block(&self) -> Rc<InstructionBlock> {
        self.block.clone()
    }
}

impl Element for Add {}

impl WithReturnType for Add {
    fn return_type(&self) -> Box<dyn Type> {
        self.lhs.return_type()
    }
}

#[derive(Clone, Debug)]
struct FloatingAdd {
    block: Rc<InstructionBlock>,
    flags: Vec<FastMathFlag>,
    lhs: Box<dyn Value>,
    rhs: Box<dyn Value>
}

impl Display for FloatingAdd {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("fadd ")?;
        for flag in self.flags {
            todo!()
        }
        f.write_fmt(format_args!("{} {}, {}", self.lhs.return_type(), self.lhs, self.rhs))
    }
}

impl Instruction for FloatingAdd {
    fn block(&self) -> Rc<InstructionBlock> {
        self.block.clone()
    }
}

impl Element for FloatingAdd {}

impl WithReturnType for FloatingAdd {
    fn return_type(&self) -> Box<dyn Type> {
        self.lhs.return_type()
    }
}

struct Subtraction {

}
