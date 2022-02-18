use std::fmt::Display;

use crate::reference::Value;

pub enum Instruction<'s> {
    Add(AddInstruction<'s>),
    FloatAdd(FloatAddInstruction<'s>),
    Sub(SubInstruction<'s>),
    FloatSub(FloatSubInstruction<'s>),
    Multiply(MultiplyInstruction<'s>),
    FloatMultiply(FloatMultiplyInstruction<'s>),
    UnsignedDivision(UnsignedDivisionInstruction<'s>),
    SignedDivision(SignedDivisionInstruction<'s>),
    FloatDivision(FloatDivisionInstruction<'s>),
    UnsignedRemainder(UnsignedRemainderInstruction<'s>),
    SignedRemainder(SignedRemainderInstruction<'s>),
    FloatRemainder(FloatRemainderInstruction<'s>)
}

impl Display for Instruction<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (match self {
            Self::Add(it) => it as &dyn Display,
            Self::FloatAdd(it) => it as &dyn Display,
            Self::Sub(it) => it as &dyn Display,
            Self::FloatSub(it) => it as &dyn Display,
            Self::Multiply(it) => it as &dyn Display,
            Self::FloatMultiply(it) => it as &dyn Display,
            Self::UnsignedDivision(it) => it as &dyn Display,
            Self::SignedDivision(it) => it as &dyn Display,
            Self::FloatDivision(it) => it as &dyn Display,
            Self::UnsignedRemainder(it) => it as &dyn Display,
            Self::SignedRemainder(it) => it as &dyn Display,
            Self::FloatRemainder(it) => it as &dyn Display,
        }).fmt(f)
    }
}

pub struct AddInstruction<'s> {
    nuw: bool,
    nsw: bool,
    lhs: Value<'s>,
    rhs: Value<'s>,
}

impl Display for AddInstruction<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("add ")?;
        if self.nuw { f.write_str("nuw ")?; }
        if self.nsw { f.write_str("nsw ")?; }
        f.write_fmt(format_args!("{}, {}", self.lhs, match &self.rhs {
            Value::Constant(it) => it as &dyn Display,
            Value::Reference(it) => it as &dyn Display
        }))?;
        Ok(())
    }
}

pub struct FloatAddInstruction<'s> {
    //fast_math_flags: Vec<FastMathFlag>,
    lhs: Value<'s>,
    rhs: Value<'s>
}

impl Display for FloatAddInstruction<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("fadd {}, {}", self.lhs, match &self.rhs {
            Value::Constant(it) => it as &dyn Display,
            Value::Reference(it) => it as &dyn Display
        }))
    }
}

pub struct SubInstruction<'s> {
    nuw: bool,
    nsw: bool,
    lhs: Value<'s>,
    rhs: Value<'s>,
}

impl Display for SubInstruction<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("sub ")?;
        if self.nuw { f.write_str("nuw ")?; }
        if self.nsw { f.write_str("nsw ")?; }
        f.write_fmt(format_args!("{}, {}", self.lhs, match &self.rhs {
            Value::Constant(it) => it as &dyn Display,
            Value::Reference(it) => it as &dyn Display
        }))?;
        Ok(())
    }
}

pub struct FloatSubInstruction<'s> {
    //fast_math_flags: Vec<FastMathFlag>,
    lhs: Value<'s>,
    rhs: Value<'s>
}

impl Display for FloatSubInstruction<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("fsub {}, {}", self.lhs, match &self.rhs {
            Value::Constant(it) => it as &dyn Display,
            Value::Reference(it) => it as &dyn Display
        }))
    }
}

pub struct MultiplyInstruction<'s> {
    nuw: bool,
    nsw: bool,
    lhs: Value<'s>,
    rhs: Value<'s>,
}

impl Display for MultiplyInstruction<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("mul ")?;
        if self.nuw { f.write_str("nuw ")?; }
        if self.nsw { f.write_str("nsw ")?; }
        f.write_fmt(format_args!("{}, {}", self.lhs, match &self.rhs {
            Value::Constant(it) => it as &dyn Display,
            Value::Reference(it) => it as &dyn Display
        }))?;
        Ok(())
    }
}

pub struct FloatMultiplyInstruction<'s> {
    //fast_math_flags: Vec<FastMathFlag>,
    lhs: Value<'s>,
    rhs: Value<'s>,
}

impl Display for FloatMultiplyInstruction<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("fmul {}, {}", self.lhs, match &self.rhs {
            Value::Constant(it) => it as &dyn Display,
            Value::Reference(it) => it as &dyn Display
        }))
    }
}

pub struct UnsignedDivisionInstruction<'s> {
    exact: bool,
    lhs: Value<'s>,
    rhs: Value<'s>,
}

impl Display for UnsignedDivisionInstruction<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("udiv ")?;
        if self.exact { f.write_str("exact ")?; }
        f.write_fmt(format_args!("{}, {}", self.lhs, match &self.rhs {
            Value::Constant(it) => it as &dyn Display,
            Value::Reference(it) => it as &dyn Display
        }))?;
        Ok(())
    }
}

pub struct SignedDivisionInstruction<'s> {
    exact: bool,
    lhs: Value<'s>,
    rhs: Value<'s>,
}

impl Display for SignedDivisionInstruction<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("sdiv ")?;
        if self.exact { f.write_str("exact ")?; }
        f.write_fmt(format_args!("{}, {}", self.lhs, match &self.rhs {
            Value::Constant(it) => it as &dyn Display,
            Value::Reference(it) => it as &dyn Display
        }))?;
        Ok(())
    }
}

pub struct FloatDivisionInstruction<'s> {
    //fast_math_flags: Vec<FastMathFlag>,
    lhs: Value<'s>,
    rhs: Value<'s>,
}

impl Display for FloatDivisionInstruction<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}, {}", self.lhs, match &self.rhs {
            Value::Constant(it) => it as &dyn Display,
            Value::Reference(it) => it as &dyn Display
        }))
    }
}

pub struct UnsignedRemainderInstruction<'s> {
    lhs: Value<'s>,
    rhs: Value<'s>,
}

impl Display for UnsignedRemainderInstruction<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("urem {}, {}", self.lhs, match &self.rhs {
            Value::Constant(it) => it as &dyn Display,
            Value::Reference(it) => it as &dyn Display
        }))
    }
}

pub struct SignedRemainderInstruction<'s> {
    lhs: Value<'s>,
    rhs: Value<'s>,
}

impl Display for SignedRemainderInstruction<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("srem {}, {}", self.lhs, match &self.rhs {
            Value::Constant(it) => it as &dyn Display,
            Value::Reference(it) => it as &dyn Display
        }))
    }
}

pub struct FloatRemainderInstruction<'s> {
    //fast_math_flags: Vec<FastMathFlag>,
    lhs: Value<'s>,
    rhs: Value<'s>,
}

impl Display for FloatRemainderInstruction<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("frem {}, {}", self.lhs, match &self.rhs {
            Value::Constant(it) => it as &dyn Display,
            Value::Reference(it) => it as &dyn Display
        }))
    }
}

