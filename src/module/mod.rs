use std::rc::Rc;
use crate::{Element, ElementWithName, LinkageType};

mod function;
mod global_variable;
mod comdat;

pub struct Module {
    elements: Vec<Box<dyn ModuleElement>>,
}

pub trait ModuleElement : ElementWithName {
    fn raw_name(&self) -> String;
    fn linkage_type(&self) -> LinkageType;
    fn module(&self) -> Rc<Module>;
}

impl<T> ElementWithName for T where T: ModuleElement {
    fn name(&self) -> String {
        format!("@{}", self.raw_name())
    }
}
