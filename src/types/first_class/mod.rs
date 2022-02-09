use std::fmt::Display;

use crate::IRElement;

pub mod single_value;
pub mod aggregate;

#[derive(Debug)]
pub enum Type {
    SingleValue(single_value::Type),
    Label(LabelType),
    Token(TokenType),
    Metadata(MetadataType),
    Aggregate(aggregate::Type)
}

impl From<single_value::Type> for Type {
    fn from(_type: single_value::Type) -> Self {
        return Type::SingleValue(_type);
    }
}

impl From<single_value::IntegerType> for Type {
    fn from(_type: single_value::IntegerType) -> Self {
        return Type::from(single_value::Type::Integer(_type));
    }
}

impl From<single_value::FloatType> for Type {
    fn from(_type: single_value::FloatType) -> Self {
        return Type::from(single_value::Type::Float(_type));
    }
}

impl From<single_value::X86AMXType> for Type {
    fn from(_type: single_value::X86AMXType) -> Self {
        return Type::from(single_value::Type::X86AMX(_type));
    }
}

impl From<single_value::X86MMXType> for Type {
    fn from(_type: single_value::X86MMXType) -> Self {
        return Type::from(single_value::Type::X86MMX(_type));
    }
}

impl From<single_value::PointerType> for Type {
    fn from(_type: single_value::PointerType) -> Self {
        return Type::from(single_value::Type::Pointer(_type));
    }
}

impl From<single_value::VectorType> for Type {
    fn from(_type: single_value::VectorType) -> Self {
        return Type::from(single_value::Type::Vector(_type));
    }
}

impl From<LabelType> for Type {
    fn from(_type: LabelType) -> Self {
        return Type::Label(_type);
    }
}

impl From<TokenType> for Type {
    fn from(_type: TokenType) -> Self {
        return Type::Token(_type);
    }
}

impl From<MetadataType> for Type {
    fn from(_type: MetadataType) -> Self {
        return Type::Metadata(_type);
    }
}

impl From<aggregate::Type> for Type {
    fn from(_type: aggregate::Type) -> Self {
        return Type::Aggregate(_type);
    }
}

impl From<aggregate::ArrayType> for Type {
    fn from(_type: aggregate::ArrayType) -> Self {
        return Type::from(aggregate::Type::Array(_type));
    }
}

impl From<aggregate::StructType> for Type {
    fn from(_type: aggregate::StructType) -> Self {
        return Type::from(aggregate::Type::Struct(_type));
    }
}

impl From<aggregate::OpaqueStructType> for Type {
    fn from(_type: aggregate::OpaqueStructType) -> Self {
        return Type::from(aggregate::Type::OpaqueStruct(_type));
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let displayable: &dyn Display = match self {
            Self::SingleValue(_type) => _type as &dyn Display,
            Self::Label(_type) => _type as &dyn Display,
            Self::Token(_type) => _type as &dyn Display,
            Self::Metadata(_type) => _type as &dyn Display,
            Self::Aggregate(_type) => _type as &dyn Display,
        };
        Display::fmt(displayable, f)
    }
}

impl IRElement for Type {}

#[derive(Debug)]
pub struct LabelType;

impl Display for LabelType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("label")
    }
}

impl IRElement for LabelType {}

#[derive(Debug)]
pub struct TokenType;

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("token")
    }
}

impl IRElement for TokenType {}

#[derive(Debug)]
pub struct MetadataType;

impl Display for MetadataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("metadata")
    }
}

impl IRElement for MetadataType {}

