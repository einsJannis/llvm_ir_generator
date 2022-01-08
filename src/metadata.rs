use std::fmt::{Debug, Display, Formatter};
use std::rc::Rc;
use crate::{Element, ElementWithName, LinkageType, WithName, WithReturnType};
use crate::constant::Constant;
use crate::module::{Module, ModuleElement};
use crate::reference::{Referencable, Reference};
use crate::types::Type;

pub trait Metadata: Constant {}

#[derive(Clone, Debug)]
struct NamedMetadata {
    name: String,
    data: Box<dyn Metadata>
}

impl Display for NamedMetadata {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("!{} = {}", self.name, self.data))
    }
}

impl Element for NamedMetadata {}

impl WithReturnType for NamedMetadata {
    fn return_type(&self) -> Box<dyn Type> {
        crate::types::metadata()
    }
}

impl WithName for NamedMetadata {
    fn name(&self) -> String {
        format!("!{}", self.name)
    }
}

impl Referencable for NamedMetadata {
    fn reference(s: &Rc<Self>) -> Reference<Self> {
        Reference::new(
            s.clone(),
            |s: Rc<Reference<Self>>, f: &mut Formatter<'_>| f.write_fmt(format_args!("!{}", s.get().name))
        )
    }
}
