use std::fmt::Display;

use crate::reference::Value;

pub enum Instruction<'s> {
    ExtractElement(ExtractElementInstruction<'s>),
}

impl Display for Instruction<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (match self {
            Self::ExtractElement(it) => it as &dyn Display,
        }).fmt(f)
    }
}

pub struct ExtractElementInstruction<'s> {
    vector: Value<'s>,
    index: Value<'s>
}

impl Display for ExtractElementInstruction<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("extractelement {}, {}", self.vector, self.index))
    }
}

pub struct InsertElementInstruction<'s> {
    vector: Value<'s>,
    element: Value<'s>,
    index: Value<'s>
}

impl Display for InsertElementInstruction<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("insertelement {}, {}, {}", self.vector, self.element, self.index))
    }
}

pub struct ShuffleVectorInstruction<'s> {
    vector1: Value<'s>,
    vector2: Value<'s>,
    mask: Value<'s>
}

impl Display for ShuffleVectorInstruction<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("shufflevector {}, {}, {}", self.vector1, self.vector2, self.mask))
    }
}

