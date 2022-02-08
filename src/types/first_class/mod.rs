use std::fmt::Display;

use crate::IRElement;

mod single_value;
mod aggregate;

#[derive(Debug)]
pub enum Type {
    SingleValue(single_value::Type),
    Label(LabelType),
    Token(TokenType),
    Metadata(MetadataType),
    Aggregate(aggregate::Type)
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

