use std::fmt::{Debug, Display, Formatter};
use std::ops::Deref;
use std::rc::Rc;
use crate::{Element, Value, WithName};
use crate::types::{Type, WithReturnType};

pub trait Referencable : Element {
    fn reference(_: &Rc<Self>) -> Reference<Self>;
}

#[derive(Clone, Debug)]
pub struct Reference<T: WithReturnType> {
    reference: Rc<T>,
    format: Rc<fn (Rc<T>, &mut Formatter<'_>) -> std::fmt::Result>
}

impl<T: WithReturnType> Display for Reference<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.format(f)
    }
}

impl<T: WithReturnType> Reference<T> {
    fn get(self) -> Rc<T> {
        self.reference
    }
}

impl<T: WithReturnType> WithReturnType for Reference<T> {
    fn return_type(&self) -> Box<dyn Type> {
        self.reference.return_type()
    }
}

impl<T: WithReturnType> Value for Reference<T> {}

impl<T> Reference<T> {
    pub fn new(reference: Rc<T>, format: fn (s: Rc<Self>, f: &mut Formatter<'_>) -> std::fmt::Result) -> Reference<T> {
        Reference { reference, format: Rc::new(format) }
    }
    pub fn get(&self) -> Rc<T> { self.reference.clone() }
}

impl<T: WithName + WithReturnType + Element> Referencable for T {
    fn reference(s: &Rc<Self>) -> Reference<Self> {
        Reference::new(s.clone(), |s: Rc<Self>, f: &mut Formatter<'_>| s.name().fmt(f))
    }
}
