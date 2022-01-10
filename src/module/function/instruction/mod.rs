use std::any::Any;
use std::cell::Ref;
use std::fmt::{Debug, Display, Formatter};
use std::rc::Rc;
use crate::{CallingConvention, Element, Value, WithName, WithReturnType};
use crate::constant::Constant;
use crate::module::function::{Function, LocalElement, LocalElementWithName};
use crate::reference::Reference;
use crate::types::{function, Type};

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

trait TerminatorInstruction: Instruction {}

impl<T> WithReturnType for T where T: TerminatorInstruction {
    fn return_type(&self) -> Box<dyn Type> {
        crate::types::void()
    }
}

#[derive(Clone, Debug)]
struct Return {
    instruction_block: Rc<InstructionBlock>,
    value: Box<dyn Value>
}

impl Display for Return {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.value.return_type().type_id() == crate::types::void().type_id() {
            true => f.write_str("ret void"),
            false => f.write_fmt(format_args!("ret {} {}", self.value.return_type(), self.value))
        }
    }
}

impl Instruction for Return {
    fn block(&self) -> Rc<InstructionBlock> {
        self.instruction_block.clone()
    }
}

impl TerminatorInstruction for Return {}

#[derive(Clone)]
enum BranchData {
    Conditional {
        condition: Box<dyn Value>,
        if_true: Reference<InstructionBlock>,
        if_false: Reference<InstructionBlock>
    },
    Unconditional {
        destination: Reference<InstructionBlock>
    }
}

#[derive(Clone, Debug)]
struct Branch {
    instruction_block: Rc<InstructionBlock>,
    data: BranchData
}

impl Display for Branch {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.data.clone() {
            BranchData::Conditional { condition, if_true, if_false } =>
                f.write_fmt(format_args!("br i1 {}, label {}, label {}", condition, if_true, if_false)),
            BranchData::Unconditional { destination } =>
                f.write_fmt(format_args!("br label {}", destination))
        }
    }
}

impl Instruction for Branch {
    fn block(&self) -> Rc<InstructionBlock> {
        self.instruction_block.clone()
    }
}

impl TerminatorInstruction for Branch {}

#[derive(Clone, Debug)]
struct SwitchBranch {
    compare_to: Box<dyn Value>,
    if_equal: Reference<InstructionBlock>
}

impl Display for SwitchBranch {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} {}, label {}", self.compare_to.return_type(), self.compare_to, self.if_equal))
    }
}

#[derive(Clone, Debug)]
struct Switch {
    instruction_block: Rc<InstructionBlock>,
    compare_to: Box<dyn Value>,
    default: Reference<InstructionBlock>,
    branches: Vec<SwitchBranch>
}

impl Display for Switch {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("switch {} {}, label {} [ ", self.compare_to.return_type(), self.compare_to, self.default));
        for i in 0..self.branches.len() {
            self.branches[i].fmt(f)?;
            if i < self.branches.len()-1 {
                f.write_str("\n        ")
            }
        }
        f.write_str("\n]");
        Ok(())
    }
}

impl Instruction for Switch {
    fn block(&self) -> Rc<InstructionBlock> {
        self.instruction_block.clone()
    }
}

impl TerminatorInstruction for Switch {}

#[derive(Clone, Debug)]
struct IndirectBranch {
    block: Rc<InstructionBlock>,
    value: Box<dyn Value>,
    possibilities: Vec<Reference<InstructionBlock>>
}

impl Display for IndirectBranch {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("indirectbr {} {}, [ ", self.value.return_type(), self.value))?;
        for i in 0..self.possibilities.len() {
            self.possibilities[i].fmt(f);
            if i < self.possibilities.len()-1 { f.write_str(", ") }
        }
        f.write_str(" ]");
        Ok(())
    }
}

impl Instruction for IndirectBranch {
    fn block(&self) -> Rc<InstructionBlock> {
        self.block.clone()
    }
}

impl TerminatorInstruction for IndirectBranch {}

#[derive(Clone, Debug)]
struct Invoke {
    function: Function,
    arguments: Vec<Box<dyn Value>>,
    function_attributes: Vec<FunctionAttribute>,
    operand_bundles: Vec<OperandBundel>,
    normal_label: Reference<InstructionBlock>,
    exception_label: Reference<InstructionBlock>
}

impl Display for Invoke {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("invoke ")?;
        if let Some(calling_convention) = self.function.calling_convention.clone() {
            f.write_fmt(format_args!("{} ", calling_convention))
        }
        if let Some(return_attributes) = self.function.return_attribute {
            f.write_fmt(format_args!("{} ", return_attributes))
        }
        if let Some(address_space) = self.function.addr_space {
            f.write_fmt(format_args!("addrspace({}) ", address_space))
        }
        f.write_fmt(format_args!("{} {}", self.function.return_type, self.function.reference()));
        Ok(())
    }
}
