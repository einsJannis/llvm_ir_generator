use std::fmt::Display;

use crate::identifier::Identifiable;
use crate::types::ReturnType;

pub enum Referencable<'s> {
    FunctionArgument(crate::module::function::Argument<'s>),
    Label(crate::module::function::InstructionBlock<'s>),
    GlobalVariabel(crate::module::global_variable::GlobalVariable<'s>)
}

impl<'s> ReturnType for Referencable<'s> {
    fn return_type(&self) -> crate::types::first_class::Type {
        match self {
            Self::Label(it) => crate::types::first_class::LabelType.into(),
            _ => (match self {
                Self::FunctionArgument(it) => it as &dyn ReturnType,
                Self::GlobalVariabel(it) => it as &dyn ReturnType,
                Self::Label(_) => unreachable!()
            }).return_type()
        }
    }
}

impl<'s> Identifiable for Referencable<'s> {
    fn id(&self) -> crate::identifier::Identifier {
        (match self {
            Self::FunctionArgument(it) => it as &dyn Identifiable,
            Self::Label(it) => it as &dyn Identifiable,
            Self::GlobalVariabel(it) => it as &dyn Identifiable,
        }).id()
    }
}

pub struct Reference<'s>(&'s Referencable<'s>);

impl<'s> ReturnType for Reference<'s> {
    fn return_type(&self) -> crate::types::first_class::Type { self.return_type() }
}

impl<'s> Identifiable for Reference<'s> {
    fn id(&self) -> crate::identifier::Identifier { self.id() }
}

impl<'s> Display for Reference<'s> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.id().fmt(f)
    }
}

pub enum Value<'s> {
    Constant(crate::constant::Constant<'s>),
    Reference(Reference<'s>),
}

impl<'s> ReturnType for Value<'s> {
    fn return_type(&self) -> crate::types::first_class::Type {
        (match self {
            Self::Constant(it) => it as &dyn ReturnType,
            Self::Reference(it) => it as &dyn ReturnType
        }).return_type()
    }
}

impl<'s> Display for Value<'s> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} {}", self.return_type(), match self {
            Self::Constant(it) => it as &dyn Display,
            Self::Reference(it) => it as &dyn Display
        }))
    }
}

