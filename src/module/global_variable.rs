use std::fmt::Display;

use crate::identifier::GlobalIdentifier;
use crate::module::{LinkageType, RuntimePreemptionSpecifier, VisibilityStyle, DLLStorageClass, UnnamedAddress};
use crate::IRElement;

use super::ThreadLocalStorageModel;

pub struct GlobalVariable<'s> {
    identifier: GlobalIdentifier<'s>,
    linkage: LinkageType,
    preemtion_specifier: RuntimePreemptionSpecifier,
    visibility: VisibilityStyle,
    dll_storage_class: DLLStorageClass,
    thread_local: ThreadLocalStorageModel,
    unnamed_address: Option<UnnamedAddress>,
    address_space: Option<usize>,
    externally_initialized: bool,
    global_variable_type: GlobalVariableType,
    //return_type: Type,
    //initializer_constant: Constant,
    //section_name: String,
    //partition_name: String,
    //comdats: Vec<Comdat>,
    //allign: Alignment,
    //metadata: Vec<Metadata>
}

#[derive(Debug)]
#[repr(C)]
pub enum GlobalVariableType {
    Global,
    Constant
}

impl Display for GlobalVariableType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            GlobalVariableType::Global => "global",
            GlobalVariableType::Constant => "constant"
        })
    }
}

impl IRElement for GlobalVariableType {}

