use std::fmt::{Debug, Display, Formatter, Pointer};
use std::rc::Rc;
use crate::{Element, Value, WithName};
use crate::metadata::Metadata;
use crate::types::{FloatType, Type, WithReturnType};

pub trait Constant: Value {}

pub trait SimpleConstant: Constant {}

pub trait ComplexConstant: Constant {}

impl<T> Constant for T where T: SimpleConstant {}

impl<T> Constant for T where T: ComplexConstant {}

#[derive(Clone, Debug)]
struct Boolean(bool);

impl Display for Boolean {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl WithReturnType for Boolean {
    fn return_type(&self) -> Box<dyn Type> {
        crate::types::integer(1)
    }
}

impl SimpleConstant for Boolean {}

#[derive(Clone, Debug)]
struct Integer {
    value: String,
    size: usize
}

impl Display for Integer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}

impl WithReturnType for Integer {
    fn return_type(&self) -> Box<dyn Type> {
        crate::types::integer(self.size.clone())
    }
}

impl SimpleConstant for Integer {}

#[derive(Clone, Debug)]
struct Float {
    value: String,
    return_type: Box<dyn FloatType>
}

impl Display for Float {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}

impl WithReturnType for Float {
    fn return_type(&self) -> Box<dyn Type> {
        self.return_type.clone()
    }
}

impl SimpleConstant for Integer {}

#[derive(Clone, Debug)]
struct NullPointer {
    return_type: Box<dyn Type>
}

impl Display for NullPointer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("null")
    }
}

impl WithReturnType for NullPointer {
    fn return_type(&self) -> Box<dyn Type> {
        crate::types::pointer(self.return_type.clone(), None)
    }
}

impl SimpleConstant for NullPointer {}

#[derive(Clone, Debug)]
struct Token();

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("none")
    }
}

impl WithReturnType for Token {
    fn return_type(&self) -> Box<dyn Type> {
        crate::types::token()
    }
}

impl SimpleConstant for Token {}

#[derive(Clone, Debug)]
struct Structure {
    fields: Vec<dyn Value>,
    packed: bool
}

impl Display for Structure {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("{ ")?;
        for i in 0..self.fields.len() {
            let field = self.fields[i].clone();
            f.write_fmt(format_args!("{} {}", field.return_type(), field))?;
            if i < self.fields.len()-1 { f.write_str(", ")?; }
        }
        f.write_str(" }")?;
        Ok(())
    }
}

impl WithReturnType for Structure {
    fn return_type(&self) -> Box<dyn Type> {
        crate::types::structure(
            self.fields.iter().map(|it: Box<dyn Constant>| it.return_type()).collect(),
            self.packed.clone()
        )
    }
}

impl ComplexConstant for Structure {}

#[derive(Clone, Debug)]
struct Array {
    fields: Vec<dyn Value>
}

impl Display for Array {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("[ ")?;
        for i in 0..self.fields.len() {
            let field = self.fields[i].clone();
            f.write_fmt(format_args!("{} {}", field.return_type(), field))?;
            if i < self.fields.len()-1 { f.write_str(", ")?; }
        }
        f.write_str(" ]")?;
        Ok(())
    }
}

impl WithReturnType for Array {
    fn return_type(&self) -> Box<dyn Type> {
        crate::types::array(self.fields.len(), self.fields[0].return_type())
    }
}

impl ComplexConstant for Array {}

#[derive(Clone, Debug)]
struct CString {
    value: String
}

impl Display for CString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("c\"{}\"", self.value))
    }
}

impl WithReturnType for CString {
    fn return_type(&self) -> Box<dyn Type> {
        crate::types::array(self.value.len() + self.value.chars().into_iter().filter(|it| it == '\\').count(), crate::types::integer(8))
    }
}

impl ComplexConstant for CString {}

#[derive(Clone, Debug)]
struct Vector {
    fields: Vec<Box<dyn Value>>,
    block_size: usize,
    scalable: bool
}

impl Display for Vector {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("< ")?;
        for i in 0..self.fields.len() {
            let field = self.fields[i].clone();
            f.write_fmt(format_args!("{} {}", field.return_type(), field))?;
            if i < self.fields.len()-1 { f.write_str(", ")?; }
        }
        f.write_str(" >")?;
        Ok(())
    }
}

impl WithReturnType for Vector {
    fn return_type(&self) -> Box<dyn Type> {
        crate::types::vector(self.block_size.clone(), self.fields[0].return_type(), self.scalable.clone())
    }
}

impl ComplexConstant for Vector {}

#[derive(Clone, Debug)]
struct ZeroInitialized {
    return_type: Box<dyn Type>
}

impl Display for ZeroInitialized {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("zeroinitializer")
    }
}

impl WithReturnType for ZeroInitialized {
    fn return_type(&self) -> Box<dyn Type> {
        self.return_type.clone()
    }
}

impl ComplexConstant for ZeroInitialized {}

#[derive(Clone, Debug)]
struct MetadataNode {
    fields: Vec<dyn Value>
}

impl Display for MetadataNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("!{ ")?;
        for i in 0..self.fields.len() {
            let field = self.fields[i].clone();
            f.write_fmt(format_args!("{} {}", field.return_type(), field))?; //TODO: fix node references should not have type
            if i < self.fields.len()-1 { f.write_str(", ")?; }
        }
        f.write_str("}")?;
        Ok(())
    }
}

impl WithReturnType for MetadataNode {
    fn return_type(&self) -> Box<dyn Type> {
        crate::types::metadata()
    }
}

impl Metadata for MetadataNode {}

//TODO: add undef and poison

#[derive(Clone, Debug)]
struct BlockAddress {
    instruction_block: Rc<InstructionBlock>
}

impl Display for BlockAddress {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("blockaddress({}, {})". self.instruction_block.function.name(), self.instruction_block.name()))
    }
}

#[derive(Clone, Debug)]
pub struct Void();

impl Display for  {

}
