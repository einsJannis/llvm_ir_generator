use std::fmt::Display;

use crate::{identifier::{Identifier, Identifiable, GlobalIdentifier}, types::{first_class::Type, ReturnType}, constant::Constant};

use super::{LinkageType, DLLStorageClass, UnnamedAddress, CallingConvention, VisibilityStyle, RuntimePreemptionSpecifier};

pub mod instruction;

pub struct Function<'s> {
    identifier: GlobalIdentifier<'s>,
    linkage: Option<LinkageType>,
    preemtion_specifier: RuntimePreemptionSpecifier,
    visibility: VisibilityStyle,
    dll_storage_class: Option<DLLStorageClass>,
    calling_convention: CallingConvention,
    //return_attributes: Vec<ReturnAttribute>,
    pub(crate) return_type: Type,
    pub(crate) arguments: Vec<Argument<'s>>,
    unnamed_address: Option<UnnamedAddress>,
    address_space: usize,
    //function_attributes: Vec<FunctionAttribute>,
    section_name: Option<&'s str>,
    partition_name: Option<&'s str>,
    //comdat: Option<Comdat>,
    align: usize,
    //garbage_collector: Option<&'s str>,
    prefix_constant: Option<Constant<'s>>,
    prologue_constant: Option<Constant<'s>>,
    personality_constant: Option<Constant<'s>>,
    //metadata: Vec<Metadata>
    instruction_blocks: Vec<InstructionBlock<'s>>
}

impl<'s> Display for Function<'s> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("define ")?;
        if let Some(linkage) = &self.linkage { f.write_fmt(format_args!("{} ", linkage))?; }
        if let RuntimePreemptionSpecifier::Local = self.preemtion_specifier { f.write_fmt(format_args!("{} ", self.preemtion_specifier))?; }
        if let VisibilityStyle::Default = self.visibility {} else { f.write_fmt(format_args!("{} ", self.visibility))?; }
        if let Some(dll_storage_class) = &self.dll_storage_class { f.write_fmt(format_args!("{} ", dll_storage_class))?; }
        if let CallingConvention::C = self.calling_convention {} else { f.write_fmt(format_args!("{} ", self.calling_convention))?; }
        f.write_fmt(format_args!("{} {} ( ", self.return_type, self.identifier))?;
        for (i, argument) in self.arguments.iter().enumerate() {
            argument.fmt(f)?;
            if i < (self.arguments.len()-1) { f.write_str(", ")?; }
        }
        f.write_str(")")?;
        if let Some(unnamed_address) = &self.unnamed_address { f.write_fmt(format_args!(" {}", unnamed_address))?; }
        if self.address_space != 0 { f.write_fmt(format_args!(" addrspace({})", self.address_space))?; }
        if let Some(section_name) = self.section_name { f.write_fmt(format_args!(" section \"{}\"", section_name))?; }
        if let Some(partition_name) = self.partition_name { f.write_fmt(format_args!(" partition \"{}\"", partition_name))?; }
        //comdat
        if self.align != 0 { f.write_fmt(format_args!(" align {}", self.align))?; }
        //gc
        if let Some(prefix_constant) = &self.prefix_constant { f.write_fmt(format_args!(" prefix {}", prefix_constant))?; }
        if let Some(prologue_constant) = &self.prologue_constant { f.write_fmt(format_args!(" prologue {}", prologue_constant))?; }
        if let Some(personality_constant) = &self.personality_constant { f.write_fmt(format_args!(" personality {}", personality_constant))?; }
        f.write_str(" {\n")?;
        for instruction_block in &self.instruction_blocks {
            instruction_block.fmt(f)?;
        }
        f.write_str("}\n")?;
        Ok(())
    }
}

pub struct Argument<'s> {
    pub(crate) return_type: Type,
    pub(crate) identifier: Identifier<'s>
}

impl<'s> Display for Argument<'s> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} {}", self.return_type(), self.id()))
    }
}

impl<'s> Identifiable for Argument<'s> {
    fn id(&self) -> Identifier<'s> { self.identifier.clone().into() }
}

impl<'s> ReturnType for Argument<'s> {
    fn return_type(&self) -> crate::types::first_class::Type { self.return_type.clone() }
}

pub struct InstructionBlock<'s> {
    identifier: GlobalIdentifier<'s>,
    instructions: Vec<instruction::Instruction<'s>>
}

impl<'s> Display for InstructionBlock<'s> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}:\n", self.identifier.0))?;
        for instruction in &self.instructions {
            instruction.fmt(f)?;
        }
        Ok(())
    }
}

impl<'s> Identifiable for InstructionBlock<'s> {
    fn id(&self) -> Identifier<'s> {
        self.identifier.clone().into()
    }
}

