use std::fmt::Display;

use crate::{types::first_class::Type, reference::{Reference, Value}};

pub enum Instruction<'s> {
    Return(ReturnInstruction<'s>),
    //Branch(BranchInstruction<'s>),
}

impl<'s> Display for Instruction<'s> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (match self {
            Instruction::Return(it) => it as &dyn Display,
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

