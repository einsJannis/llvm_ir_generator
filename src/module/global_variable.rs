use std::fmt::Display;

use crate::identifier::{GlobalIdentifier, Identifiable};
use crate::constant::Constant;
use crate::module::{LinkageType, RuntimePreemptionSpecifier, VisibilityStyle, DLLStorageClass, UnnamedAddress};
use crate::types::ReturnType;
use crate::types::first_class::Type;

use super::ThreadLocalStorageModel;

pub struct GlobalVariable<'s> {
    identifier: GlobalIdentifier<'s>,
    linkage: Option<LinkageType>,
    preemtion_specifier: RuntimePreemptionSpecifier,
    visibility: VisibilityStyle,
    dll_storage_class: Option<DLLStorageClass>,
    thread_local: ThreadLocalStorageModel,
    unnamed_address: Option<UnnamedAddress>,
    address_space: usize,
    externally_initialized: bool,
    global_variable_type: GlobalVariableType,
    return_type: Type,
    initializer_constant: Option<Constant<'s>>,
    section_name: Option<&'s str>,
    partition_name: Option<&'s str>,
    //comdats: Vec<Comdat>,
    allign: usize,
    //metadata: Vec<Metadata>
}

impl<'s> Display for GlobalVariable<'s> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} = ", self.identifier))?;
        if let Some(linkage) = &self.linkage { f.write_fmt(format_args!("{} ", linkage))?; }
        if let RuntimePreemptionSpecifier::Local = self.preemtion_specifier { f.write_fmt(format_args!("{} ", self.preemtion_specifier))?; }
        if let VisibilityStyle::Default = self.visibility { f.write_fmt(format_args!("{} ", self.visibility))?; }
        if let Some(dll_storage_class) = &self.dll_storage_class { f.write_fmt(format_args!("{} ", dll_storage_class))?; }
        if let ThreadLocalStorageModel::GeneralDynamic = self.thread_local {} else {
            f.write_fmt(format_args!("{} ", self.thread_local))?;
        }
        if let Some(unnamed_address) = &self.unnamed_address { f.write_fmt(format_args!("{} ", unnamed_address))?; }
        if self.address_space != 0 { f.write_fmt(format_args!("addrspace({}) ", self.address_space))?; }
        if self.externally_initialized { f.write_str("externally_initialized ")?; }
        f.write_fmt(format_args!("{} ", self.global_variable_type))?;
        self.return_type.fmt(f)?;
        if let Some(initializer_constant) = &self.initializer_constant {
            f.write_fmt(format_args!(" {}", initializer_constant))?;
        }
        if let Some(section_name) = &self.section_name { f.write_fmt(format_args!(", section \"{}\"", section_name))?; }
        if let Some(partition_name) = &self.partition_name { f.write_fmt(format_args!(", partition \"{}\"", partition_name))?; }
        //comdats
        if self.allign != 0 { f.write_fmt(format_args!(", align {}", self.allign))?; }
        //metadata
        Ok(())
    }
}

impl<'s> Identifiable for GlobalVariable<'s> {
    fn id(&self) -> crate::identifier::Identifier { self.identifier.clone().into() }
}

impl<'s> ReturnType for GlobalVariable<'s> {
    fn return_type(&self) -> Type { self.return_type.clone() }
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

