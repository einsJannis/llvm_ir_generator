use core::fmt::Display;

use crate::types::first_class::Type;
use crate::types::ReturnType;

pub mod simple;
pub mod complex;

#[derive(Debug)]
pub enum Constant<'s> {
    Simple(simple::Constant<'s>),
    Complex(complex::Constant<'s>),
}

impl<'s> From<simple::Constant<'s>> for Constant<'s> {
    fn from(constant: simple::Constant<'s>) -> Self {
        Constant::Simple(constant)
    }
}

impl<'s> From<simple::BooleanConstant> for Constant<'s> {
    fn from(constant: simple::BooleanConstant) -> Self {
        Constant::from(simple::Constant::Boolean(constant))
    }
}

impl<'s> From<simple::IntegerConstant<'s>> for Constant<'s> {
    fn from(constant: simple::IntegerConstant<'s>) -> Self {
        Constant::from(simple::Constant::Integer(constant))
    }
}

impl<'s> From<simple::FloatConstant<'s>> for Constant<'s> {
    fn from(constant: simple::FloatConstant<'s>) -> Self {
        Constant::from(simple::Constant::Float(constant))
    }
}

impl<'s> From<simple::NullPointerConstant> for Constant<'s> {
    fn from(constant: simple::NullPointerConstant) -> Self {
        Constant::from(simple::Constant::NullPointer(constant))
    }
}

impl<'s> From<simple::TokenConstant> for Constant<'s> {
    fn from(constant: simple::TokenConstant) -> Self {
        Constant::from(simple::Constant::Token(constant))
    }
}

impl<'s> From<complex::Constant<'s>> for Constant<'s> {
    fn from(constant: complex::Constant<'s>) -> Self {
        Constant::Complex(constant)
    }
}

impl<'s> From<complex::StructConstant<'s>> for Constant<'s> {
    fn from(constant: complex::StructConstant<'s>) -> Self {
        Constant::from(complex::Constant::Struct(constant))
    }
}

impl<'s> From<complex::ArrayConstant<'s>> for Constant<'s> {
    fn from(constant: complex::ArrayConstant<'s>) -> Self {
        Constant::from(complex::Constant::Array(constant))
    }
}

impl<'s> From<complex::VectorConstant<'s>> for Constant<'s> {
    fn from(constant: complex::VectorConstant<'s>) -> Self {
        Constant::from(complex::Constant::Vector(constant))
    }
}

impl<'s> From<complex::ZeroInitializionConstant> for Constant<'s> {
    fn from(constant: complex::ZeroInitializionConstant) -> Self {
        Constant::from(complex::Constant::ZeroInitializion(constant))
    }
}

impl<'s> Display for Constant<'s> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (match self {
            Constant::Simple(it) => it as &dyn Display,
            Constant::Complex(it) => it as &dyn Display
        }).fmt(f)
    }
}

impl<'s> ReturnType for Constant<'s> {
    fn return_type(&self) -> Type {
        (match self {
            Constant::Simple(it) => it as &dyn ReturnType,
            Constant::Complex(it) => it as &dyn ReturnType
        }).return_type()
    }
}

