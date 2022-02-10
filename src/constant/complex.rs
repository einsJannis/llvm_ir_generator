use core::fmt::Display;

use crate::types::first_class::Type;
use crate::types::ReturnType;

#[derive(Debug)]
pub enum Constant<'s> {
    Struct(StructConstant<'s>),
    Array(ArrayConstant<'s>),
    Vector(VectorConstant<'s>),
    ZeroInitializion(ZeroInitializionConstant),
    //MetadataNode(MetadataNodeConstant)
}

impl<'s> From<StructConstant<'s>> for Constant<'s> {
    fn from(constant: StructConstant<'s>) -> Self {
        Constant::Struct(constant)
    }
}

impl<'s> From<ArrayConstant<'s>> for Constant<'s> {
    fn from(constant: ArrayConstant<'s>) -> Self {
        Constant::Array(constant)
    }
}

impl<'s> From<VectorConstant<'s>> for Constant<'s> {
    fn from(constant: VectorConstant<'s>) -> Self {
        Constant::Vector(constant)
    }
}

impl<'s> From<ZeroInitializionConstant> for Constant<'s> {
    fn from(constant: ZeroInitializionConstant) -> Self {
        Constant::ZeroInitializion(constant)
    }
}

impl<'s> Display for Constant<'s> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (match self {
            Self::Struct(it) => it as &dyn Display,
            Self::Array(it) => it as &dyn Display,
            Self::Vector(it) => it as &dyn Display,
            Self::ZeroInitializion(it) => it as &dyn Display
        }).fmt(f)
    }
}

impl<'s> ReturnType for Constant<'s> {
    fn return_type(&self) -> Type {
        (match self {
            Constant::Struct(it) => it as &dyn ReturnType,
            Constant::Array(it) => it as &dyn ReturnType,
            Constant::Vector(it) => it as &dyn ReturnType,
            Constant::ZeroInitializion(it) => it as &dyn ReturnType,
            //Constant::MetadataNode(it) => it as &dyn ReturnType
        }).return_type()
    }
}

pub enum ComplexConstantCreationError {
    IllegalReturnType
}

#[derive(Debug)]
pub struct StructConstant<'s>(Vec<(Type, crate::constant::Constant<'s>)>, bool);

impl<'s> StructConstant<'s> {
    fn new(fields: Vec<(Type, crate::constant::Constant<'s>)>, packed: bool) -> Result<Self, ComplexConstantCreationError> {
        for (_type, constant) in &fields {
            if _type != &constant.return_type() { return Err(ComplexConstantCreationError::IllegalReturnType); }
        }
        Ok(StructConstant(fields, packed))
    }
}

impl<'s> Display for StructConstant<'s> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("{ ")?;
        for (i, (_type, constant)) in self.0.iter().enumerate() {
            f.write_fmt(format_args!("{} {}", _type, constant))?;
            if i < (self.0.len()) { f.write_str(", ")?; }
        }
        f.write_str(" }")?;
        Ok(())
    }
}

impl<'s> ReturnType for StructConstant<'s> {
    fn return_type(&self) -> Type {
        let elements = self.0.iter().map(|it| it.0.clone()).collect();
        Type::from(if self.1 {
            crate::types::first_class::aggregate::StructType::new_packed(elements)
        } else {
            crate::types::first_class::aggregate::StructType::new(elements)
        })
    }
}

#[derive(Debug)]
pub struct ArrayConstant<'s>(Type, Vec<crate::constant::Constant<'s>>);

impl<'s> ArrayConstant<'s> {
    fn new(_type: Type, constants: Vec<crate::constant::Constant<'s>>) -> Result<Self, ComplexConstantCreationError> {
        for constant in &constants {
            if &_type != &constant.return_type() { return Err(ComplexConstantCreationError::IllegalReturnType); }
        }
        Ok(ArrayConstant(_type, constants))
    }
}

impl<'s> Display for ArrayConstant<'s> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("[ ")?;
        for (i, constant) in self.1.iter().enumerate() {
            f.write_fmt(format_args!("{} {}", self.0, constant))?;
            if i < (self.1.len()-1) { f.write_str(", ")?; }
        }
        f.write_str(" ]")?;
        Ok(())
    }
}

impl<'s> ReturnType for ArrayConstant<'s> {
    fn return_type(&self) -> Type {
        crate::types::first_class::aggregate::ArrayType::new(self.1.len(), Box::new(self.0.clone())).into()
    }
}

#[derive(Debug)]
pub struct VectorConstant<'s>(crate::types::first_class::single_value::VectorType, Vec<crate::constant::Constant<'s>>);

impl<'s> VectorConstant<'s> {
    fn new(_type: crate::types::first_class::single_value::VectorType, constants: Vec<crate::constant::Constant<'s>>) -> Result<Self, ComplexConstantCreationError> {
        for constant in &constants {
            if _type._type.as_ref() != &constant.return_type() { return Err(ComplexConstantCreationError::IllegalReturnType); }
        }
        Ok(VectorConstant(_type, constants))
    }
}

impl<'s> Display for VectorConstant<'s> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("< ")?;
        for (i, constant) in self.1.iter().enumerate() {
            f.write_fmt(format_args!("{} {}", self.0._type, constant))?;
            if i < (self.1.len()-1) { f.write_str(", ")?; }
        }
        f.write_str(" >")?;
        Ok(())
    }
}

impl<'s> ReturnType for VectorConstant<'s> {
    fn return_type(&self) -> Type { self.0.clone().into() }
}

#[derive(Debug)]
pub struct ZeroInitializionConstant(Type);

impl ZeroInitializionConstant {
    fn new(_type: Type) -> Self {
        ZeroInitializionConstant(_type)
    }
}

impl Display for ZeroInitializionConstant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("zeroinitializer")
    }
}

impl ReturnType for ZeroInitializionConstant {
    fn return_type(&self) -> Type { self.0.clone() }
}

//pub struct MetadataNodeConstant<'s>()

