use std::fmt::{Display, Debug};

use crate::IRElement;

#[derive(Debug)]
pub enum Type {
    Integer(IntegerType),
    Float(FloatType),
    X86MMX(X86MMXType),
    Pointer(PointerType),
    Vector(VectorType),
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let displayable: &dyn Display = match self {
            Type::Integer(_type) => _type as &dyn Display,
            Type::Float(_type) => _type as &dyn Display,
            Type::X86MMX(_type) => _type as &dyn Display,
            Type::Pointer(_type) => _type as &dyn Display,
            Type::Vector(_type) => _type as &dyn Display,
        };
        Display::fmt(displayable, f)
    }
}

impl IRElement for Type {}

#[derive(Debug)]
#[repr(transparent)]
pub struct IntegerType(usize);

impl Display for IntegerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("i{}", self.0))
    }
}

impl IRElement for IntegerType {}

#[derive(Debug)]
pub enum FloatType {
    Half,
    BFloat,
    Float,
    Double,
    FP128,
    X86FP80,
    PPCFP128
}

impl Display for FloatType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Half => "half",
            Self::BFloat => "bfloat",
            Self::Float => "float",
            Self::Double => "double",
            Self::FP128 => "fp128",
            Self::X86FP80 => "x86_fp80",
            Self::PPCFP128 => "ppc_fp128"
        })
    }
}

impl IRElement for FloatType {}

#[derive(Debug)]
pub struct X86AMXType;

impl Display for X86AMXType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("x86_amx")
    }
}

impl IRElement for X86AMXType {}

#[derive(Debug)]
pub struct X86MMXType;

impl Display for X86MMXType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("x86_mmx")
    }
}

impl IRElement for X86MMXType {}

#[derive(Debug)]
pub struct PointerType {
    _type: Box<crate::types::Type>,
    address_space: usize
}

impl Display for PointerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let crate::types::Type::FirstClass(crate::types::first_class::Type::Aggregate(crate::types::first_class::aggregate::Type::OpaqueStruct(_))) = self._type.as_ref() {
            if self.address_space == 0 {
                f.write_str("ptr")
            } else {
                f.write_fmt(format_args!("ptr addrspace({})", self.address_space))
            }
        } else {
            if self.address_space == 0 {
                f.write_fmt(format_args!("{}*", self._type))
            } else {
                f.write_fmt(format_args!("{} addrspace({})*", self._type, self.address_space))
            }
        }
    }
}

impl IRElement for PointerType {}

#[derive(Debug)]
pub struct VectorType {
    factor: usize,
    _type: Box<crate::types::first_class::Type>,
    scalable: bool
}

impl Display for VectorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.scalable {
            f.write_fmt(format_args!("<vscale x {} x {}>", self.factor, self._type))
        } else {
            f.write_fmt(format_args!("<{} x {}>", self.factor, self._type))
        }
    }
}

impl IRElement for VectorType {}

