use std::any::Any;
use std::cell::Ref;
use std::fmt::{Debug, Display, Formatter, Write};
use std::rc::Rc;
use crate::{CallingConvention, Element, Value, WithName, WithReturnType};
use crate::constant::Constant;
use crate::module::function::{Function, LocalElement, LocalElementWithName};
use crate::reference::Reference;
use crate::types::{function, Type};

mod terminal;
mod opperator;

#[derive(Clone, Debug)]
pub struct InstructionBlock {
    function: Function,
    name: String,
    instructions: Vec<dyn Instruction>,
    terminator: Box<dyn TerminatorInstruction>
}

impl Display for InstructionBlock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}:\n", self.name));
        let mut vec = self.instructions.clone();
        vec.push(self.terminator.clone());
        for instruction in vec {
            f.write_fmt(format_args!("    {}\n", instruction))
        }
        Ok(())
    }
}

impl Element for InstructionBlock {}

impl WithReturnType for InstructionBlock {
    fn return_type(&self) -> Box<dyn Type> {
        crate::types::label()
    }
}

impl LocalElement for InstructionBlock {
    fn function(&self) -> Rc<Function> {
        self.function.clone()
    }
}

impl LocalElementWithName for InstructionBlock {
    fn raw_name(&self) -> String {
        self.name.clone()
    }
}

trait Instruction: LocalElement {
    fn block(&self) -> Rc<InstructionBlock>;
}

impl<T> LocalElement for T where T: Instruction {
    fn function(&self) -> Rc<Function> {
        self.block().function()
    }
}

