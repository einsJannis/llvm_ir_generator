use std::fmt::Display;

use crate::{types::first_class::Type, reference::Value};

pub enum Instruction<'s> {
    AllocA(AllocAInstruction<'s>),
}

impl Display for Instruction<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (match self {
            Instruction::AllocA(it) => it as &dyn Display,
        }).fmt(f)
    }
}

pub struct AllocAInstruction<'s> {
    //inalloca,
    _type: Type,
    num_elements: Option<Value<'s>>,
    align: Option<usize>,
    addrspace: Option<usize>
}

impl Display for AllocAInstruction<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("alloca {}", self._type))?;
        if let Some(num_elements) = &self.num_elements { f.write_fmt(format_args!(", {}", num_elements))?; }
        if let Some(align) = self.align { f.write_fmt(format_args!(", {}", align))?; }
        if let Some(addrspace) = self.addrspace { f.write_fmt(format_args!(", {}", addrspace))?; }
        Ok(())
    }
}

pub struct LoadInstruction<'s> {

}

