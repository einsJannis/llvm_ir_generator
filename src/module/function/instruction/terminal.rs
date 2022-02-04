use std::fmt::{Debug, Display, Formatter};
use std::rc::Rc;
use crate::module::function::Function;
use crate::module::function::instruction::{Instruction, InstructionBlock};
use crate::reference::Reference;
use crate::types::Type;
use crate::{Element, Value, WithReturnType};

trait TerminatorInstruction: Instruction {}

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
    block: Rc<InstructionBlock>,
    function: Reference<Function>,
    arguments: Vec<Box<dyn Value>>,
    operand_bundles: Vec<OperandBundel>,
    normal_label: Reference<InstructionBlock>,
    exception_label: Reference<InstructionBlock>
}

impl Display for Invoke {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("invoke ")?;
        if let Some(calling_convention) = self.function.get().calling_convention.clone() {
            f.write_fmt(format_args!("{} ", calling_convention))?;
        }
        if let Some(return_attributes) = self.function.get().return_attribute {
            f.write_fmt(format_args!("{} ", return_attributes))?;
        }
        if let Some(address_space) = self.function.get().addr_space {
            f.write_fmt(format_args!("addrspace({}) ", address_space))?;
        }
        f.write_fmt(format_args!("{} {}", self.function.get().return_type, self.function))?;
        f.write_str("( ")?;
        for (i, value) in self.arguments.iter().enumerate() {
            f.write_fmt(format_args!("{} {}", value.return_type(), value))?;
            if i != self.arguments.len() {
                f.write_str(", ")?;
            }
        }
        f.write_str(" )")?;
        for attribute in self.function.attributes {
            todo!()
        }
        for operand_bundle in self.operand_bundles {
            todo!()
        }
        f.write_fmt(format_args!(" to label {} unwind label {}", self.normal_label, self.exception_label))?;
        Ok(())
    }
}

impl Instruction for Invoke {
    fn block(&self) -> Rc<InstructionBlock> {
        self.block.clone()
    }
}

impl Element for Invoke {}

impl WithReturnType for Invoke {
    fn return_type(&self) -> Box<dyn Type> {
        self.function.get().return_type()
    }
}

impl TerminatorInstruction for Invoke {}

// CallBranch not implemented

#[derive(Clone, Debug)]
struct Resume {
    block: Rc<InstructionBlock>,
    value: Box<dyn Value>
}

impl Display for Resume {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("resume {} {}", self.value.return_type(), self.value))
    }
}

impl Instruction for Resume {
    fn block(&self) -> Rc<InstructionBlock> {
        self.block.clone()
    }
}

impl TerminatorInstruction for Resume {}

//CatchSwitch not implemented

//CatchReturn not implemented

//CleanupReturn not implemented

#[derive(Clone, Debug)]
struct Unreachable {
    block: Rc<InstructionBlock>
}

impl Display for Unreachable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("unreachable")
    }
}

impl Instruction for Unreachable {
    fn block(&self) -> Rc<InstructionBlock> {
        self.block.clone()
    }
}
