use std::fmt::{Display, Debug};

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Integer(IntegerType),
    Float(FloatType),
    X86AMX(X86AMXType),
    X86MMX(X86MMXType),
    Pointer(PointerType),
    Vector(VectorType),
}

impl From<IntegerType> for Type {
    fn from(_type: IntegerType) -> Self {
        return Type::Integer(_type);
    }
}

impl From<FloatType> for Type {
    fn from(_type: FloatType) -> Self {
        return Type::Float(_type);
    }
}

impl From<X86AMXType> for Type {
    fn from(_type: X86AMXType) -> Self {
        return Type::X86AMX(_type);
    }
}

impl From<X86MMXType> for Type {
    fn from(_type: X86MMXType) -> Self {
        return Type::X86MMX(_type);
    }
}

impl From<PointerType> for Type {
    fn from(_type: PointerType) -> Self {
        return Type::Pointer(_type);
    }
}

impl From<VectorType> for Type {
    fn from(_type: VectorType) -> Self {
        return Type::Vector(_type);
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let displayable: &dyn Display = match self {
            Type::Integer(_type) => _type as &dyn Display,
            Type::Float(_type) => _type as &dyn Display,
            Type::X86AMX(_type) => _type as &dyn Display,
            Type::X86MMX(_type) => _type as &dyn Display,
            Type::Pointer(_type) => _type as &dyn Display,
            Type::Vector(_type) => _type as &dyn Display,
        };
        Display::fmt(displayable, f)
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(transparent)]
pub struct IntegerType(pub usize);

impl Display for IntegerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("i{}", self.0))
    }
}

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
pub struct X86AMXType;

impl Display for X86AMXType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("x86_amx")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct X86MMXType;

impl Display for X86MMXType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("x86_mmx")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PointerType {
    _type: Box<crate::types::Type>,
    address_space: usize
}

impl PointerType {
    pub fn new(_type: Box<crate::types::Type>) -> PointerType {
        PointerType { _type, address_space: 0 }
    }
    pub fn new_with_address_space(_type: Box<crate::types::Type>, address_space: usize) -> PointerType {
        PointerType { _type, address_space }
    }
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

#[derive(Debug, Clone, PartialEq)]
pub struct VectorType {
    pub factor: usize,
    pub _type: Box<crate::types::first_class::Type>,
    pub scalable: bool
}

impl VectorType {
    fn new(factor: usize, _type: Box<crate::types::first_class::Type>) -> Self {
        VectorType { factor, _type, scalable: false }
    }
    fn new_scalable(factor: usize, _type: Box<crate::types::first_class::Type>) -> Self {
        VectorType { factor, _type, scalable: true }
    }
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

