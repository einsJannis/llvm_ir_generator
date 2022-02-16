use std::fmt::Display;

use crate::reference::Value;

pub enum Instruction<'s> {
    FloatNegation(FloatNegationInstruction<'s>),
}

impl Display for Instruction<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (match self {
            Self::FloatNegation(it) => it as &dyn Display,
        }).fmt(f)
    }
}

pub enum ReturningInstruction<'s> {
    FloatNegation(FloatNegationInstruction<'s>),
}

impl Display for ReturningInstruction<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (match self {
            Self::FloatNegation(it) => it as &dyn Display,
        }).fmt(f)
    }
}

pub struct FloatNegationInstruction<'s> {
    //fast_math_flags: Vec<FastMathFlag>,
    value: Value<'s>
}

impl Display for FloatNegationInstruction<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("fneg {}", self.value))
    }
}

