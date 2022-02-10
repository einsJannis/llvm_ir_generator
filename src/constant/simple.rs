use core::fmt::Display;

use crate::types::first_class::Type;
use crate::types::ReturnType;

#[derive(Debug)]
pub enum Constant<'s> {
    Boolean(BooleanConstant),
    Integer(IntegerConstant<'s>),
    Float(FloatConstant<'s>),
    NullPointer(NullPointerConstant),
    Token(TokenConstant),
}

impl<'s> From<BooleanConstant> for Constant<'s> {
    fn from(constant: BooleanConstant) -> Self {
        Constant::Boolean(constant)
    }
}

impl<'s> From<IntegerConstant<'s>> for Constant<'s> {
    fn from(constant: IntegerConstant<'s>) -> Self {
        Constant::Integer(constant)
    }
}

impl<'s> From<FloatConstant<'s>> for Constant<'s> {
    fn from(constant: FloatConstant<'s>) -> Self {
        Constant::Float(constant)
    }
}

impl<'s> From<NullPointerConstant> for Constant<'s> {
    fn from(constant: NullPointerConstant) -> Self {
        Constant::NullPointer(constant)
    }
}

impl<'s> From<TokenConstant> for Constant<'s> {
    fn from(constant: TokenConstant) -> Self {
        Constant::Token(constant)
    }
}

impl<'s> Display for Constant<'s> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (match self {
            Self::Boolean(it) => it as &dyn Display,
            Self::Integer(it) => it as &dyn Display,
            Self::Float(it) => it as &dyn Display,
            Self::NullPointer(it) => it as &dyn Display,
            Self::Token(it) => it as &dyn Display
        }).fmt(f)
    }
}

impl<'s> ReturnType for Constant<'s> {
    fn return_type(&self) -> Type {
        (match self {
            Constant::Boolean(it) => it as &dyn ReturnType,
            Constant::Integer(it) => it as &dyn ReturnType,
            Constant::Float(it) => it as &dyn ReturnType,
            Constant::NullPointer(it) => it as &dyn ReturnType,
            Constant::Token(it) => it as &dyn ReturnType
        }).return_type()
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct BooleanConstant(bool);

impl From<bool> for BooleanConstant {
    fn from(boolean: bool) -> Self {
        BooleanConstant(boolean)
    }
}

impl Display for BooleanConstant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0 { f.write_str("true") } else { f.write_str("false") }
    }
}

impl ReturnType for BooleanConstant {
    fn return_type(&self) -> Type {
        crate::types::first_class::single_value::IntegerType(1).into()
    }
}

#[derive(Debug)]
pub struct IntegerConstant<'s>(&'s str, crate::types::first_class::single_value::IntegerType);

impl<'s> IntegerConstant<'s> {
    fn new(_type: crate::types::first_class::single_value::IntegerType, value: &'s str) -> IntegerConstant<'s> {
        IntegerConstant(value, _type)
    }
}

impl<'s> Display for IntegerConstant<'s> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0)
    }
}

impl<'s> ReturnType for IntegerConstant<'s> {
    fn return_type(&self) -> Type { self.1.clone().into() }
}

#[derive(Debug)]
pub struct FloatConstant<'s>(&'s str, crate::types::first_class::single_value::FloatType);

impl<'s> FloatConstant<'s> {
    fn new(_type: crate::types::first_class::single_value::FloatType, value: &'s str) -> FloatConstant {
        FloatConstant(value, _type)
    }
}

impl<'s> Display for FloatConstant<'s> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0)
    }
}

impl<'s> ReturnType for FloatConstant<'s> {
    fn return_type(&self) -> Type { self.1.clone().into() }
}

#[derive(Debug)]
pub struct NullPointerConstant(crate::types::first_class::single_value::PointerType);

impl NullPointerConstant {
    fn new(_type: crate::types::first_class::single_value::PointerType) -> NullPointerConstant {
        NullPointerConstant(_type)
    }
}

impl Display for NullPointerConstant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("null")
    }
}

impl ReturnType for NullPointerConstant {
    fn return_type(&self) -> Type { self.0.clone().into() }
}

#[derive(Debug)]
pub struct TokenConstant;

impl Display for TokenConstant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("none")
    }
}

impl ReturnType for TokenConstant {
    fn return_type(&self) -> Type { 
        crate::types::first_class::TokenType.into()
    }
}

