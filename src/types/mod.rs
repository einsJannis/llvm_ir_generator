use core::fmt::{Display, Debug};

use crate::IRElement;
use crate::module::function::Function;

pub mod first_class;

pub trait ReturnType {
    fn return_type(&self) -> first_class::Type;
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Void,
    Function(FunctionType),
    FirstClass(first_class::Type),
}

impl From<()> for Type {
    fn from(_: ()) -> Self {
        return Type::Void;
    }
}

impl From<FunctionType> for Type {
    fn from(_type: FunctionType) -> Self {
        return Type::Function(_type);
    }
}

impl From<first_class::Type> for Type {
    fn from(_type: first_class::Type) -> Self {
        return Type::FirstClass(_type);
    }
}

impl From<first_class::single_value::Type> for Type {
    fn from(_type: first_class::single_value::Type) -> Self {
        return Type::from(first_class::Type::SingleValue(_type));
    }
}

impl From<first_class::single_value::IntegerType> for Type {
    fn from(_type: first_class::single_value::IntegerType) -> Self {
        return Type::from(first_class::single_value::Type::Integer(_type));
    }
}

impl From<first_class::single_value::FloatType> for Type {
    fn from(_type: first_class::single_value::FloatType) -> Self {
        return Type::from(first_class::single_value::Type::Float(_type));
    }
}

impl From<first_class::single_value::X86AMXType> for Type {
    fn from(_type: first_class::single_value::X86AMXType) -> Self {
        return Type::from(first_class::single_value::Type::X86AMX(_type));
    }
}

impl From<first_class::single_value::X86MMXType> for Type {
    fn from(_type: first_class::single_value::X86MMXType) -> Self {
        return Type::from(first_class::single_value::Type::X86MMX(_type));
    }
}

impl From<first_class::single_value::PointerType> for Type {
    fn from(_type: first_class::single_value::PointerType) -> Self {
        return Type::from(first_class::single_value::Type::Pointer(_type));
    }
}

impl From<first_class::single_value::VectorType> for Type {
    fn from(_type: first_class::single_value::VectorType) -> Self {
        return Type::from(first_class::single_value::Type::Vector(_type));
    }
}

impl From<first_class::LabelType> for Type {
    fn from(_type: first_class::LabelType) -> Self {
        return Type::from(first_class::Type::Label(_type));
    }
}

impl From<first_class::TokenType> for Type {
    fn from(_type: first_class::TokenType) -> Self {
        return Type::from(first_class::Type::Token(_type));
    }
}

impl From<first_class::MetadataType> for Type {
    fn from(_type: first_class::MetadataType) -> Self {
        return Type::from(first_class::Type::Metadata(_type));
    }
}

impl From<first_class::aggregate::Type> for Type {
    fn from(_type: first_class::aggregate::Type) -> Self {
        return Type::from(first_class::Type::Aggregate(_type));
    }
}

impl From<first_class::aggregate::ArrayType> for Type {
    fn from(_type: first_class::aggregate::ArrayType) -> Self {
        return Type::from(first_class::aggregate::Type::Array(_type));
    }
}

impl From<first_class::aggregate::StructType> for Type {
    fn from(_type: first_class::aggregate::StructType) -> Self {
        return Type::from(first_class::aggregate::Type::Struct(_type));
    }
}

impl From<first_class::aggregate::OpaqueStructType> for Type {
    fn from(_type: first_class::aggregate::OpaqueStructType) -> Self {
        return Type::from(first_class::aggregate::Type::OpaqueStruct(_type));
    }
}

impl IRElement for Type {}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let displayable: &dyn Display = match self {
            Type::Void => &"void" as &dyn Display,
            Type::Function(function) => function as &dyn Display,
            Type::FirstClass(_type) => _type as &dyn Display,
        };
        Display::fmt(displayable, f)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionType { 
    return_type: first_class::Type,
    argument_types: Vec<first_class::Type>
}

impl FunctionType {
    pub fn new(return_type: first_class::Type, argument_types: Vec<first_class::Type>) -> Self {
        FunctionType { return_type, argument_types }
    }
}

impl IRElement for FunctionType {}

impl Display for FunctionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.return_type, f)?;
        f.write_str(" ( ")?;
        for (i, argument_type) in self.argument_types.iter().enumerate() {
            Display::fmt(argument_type, f)?;
            if i < (self.argument_types.len() - 1) { f.write_str(", ")?; }
        }
        f.write_str(" )")?;
        Ok(())
    }
}

impl From<&Function<'_>> for FunctionType {
    fn from(function: &Function) -> Self {
        FunctionType { return_type: function.return_type.clone(), argument_types: function.arguments.iter().map(|it| it.return_type.clone()).collect() }
    }
}

