use std::fmt::Display;

use crate::reference::Value;

pub enum Instruction<'s> {
    ExtractValue(ExtractValueInstruction<'s>),
    InsertValue(InsertValueInstruction<'s>)
}

impl Display for Instruction<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (match self {
            Self::ExtractValue(it) => it as &dyn Display,
            Self::InsertValue(it) => it as &dyn Display
        }).fmt(f)
    }
}

pub struct ExtractValueInstruction<'s> {
    parent: Value<'s>,
    indecies: Vec<Value<'s>>
}

impl Display for ExtractValueInstruction<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("extractvalue {}", self.parent))?;
        for index in &self.indecies {
            f.write_fmt(format_args!(", {}", match &index {
                Value::Constant(it) => it as &dyn Display,
                Value::Reference(it) => it as &dyn Display
            }))?;
        }
        Ok(())
    }
}

pub struct InsertValueInstruction<'s> {
    parent: Value<'s>,
    element: Value<'s>,
    indecies: Vec<Value<'s>>
}

impl Display for InsertValueInstruction<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("extractvalue {}, {}", self.parent, self.element))?;
        for index in &self.indecies {
            f.write_fmt(format_args!(", {}", match &index {
                Value::Constant(it) => it as &dyn Display,
                Value::Reference(it) => it as &dyn Display
            }))?;
        }
        Ok(())
    }
}

