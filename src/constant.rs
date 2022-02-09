use core::fmt::Display;

use crate::types::first_class::Type;

pub trait ReturnType {
    fn return_type(&self) -> Type;
}

#[derive(Debug)]
pub enum Constant<'s> {
    Simple(SimpleConstant<'s>),
    Complex(ComplexConstant<'s>),
}

impl<'s> Display for Constant<'s> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (match self {
            Constant::Simple(it) => it as &dyn Display,
            Constant::Complex(it) => it as &dyn Display
        }).fmt(f)
    }
}

impl<'s> ReturnType for Constant<'s> {
    fn return_type(&self) -> Type {
        (match self {
            Constant::Simple(it) => it as &dyn ReturnType,
            Constant::Complex(it) => it as &dyn ReturnType
        }).return_type()
    }
}

#[derive(Debug)]
pub enum SimpleConstant<'s> {
    Boolean(BooleanConstant),
    Integer(IntegerConstant<'s>),
    Float(FloatConstant<'s>),
    NullPointer(NullPointerConstant),
    Token(TokenConstant),
}

impl<'s> Display for SimpleConstant<'s> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (match self {
            Self::Boolean(it) => it as &dyn Display,
            Self::Integer(it) => it as &dyn Display,
            Self::Float(it) => it as &dyn Display,
            Self::NullPointer(it) => it as &dyn Display,
            Self::Token(it) => it as &dyn Display
        }).fmt(f)
    }
}

impl<'s> ReturnType for SimpleConstant<'s> {
    fn return_type(&self) -> Type {
        (match self {
            SimpleConstant::Boolean(it) => it as &dyn ReturnType,
            SimpleConstant::Integer(it) => it as &dyn ReturnType,
            SimpleConstant::Float(it) => it as &dyn ReturnType,
            SimpleConstant::NullPointer(it) => it as &dyn ReturnType,
            SimpleConstant::Token(it) => it as &dyn ReturnType
        }).return_type()
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct BooleanConstant(bool);

impl Display for BooleanConstant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0 { f.write_str("true") } else { f.write_str("false") }
    }
}

impl ReturnType for BooleanConstant {
    fn return_type(&self) -> Type {
        crate::types::first_class::single_value::IntegerType(1).into()
    }
}

#[derive(Debug)]
pub struct IntegerConstant<'s>(&'s str, crate::types::first_class::single_value::IntegerType);

impl<'s> Display for IntegerConstant<'s> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0)
    }
}

impl<'s> ReturnType for IntegerConstant<'s> {
    fn return_type(&self) -> Type { self.1.clone().into() }
}

#[derive(Debug)]
pub struct FloatConstant<'s>(&'s str, crate::types::first_class::single_value::FloatType);

impl<'s> Display for FloatConstant<'s> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0)
    }
}

impl<'s> ReturnType for FloatConstant<'s> {
    fn return_type(&self) -> Type { self.1.clone().into() }
}

#[derive(Debug)]
pub struct NullPointerConstant(crate::types::first_class::single_value::PointerType);

impl Display for NullPointerConstant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("null")
    }
}

impl ReturnType for NullPointerConstant {
    fn return_type(&self) -> Type { self.0.clone().into() }
}

#[derive(Debug)]
pub struct TokenConstant;

impl Display for TokenConstant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("none")
    }
}

impl ReturnType for TokenConstant {
    fn return_type(&self) -> Type { 
        crate::types::first_class::TokenType.into()
    }
}

#[derive(Debug)]
pub enum ComplexConstant<'s> {
    Struct(StructConstant<'s>),
    Array(ArrayConstant<'s>),
    Vector(VectorConstant<'s>),
    ZeroInitializion(ZeroInitializionConstant),
    //MetadataNode(MetadataNodeConstant)
}

impl<'s> Display for ComplexConstant<'s> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (match self {
            Self::Struct(it) => it as &dyn Display,
            Self::Array(it) => it as &dyn Display,
            Self::Vector(it) => it as &dyn Display,
            Self::ZeroInitializion(it) => it as &dyn Display
        }).fmt(f)
    }
}

impl<'s> ReturnType for ComplexConstant<'s> {
    fn return_type(&self) -> Type {
        (match self {
            ComplexConstant::Struct(it) => it as &dyn ReturnType,
            ComplexConstant::Array(it) => it as &dyn ReturnType,
            ComplexConstant::Vector(it) => it as &dyn ReturnType,
            ComplexConstant::ZeroInitializion(it) => it as &dyn ReturnType,
            //ComplexConstant::MetadataNode(it) => it as &dyn ReturnType
        }).return_type()
    }
}

#[derive(Debug)]
pub struct StructConstant<'s>(Vec<(Type, Constant<'s>)>, bool);

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
pub struct ArrayConstant<'s>(Type, Vec<Constant<'s>>);

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
pub struct VectorConstant<'s>(crate::types::first_class::single_value::VectorType, Vec<Constant<'s>>);

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

impl Display for ZeroInitializionConstant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("zeroinitializer")
    }
}

impl ReturnType for ZeroInitializionConstant {
    fn return_type(&self) -> Type { self.0.clone() }
}

//pub struct MetadataNodeConstant<'s>()

