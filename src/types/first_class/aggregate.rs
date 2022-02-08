use std::fmt::{Display, Debug};

use crate::IRElement;

#[derive(Debug)]
pub enum Type {
    Struct(StructType),
    OpaqueStruct(OpaqueStructType)
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let displayable: &dyn Display = match self {
            Self::Struct(_type) => _type as &dyn Display,
            Self::OpaqueStruct(_type) => _type as &dyn Display
        };
        Display::fmt(displayable, f)
    }
}

impl IRElement for Type {}

#[derive(Debug)]
pub struct StructType {
    packed: bool,
    types: Vec<crate::types::first_class::Type>
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

impl IRElement for StructType {}

#[derive(Debug)]
pub struct OpaqueStructType;

impl Display for OpaqueStructType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("type opaque")
    }
}

impl IRElement for OpaqueStructType {}

