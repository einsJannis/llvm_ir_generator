use std::fmt::Display;

use crate::{reference::Value, module::function::Function, types::FunctionType};

pub enum Instruction<'s> {
    Return(ReturnInstruction<'s>),
    Branch(BranchInstruction<'s>),
    Switch(SwitchInstruction<'s>),
    IndirectBranch(IndirectBranchInstruction<'s>),
    Invoke(InvokeInstruction<'s>),
    CallBranch(CallBranchInstruction<'s>),
    Resume(ResumeInstruction<'s>),
}

impl<'s> Display for Instruction<'s> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (match self {
            Instruction::Return(it) => it as &dyn Display,
            Instruction::Branch(it) => it as &dyn Display,
            Instruction::Switch(it) => it as &dyn Display,
            Instruction::IndirectBranch(it) => it as &dyn Display,
            Instruction::Invoke(it) => it as &dyn Display,
            Instruction::CallBranch(it) => it as &dyn Display,
            Instruction::Resume(it) => it as &dyn Display,
        }).fmt(f)
    }
}

pub enum ReturningInstruction<'s> {
    Invoke(InvokeInstruction<'s>),
    CallBranch(CallBranchInstruction<'s>),
    CatchSwitch(CatchSwitchInstruction<'s>)
}

impl<'s> Display for ReturningInstruction<'s> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (match self {
            Self::Invoke(it) => it as &dyn Display,
            Self::CallBranch(it) => it as &dyn Display,
            Self::CatchSwitch(it) => it as &dyn Display
        }).fmt(f)
    }
}

pub enum ReturnInstruction<'s> {
    Void,
    NonVoid(Value<'s>)
}

impl<'s> Display for ReturnInstruction<'s> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReturnInstruction::Void => f.write_str("ret void"),
            ReturnInstruction::NonVoid(value) => f.write_fmt(format_args!("ret {}", value))
        }
    }
}

pub enum BranchInstruction<'s> {
    Unconditional { label: Value<'s> },
    Conditional { condition: Value<'s>, iflabel: Value<'s>, elselabel: Value<'s> }
}

impl Display for BranchInstruction<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unconditional { label } => f.write_fmt(format_args!("br {}", label)),
            Self::Conditional { condition, iflabel, elselabel } => f.write_fmt(format_args!("br {}, {}, {}", condition, iflabel, elselabel))
        }
    }
}

pub struct SwitchInstruction<'s> {
    input: Value<'s>,
    defaultdest: Value<'s>,
    cases: Vec<(Value<'s>, Value<'s>)>
}

impl Display for SwitchInstruction<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("switch {}, {} [", self.input, self.defaultdest))?;
        for (i, case) in self.cases.iter().enumerate() {
            f.write_fmt(format_args!("{}, {}", case.0, case.1))?;
            if i < (self.cases.len()-1) { f.write_str(", ")?; }
        }
        f.write_str(" ]")?;
        Ok(())
    }
}

pub struct IndirectBranchInstruction<'s> {
    pointer: Value<'s>,
    possible_labels: Vec<Value<'s>>
}

impl Display for IndirectBranchInstruction<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("indirectbr {}, [ ", self.pointer))?;
        for (i, possible_label) in self.possible_labels.iter().enumerate() {
            possible_label.fmt(f)?;
            if i < (self.possible_labels.len()-1) { f.write_str(", ")?; }
        }
        f.write_str(" ]")?;
        Ok(())
    }
}

pub struct InvokeInstruction<'s> {
    function_ref: &'s Function<'s>,
    arguments: Vec<Value<'s>>,
    normal: Value<'s>,
    exception: Value<'s>
}

impl Display for InvokeInstruction<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("invoke ")?;
        if let crate::module::CallingConvention::C = self.function_ref.calling_convention {} else {
            f.write_fmt(format_args!("{} ", self.function_ref.calling_convention))?;
        }
        if self.function_ref.address_space == 0 {
            f.write_fmt(format_args!("addrspace({}) ", self.function_ref.address_space))?;
        }
        f.write_fmt(format_args!("{} {}( ", FunctionType::from(self.function_ref), self.function_ref.identifier))?;
        for (i, argument) in self.arguments.iter().enumerate() {
            argument.fmt(f)?;
            if i < (self.arguments.len()-1) { f.write_str(", ")?; }
        }
        f.write_fmt(format_args!(" ) to {} unwind {}", self.normal, self.exception))?;
        Ok(())
    }
}

pub struct CallBranchInstruction<'s> {
    function_ref: &'s Function<'s>,
    arguments: Vec<Value<'s>>,
    fallthrough: Value<'s>,
    indirect: Vec<Value<'s>>,
}

impl Display for CallBranchInstruction<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("callbr ")?;
        if let crate::module::CallingConvention::C = self.function_ref.calling_convention {} else {
            f.write_fmt(format_args!("{} ", self.function_ref.calling_convention))?;
        }
        if self.function_ref.address_space == 0 {
            f.write_fmt(format_args!("addrspace({}) ", self.function_ref.address_space))?;
        }
        f.write_fmt(format_args!("{} {}( ", FunctionType::from(self.function_ref), self.function_ref.identifier))?;
        for (i, argument) in self.arguments.iter().enumerate() {
            argument.fmt(f)?;
            if i < (self.arguments.len()-1) { f.write_str(", ")?; }
        }
        f.write_fmt(format_args!(" ) to {} [ ", self.fallthrough))?;
        for (i, indirect) in self.indirect.iter().enumerate() {
            indirect.fmt(f)?;
            if i < (self.indirect.len()-1) { f.write_str(", ")?; }
        }
        f.write_str(" ]")?;
        Ok(())
    }
}

pub struct ResumeInstruction<'s>(Value<'s>);

impl Display for ResumeInstruction<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("resume {}", self.0))
    }
}

pub struct CatchSwitchInstruction<'s> {
    parent: Value<'s>,
    handlers: Vec<Value<'s>>,
    unwind: Option<Value<'s>>
}

impl Display for CatchSwitchInstruction<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("catchswitch within ")?;
        match &self.parent {
            Value::Constant(crate::constant::Constant::Simple(crate::constant::simple::Constant::Token(crate::constant::simple::TokenConstant))) => 
                crate::constant::simple::TokenConstant.fmt(f)?,
            Value::Reference(it) => it.fmt(f)?,
            _ => unreachable!()
        }
        f.write_str(" [ ")?;
        for (i, handler) in self.handlers.iter().enumerate() { 
            handler.fmt(f)?;
            if i < (self.handlers.len()-1) { f.write_str(", ")?; }
        }
        f.write_str(" ] unwind ")?;
        if let Some(unwind) = &self.unwind { unwind.fmt(f)?; } else { f.write_str("to caller")?; }
        Ok(())
    }
}

pub struct CatchReturnInstruction<'s> {
    token: Value<'s>,
    label: Value<'s>
}

impl Display for CatchReturnInstruction<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("catchret from {} to {}", self.token, self.label))
    }
}

pub struct CleanUpReturnInstruction<'s> {
    value: Value<'s>,
    label: Option<Value<'s>>
}

impl Display for CleanUpReturnInstruction<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("cleanupret from {} unwind ", self.value))?;
        if let Some(label) = &self.label { label.fmt(f)?; } else { f.write_str("to caller")?; }
        Ok(())
    }
}

pub struct UnreachableInstruction;

impl Display for UnreachableInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("unreachable")
    }
}

