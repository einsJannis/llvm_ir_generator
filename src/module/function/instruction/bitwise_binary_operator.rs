use std::fmt::Display;

use crate::reference::Value;

pub enum Instruction<'s> {
    ShiftLeft(ShiftLeftInstruction<'s>),
    LogicalShiftLeft(LogicalShiftLeftInstruction<'s>),
    ArithmeticShiftLeft(ArithmeticShiftLeftInstruction<'s>),
    And(AndInstruction<'s>),
    Or(OrInstruction<'s>),
    XOr(XOrInstruction<'s>)
}

impl Display for Instruction<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (match self {
            Self::ShiftLeft(it) => it as &dyn Display,
            Self::LogicalShiftLeft(it) => it as &dyn Display,
            Self::ArithmeticShiftLeft(it) => it as &dyn Display,
            Self::And(it) => it as &dyn Display,
            Self::Or(it) => it as &dyn Display,
            Self::XOr(it) => it as &dyn Display
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

pub struct LogicalShiftLeftInstruction<'s> {
    exact: bool,
    lhs: Value<'s>,
    rhs: Value<'s>
}

impl Display for LogicalShiftLeftInstruction<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("lshr ")?;
        if self.exact { f.write_str("exact ")?; }
        f.write_fmt(format_args!("{}, {}", self.lhs, match &self.rhs {
            Value::Constant(it) => it as &dyn Display,
            Value::Reference(it) => it as &dyn Display
        }))?;
        Ok(())
    }
}

pub struct ArithmeticShiftLeftInstruction<'s> {
    exact: bool,
    lhs: Value<'s>,
    rhs: Value<'s>
}

impl Display for ArithmeticShiftLeftInstruction<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ashr ")?;
        if self.exact { f.write_str("exact ")?; }
        f.write_fmt(format_args!("{}, {}", self.lhs, match &self.rhs {
            Value::Constant(it) => it as &dyn Display,
            Value::Reference(it) => it as &dyn Display
        }))?;
        Ok(())
    }
}

pub struct AndInstruction<'s> {
    lhs: Value<'s>,
    rhs: Value<'s>
}

impl Display for AndInstruction<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("and {}, {}", self.lhs, match &self.rhs {
            Value::Constant(it) => it as &dyn Display,
            Value::Reference(it) => it as &dyn Display
        }))
    }
}

pub struct OrInstruction<'s> {
    lhs: Value<'s>,
    rhs: Value<'s>
}

impl Display for OrInstruction<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("or {}, {}", self.lhs, match &self.rhs {
            Value::Constant(it) => it as &dyn Display,
            Value::Reference(it) => it as &dyn Display
        }))
    }
}

pub struct XOrInstruction<'s> {
    lhs: Value<'s>,
    rhs: Value<'s>
}

impl Display for XOrInstruction<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("xor {}, {}", self.lhs, match &self.rhs {
            Value::Constant(it) => it as &dyn Display,
            Value::Reference(it) => it as &dyn Display
        }))
    }
}

