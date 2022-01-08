use std::any::Any;
use std::borrow::Borrow;
use std::fmt::{Debug, Display, Formatter, Write};
use std::iter::Map;
use std::rc::Rc;
use crate::{Element, WithName};

pub trait Type: WithName + WithReturnType {}

pub trait WithReturnType: Element {
    fn return_type(&self) -> Box<dyn Type>;
}

impl<T> Element for T where T: WithReturnType {}

pub trait SingleValueType: Type {}

pub trait FloatType: SingleValueType {}

pub trait AggregateType: Type {}

impl<T> Element for T where T: WithReturnType  {}

impl<T> WithReturnType for T where T: Type {
    fn return_type(&self) -> Box<dyn Type> {
        r#type()
    }
}

impl<T> WithName for T where T: Type {
    fn name(&self) -> String {
        format!("{}", self)
    }
}

impl<T> Type for T where T: SingleValueType {}

impl<T> SingleValueType for T where T: FloatType {}

impl<T> Type for T where T: AggregateType {}

pub fn void() -> Box<dyn Type> { Box::new(Void) }

#[derive(Clone, Debug)]
struct Void();

impl Display for Void {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str("void")
    }
}

impl Element for Void {}

impl Type for Void {}

pub fn r#type() -> Box<dyn Type> { Box::new(TypeType) }

#[derive(Clone, Debug)]
struct TypeType();

impl Display for TypeType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("type")
    }
}

impl Type for TypeType {}

pub fn function(return_type: Box<dyn Type>, argument_types: Vec<Box<dyn Type>>) -> Box<dyn Type> {
    Box::new(Function { return_type, argument_types })
}

#[derive(Clone, Debug)]
struct Function {
    return_type: Box<dyn Type>,
    argument_types: Vec<Box<dyn Type>>
}

impl Display for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.return_type.fmt(f)?;
        f.write_str(" ( ")?;
        for i in 0..self.argument_types.len() {
            self.argument_types[i].fmt(f)?;
            if i < self.argument_types.len()-1 {
                f.write_str(", ")?
            }
        }
        f.write_str(" )")?;
        Ok(())
    }
}

impl Type for Function {}

pub fn integer(n: usize) -> Box<dyn SingleValueType> { Box::new(Integer { n }) }

#[derive(Clone, Debug)]
struct Integer {
    n: usize
}

impl Display for Integer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("i{}", self.n.clone()))
    }
}

impl SingleValueType for Integer {}

pub fn half() -> Box<dyn FloatType> { Box::new(Half) }

#[derive(Clone, Debug)]
struct Half();

impl Display for Half {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("half")
    }
}

impl FloatType for Half {}

pub fn bfloat() -> Box<dyn FloatType> { Box::new(BFloat) }

#[derive(Clone, Debug)]
struct BFloat();

impl Display for BFloat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("bfloat")
    }
}

impl FloatType for BFloat {}

pub fn float() -> Box<dyn FloatType> { Box::new(Float) }

#[derive(Clone, Debug)]
struct Float();

impl Display for Float {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("float")
    }
}

impl FloatType for Float {}

pub fn double() -> Box<dyn FloatType> { Box::new(Double) }

#[derive(Clone, Debug)]
struct Double();

impl Display for Double {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("double")
    }
}

impl FloatType for Double {}

pub fn fp128() -> Box<dyn FloatType> { Box::new(FP128) }

#[derive(Clone, Debug)]
struct FP128();

impl Display for FP128 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("fp128")
    }
}

impl FloatType for FP128 {}

pub fn x86_fp80() -> Box<dyn FloatType> { Box::new(X86FP80) }

#[derive(Clone, Debug)]
struct X86FP80();

impl Display for X86FP80 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("x86_fp80")
    }
}

impl FloatType for X86FP80 {}

pub fn ppc_fp128() -> Box<dyn FloatType> { Box::new(PPCFP128) }

#[derive(Clone, Debug)]
struct PPCFP128();

impl Display for PPCFP128 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("ppc_fp128")
    }
}

impl SingleValueType for PPCFP128 {}

pub fn x86_amx() -> Box<dyn SingleValueType> { Box::new(X86AMX) }

#[derive(Clone, Debug)]
struct X86AMX();

impl Display for X86AMX {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("x86_amx")
    }
}

impl SingleValueType for X86AMX {}

pub fn x86_mmx() -> Box<dyn SingleValueType> { Box::new(X86MMX) }

#[derive(Clone, Debug)]
struct X86MMX();

impl Display for X86MMX {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("x86_mmx")
    }
}

impl SingleValueType for X86MMX {}

pub fn pointer(return_type: Box<dyn Type>, address_space: Option<usize>) -> Box<dyn Type> {
    Box::new(Pointer { return_type, address_space })
}

#[derive(Clone, Debug)]
struct Pointer {
    return_type: Box<dyn Type>,
    address_space: Option<usize>
}

impl Pointer {
    fn fmt_address_space(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(address_space) = &self.address_space {
            f.write_fmt(format_args!(" addrspace({})", address_space.to_string()))?;
        }
        Ok(())
    }
}

impl Display for Pointer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if &self.return_type.type_id() == Opaque.type_id() {
            f.write_str("ptr")?;
            self.fmt_address_space(f)?;
        } else {
            return_type.fmt(f)?;
            self.fmt_address_space(f)?;
            f.write_str(" *")?;
        }
        Ok(())
    }
}

impl SingleValueType for Pointer {}

pub fn vector(block_size: usize, return_type: Box<dyn Type>, scalable: bool) -> Box<dyn SingleValueType> {
    Box::new(Vector { block_size, return_type, scalable })
}

#[derive(Clone, Debug)]
struct Vector {
    block_size: usize,
    return_type: Box<dyn Type>,
    scalable: bool
}

impl Display for Vector {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let fmt_str = if self.scalable { "< vscale x {} x {} >" } else { "< {} x {} >" };
        f.write_fmt(format_args!(fmt_str, self.block_size, self.return_type))
    }
}

impl SingleValueType for Vector {}

pub fn label() -> Box<dyn Type> { Box::new(Label) }

#[dervie(Clone, Debug)]
struct Label();

impl Display for Label {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("label")
    }
}

impl Type for Label {}

pub fn token() -> Box<dyn Type> { Box::new(Token) }

#[derive(Clone, Debug)]
struct Token();

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("token")
    }
}

impl Type for Token {}

pub fn metadata() -> Box<dyn Type> { Box::new(Metadata) }

#[derive(Clone, Debug)]
struct Metadata();

impl Display for Metadata {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("metadata")
    }
}

impl Type for Metadata {}

pub fn array(size: usize, return_type: Box<dyn Type>) -> Box<dyn AggregateType> {
    if return_type.type_id() == Metadata.type_id() { panic!("Metadata not allowed in arrays") }
    Box::new(Array { size, return_type })
}

#[derive(Clone, Debug)]
struct Array {
    size: usize,
    return_type: Box<dyn Type>
}

impl Display for Array {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("[ {} x {} ]", self.size.to_string(), self.return_type))
    }
}

impl AggregateType for Array {}

pub fn structure(fields: Vec<Box<dyn Type>>, packed: bool) -> Box<dyn AggregateType> {
    Box::new(Structure { fields, packed })
}

#[derive(Clone, Debug)]
struct Structure {
    fields: Vec<Box<dyn Type>>,
    packed: bool
}

impl Display for Structure {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.packed { f.write_str("<")?; }
        f.write_str("{ ")?;
        for i in 0..self.fields.len() {
            self.fields[i].fmt(f)?;
            if i < self.fields.len()-1 { f.write_str(", ")?; }
        }
        f.write_str(" }")?;
        if self.packed { f.write_char('>')?; }
        Ok(())
    }
}

pub fn opaque() -> Box<dyn AggregateType> { Box::new(Opaque) }

#[derive(Clone, Debug)]
struct Opaque();

impl Display for Opaque {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("opaque")
    }
}

impl AggregateType for Opaque {}
