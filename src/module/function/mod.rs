mod instruction;

use std::fmt::{Display, Formatter};
use std::rc::Rc;
use crate::{CallingConvention, DLLStorageClass, Element, LinkageType, RuntimePreemptionSpecifier, Value, VisibilityStyle, WithName, WithReturnType};
use crate::metadata::Metadata;
use crate::module::comdat::Comdat;
use crate::module::function::instruction::InstructionBlock;
use crate::reference::{Referencable, Reference};
use crate::types::Type;

pub struct Function {
    linkage: LinkageType,
    preemption_specifier: Option<RuntimePreemptionSpecifier>,
    visibility: Option<VisibilityStyle>,
    dll_storage_class: Option<DLLStorageClass>,
    calling_convention: Option<CallingConvention>,
    return_attribute: Option<ReturnAttribute>,
    return_type: Rc<dyn Type>,
    name: String,
    arguments: Vec<Argument>,
    unnamed_addr: bool,
    local: bool,
    addr_space: Option<usize>,
    attributes: Vec<FunctionAttribute>,
    section: Option<String>,
    comdat: Option<Rc<Comdat>>,
    align: Option<usize>,
    garbage_collector: Option<String>,
    prefix_constant: Box<dyn Value>,
    prologue_constant: Box<dyn Value>,
    personality_constant: FunctionRef,
    metadata: Vec<dyn Metadata>,
    instruction_blocks: Vec<InstructionBlock>
}

trait LocalElement: WithReturnType {
    fn function(&self) -> Rc<Function>;
}

trait LocalElementWithName: LocalElement {
    fn raw_name(&self) -> String;
}

impl<T> WithName for T where T: LocalElementWithName {
    fn name(&self) -> String {
        format!("%{}", self.return_type())
    }
}

#[derive(Clone, Debug)]
struct Argument {
    return_type: Box<dyn Type>,
    name: String,
    _function: Rc<Function>
}

impl Display for Argument {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} {}", self.return_type, self.name))
    }
}

impl Element for Argument {}

impl WithReturnType for Argument {
    fn return_type(&self) -> Box<dyn Type> {
        self.return_type.clone()
    }
}

impl LocalElement for Argument {
    fn function(&self) -> Rc<Function> {
        self._function.clone()
    }
}

impl LocalElementWithName for Argument {
    fn raw_name(&self) -> String {
        self.name.clone()
    }
}
