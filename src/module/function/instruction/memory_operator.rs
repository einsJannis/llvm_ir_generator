use std::fmt::Display;

use crate::{types::{first_class::Type, ReturnType}, reference::Value};

pub enum Instruction<'s> {
    AllocA(AllocAInstruction<'s>),
}

impl Display for Instruction<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (match self {
            Instruction::AllocA(it) => it as &dyn Display,
        }).fmt(f)
    }
}

pub struct AllocAInstruction<'s> {
    //inalloca,
    _type: Type,
    num_elements: Option<Value<'s>>,
    align: Option<usize>,
    addrspace: Option<usize>
}

impl Display for AllocAInstruction<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("alloca {}", self._type))?;
        if let Some(num_elements) = &self.num_elements { f.write_fmt(format_args!(", {}", num_elements))?; }
        if let Some(align) = self.align { f.write_fmt(format_args!(", {}", align))?; }
        if let Some(addrspace) = self.addrspace { f.write_fmt(format_args!(", {}", addrspace))?; }
        Ok(())
    }
}

pub enum AtomicMemoryOrderingConstraint {
    Unordered,
    Monotonic,
    Acquire,
    Release,
    AcquireRelease,
    SequentiallyConsistent
}

pub enum LoadInstruction<'s> {
    Normal {
        volatile: bool,
        value: Value<'s>,
        alignment: usize,
        //nontemporal: Option<Metadata>,
        //invariant_load: Option<Metadata>,
        //invariant_group: Option<Metadata>,
        //nonnull: Option<Metadata>,
        //dereferencable: Option<Metadata>,
        //dereferencable_or_null: Option<Metadata>,
        //align_m: Option<Metadata>,
        //noundef: Option<Metadata>
    },
    Atomic {
        volatile: bool,
        value: Value<'s>,
        syncscope: Option<&'s str>,
        ordering: AtomicMemoryOrderingConstraint,
        alignment: usize,
        //invariant_group: Metadata,
    }
}

impl Display for LoadInstruction<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("load ")?;
        match self {
            Self::Normal { volatile, value, alignment } => {
                if volatile { f.write_str("volatile ")?; }
                let pointer_type = match value.return_type() {
                    Type::SingleValue(crate::types::first_class::single_value::Type::Pointer(_type)) => _type,
                    _ => panic!()
                };
                f.write_fmt(format_args!("{}, {}", return_type._type, value))?;
                if alignment != 0 { f.write_fmt(", align {}", alignment)?; }
            }
            Self::Atomic { volatile, value, syncscope, ordering, alignment } => {
                f.write_str("atomic ")?;
                if volatile { f.write_str("volatile ")?; }
                let pointer_type = match value.return_type() {
                    Type::SingleValue(crate::types::first_class::single_value::Type::Pointer(_type)) => _type,
                    _ => panic!()
                };
                f.write_fmt(format_args!("{}, {}", return_type._type, value))?;
                if let Some(syncscope) = syncscope { f.write_fmt(" syncscope(\"{}\")", syncscope)?; }
                f.write_fmt(format_args!(" {}, align {}", ordering, alignment))?;
            }
        }
    }
}

pub enum StoreInstruction<'s> {
    Normal {
        volatile: bool,
        value: Value<'s>,
        pointer: Value<'s>,
        alignment: usize,
        //nontemporal: Metadata,
        //invariant_group: Metadata,
    },
    Atomic {
        volatile: bool,
        value: Value<'s>,
        pointer: Value<'s>,
        syncscope: Option<&'s str>,
        ordering: AtomicMemoryOrderingConstraint,
        alignment: usize,
        //invariant_group: Metadata,
    }
}

impl Display for StoreInstruction<'s> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("store ")?;
        match self {
            Self::Normal { volatile, value, pointer, alignment } => {
                if volatile { f.write_str("volatile ")?; }
                f.write_fmt(format_args!("{}, {}", value, pointer))?;
                if alignment != 0 { f.write_fmt(", align {}", alignment)?; }
            }
            Self::Atomic { volatile, value, pointer, syncscope, ordering, alignment } => {
                f.write_str("atomic ")?;
                if volatile { f.write_str("volatile ")?; }
                f.write_fmt(format_args!("{}, {}", value, pointer))?;
                if let Some(syncscope) = syncscope { f.write_fmt(" syncscope(\"{}\")", syncscope)?; }
                f.write_fmt(format_args!(" {}, align {}", ordering, alignment))?;
            }
        }
    }
}

