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

impl Display for AtomicMemoryOrderingConstraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unordered => f.write_str("unordered"),
            Self::Monotonic => f.write_str("monotonic"),
            Self::Acquire => f.write_str("acquire"),
            Self::Release => f.write_str("release"),
            Self::AcquireRelease => f.write_str("acq_rel"),
            Self::SequentiallyConsistent => f.write_str("swq_cst")
        }
    }
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
                if *volatile { f.write_str("volatile ")?; }
                let pointer_type = match value.return_type() {
                    Type::SingleValue(crate::types::first_class::single_value::Type::Pointer(_type)) => _type,
                    _ => panic!()
                };
                f.write_fmt(format_args!("{}, {}", pointer_type._type, value))?;
                if *alignment != 0 { f.write_fmt(format_args!(", align {}", alignment))?; }
            }
            Self::Atomic { volatile, value, syncscope, ordering, alignment } => {
                f.write_str("atomic ")?;
                if *volatile { f.write_str("volatile ")?; }
                let pointer_type = match value.return_type() {
                    Type::SingleValue(crate::types::first_class::single_value::Type::Pointer(_type)) => _type,
                    _ => panic!()
                };
                f.write_fmt(format_args!("{}, {}", pointer_type._type, value))?;
                if let Some(syncscope) = syncscope { f.write_fmt(format_args!(" syncscope(\"{}\")", syncscope))?; }
                f.write_fmt(format_args!(" {}, align {}", ordering, alignment))?;
            }
        }
        Ok(())
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

impl Display for StoreInstruction<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("store ")?;
        match self {
            Self::Normal { volatile, value, pointer, alignment } => {
                if *volatile { f.write_str("volatile ")?; }
                f.write_fmt(format_args!("{}, {}", value, pointer))?;
                if *alignment != 0 { f.write_fmt(format_args!(", align {}", alignment))?; }
            }
            Self::Atomic { volatile, value, pointer, syncscope, ordering, alignment } => {
                f.write_str("atomic ")?;
                if *volatile { f.write_str("volatile ")?; }
                f.write_fmt(format_args!("{}, {}", value, pointer))?;
                if let Some(syncscope) = syncscope { f.write_fmt(format_args!(" syncscope(\"{}\")", syncscope))?; }
                f.write_fmt(format_args!(" {}, align {}", ordering, alignment))?;
            }
        }
        Ok(())
    }
}

struct FenceInstruction<'s> {
    syncscope: Option<&'s str>,
    ordering: AtomicMemoryOrderingConstraint
}

impl Display for FenceInstruction<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("fence ")?;
        if let Some(syncscope) = self.syncscope { f.write_fmt(format_args!("syncscope(\"{}\") ", syncscope))?; }
        self.ordering.fmt(f)
    }
}

struct CompareExchangeInstruction<'s> {
    weak: bool,
    volatile: bool,
    pointer: Value<'s>,
    compare: Value<'s>,
    new: Value<'s>,
    syncscope: Option<&'s str>,
    success_ordering: AtomicMemoryOrderingConstraint,
    failure_ordering: AtomicMemoryOrderingConstraint,
    alignment: usize,
}

impl Display for CompareExchangeInstruction<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("cmpxchg ")?;
        if self.weak { f.write_str(" weak")?; }
        if self.volatile { f.write_str(" volatile")?; }
        f.write_fmt(format_args!("{}, {}, {}", self.pointer, self.compare, self.new))?;
        if let Some(syncscope) = self.syncscope { f.write_fmt(format_args!(" syncscope(\"{}\")", syncscope))?; }
        f.write_fmt(format_args!(" {} {}", self.success_ordering, self.failure_ordering))?;
        if self.alignment != 0 { f.write_fmt(format_args!(", align {}", self.alignment))?; }
        Ok(())
    }
}

pub enum AtomicReadModifyWriteOperation {
    Exchange,
    Add,
    Subtract,
    And,
    NotAnd,
    Or,
    ExclusiveOr,
    Maximum,
    Minimum,
    UnsignedMaximum,
    UnsignedMinimum,
    FloatAdd,
    FloatSubtract
}

impl Display for AtomicReadModifyWriteOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Exchange => "xchg",
            Self::Add => "add",
            Self::Subtract => "sub",
            Self::And => "and",
            Self::NotAnd => "nand",
            Self::Or => "or",
            Self::ExclusiveOr => "xor",
            Self::Maximum => "max",
            Self::Minimum => "min",
            Self::UnsignedMaximum => "umax",
            Self::UnsignedMinimum => "umin",
            Self::FloatAdd => "fadd",
            Self::FloatSubtract => "fsub"
        })
    }
}

struct AtomicReadModifyWriteInstruction<'s> {
    volatile: bool,
    operation: AtomicReadModifyWriteOperation,
    pointer: Value<'s>,
    value: Value<'s>,
    syncscope: Option<&'s str>,
    ordering: AtomicMemoryOrderingConstraint,
    alignment: usize
}

impl Display for AtomicReadModifyWriteInstruction<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("atomicrmw ")?;
        if self.volatile { f.write_str("volatile ")?; }
        f.write_fmt(format_args!("{} {}, {}", self.operation, self.pointer, self.value))?;
        Ok(())
    }
}

struct GetElementPointerInstruction<'s> {
    inbounds: bool,
    pointer: Value<'s>,
    indecies: Vec<(bool, Value<'s>)>
}

impl Display for GetElementPointerInstruction<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("getelementptr ")?;
        if self.inbounds { f.write_str("inbounds")?; }
        self.pointer.fmt(f)?;
        for (inbound, index) in &self.indecies {
            f.write_str(", ")?;
            if *inbound { f.write_str("inbound")?; }
            index.fmt(f)?;
        }
        Ok(())
    }
}

