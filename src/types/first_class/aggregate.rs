use std::fmt::{Display, Debug};

use crate::IRElement;

#[derive(Debug, Clone)]
pub enum Type {
    Array(ArrayType),
    Struct(StructType),
    OpaqueStruct(OpaqueStructType)
}

impl From<ArrayType> for Type {
    fn from(_type: ArrayType) -> Self {
        return Type::Array(_type);
    }
}

impl From<StructType> for Type {
    fn from(_type: StructType) -> Self {
        return Type::Struct(_type)
    }
}

impl From<OpaqueStructType> for Type {
    fn from(_type: OpaqueStructType) -> Self {
        return Type::OpaqueStruct(_type);
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let displayable: &dyn Display = match self {
            Self::Array(_type) => _type as &dyn Display,
            Self::Struct(_type) => _type as &dyn Display,
            Self::OpaqueStruct(_type) => _type as &dyn Display
        };
        Display::fmt(displayable, f)
    }
}

#[derive(Debug, Clone)]
pub struct ArrayType {
    size: usize,
    _type: Box<crate::types::first_class::Type>,
}

impl ArrayType {
    pub fn new(size: usize, _type: Box<crate::types::first_class::Type>) -> Self {
        ArrayType { size, _type }
    }
}

impl Display for ArrayType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("[ {} x {} ]", self.size, self._type))
    }
}

#[derive(Debug, Clone)]
pub struct StructType {
    packed: bool,
    types: Vec<crate::types::first_class::Type>
}

impl StructType {
    pub fn new(types: Vec<crate::types::first_class::Type>) -> Self {
        StructType { packed: false, types }
    }
    pub fn new_packed(types: Vec<crate::types::first_class::Type>) -> Self {
        StructType { packed: true, types }
    }
}

impl Display for StructType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("type ")?;
        if self.packed { f.write_str("<")?; }
        f.write_str("{ ")?;
        for (i, _type) in self.types.iter().enumerate() {
            Display::fmt(_type, f)?;
            if i < self.types.len() { f.write_str(", ")?; }
        }
        f.write_str(" }")?;
        if self.packed { f.write_str(">")?; }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct OpaqueStructType;

impl Display for OpaqueStructType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("type opaque")
    }
}

