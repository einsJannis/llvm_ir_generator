use core::fmt::{Display, Debug};

use crate::IRElement;

mod first_class;

#[derive(Debug)]
pub enum Type {
    Void,
    Function(FunctionType),
    FirstClass(first_class::Type),
}

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

impl IRElement for Type {}

#[derive(Debug)]
pub struct FunctionType { 
    return_type: first_class::Type,
    argument_types: Vec<Type>
}

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

impl IRElement for FunctionType {}

