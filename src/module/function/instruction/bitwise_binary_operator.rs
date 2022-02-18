use std::fmt::Display;

use crate::reference::Value;

pub enum Instruction<'s> {
    ShiftLeft(ShiftLeftInstruction<'s>),
}

impl Display for Instruction<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (match self {
            Self::ShiftLeft(it) => it as &dyn Display,
        }).fmt(f)
    }
}

pub struct ShiftLeftInstruction<'s> {
    nuw: bool,
    nsw: bool,
    lhs: Value<'s>,
    rhs: Value<'s>
}

impl Display for ShiftLeftInstruction<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("shl ")?;
        if self.nuw { f.write_str("nuw ")?; }
        if self.nsw { f.write_str("nsw ")?; }
        f.write_fmt(format_args!("{}, {}", self.lhs, match &self.rhs {
            Value::Constant(it) => it as &dyn Display,
            Value::Reference(it) => it as &dyn Display
        }))?;
        Ok(())
    }
}

