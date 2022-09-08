use core::fmt::{Display, Debug};
use crate::IRElement;

pub mod global_variable;
pub mod function;

#[repr(transparent)]
pub struct Module<'s>(Vec<ModuleElement<'s>>);

#[derive(Debug, Clone, PartialEq)]
pub enum ModuleElement<'s> {
    GlobalVariable(global_variable::GlobalVariable<'s>)
}

impl<'s> IRElement for ModuleElement<'s> {}

impl<'s> Display for ModuleElement<'s> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GlobalVariable(it) => it.fmt(f)
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub enum LinkageType {
    Private,
    Internal,
    AvailableExternally,
    LinkOnce,
    Weak,
    Common,
    Appending,
    ExternWeak,
    LinkOnceOdr,
    WeakOdr,
    External
}

impl IRElement for LinkageType {}

impl Display for LinkageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            LinkageType::Private => "private",
            LinkageType::Internal => "internal",
            LinkageType::AvailableExternally => "available_externally",
            LinkageType::LinkOnce => "linkonce",
            LinkageType::Weak => "weak",
            LinkageType::Common => "common",
            LinkageType::Appending => "appending",
            LinkageType::ExternWeak => "extern_weak",
            LinkageType::LinkOnceOdr => "linkonce_odr",
            LinkageType::WeakOdr => "weak_odr",
            LinkageType::External => "external"
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub enum CallingConvention {
    C,
    Fast,
    Cold,
    GHC,
    HiPE,
    WebKitJs,
    AnyReg,
    PreserveMost,
    PreserveAll,
    CXXFastTLS,
    Tail,
    Swift,
    SwiftTail,
    //PropirataryBullShit,
    Numbered(usize)
}

impl IRElement for CallingConvention {}

impl Display for CallingConvention {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CallingConvention::Numbered(n) => f.write_fmt(format_args!("cc {}", n)),
            _ => f.write_str(match self {
                CallingConvention::C => "ccc",
                CallingConvention::Fast => "fastcc",
                CallingConvention::Cold => "coldcc",
                CallingConvention::GHC => "cc 10",
                CallingConvention::HiPE => "cc 11",
                CallingConvention::WebKitJs => "webkit_jscc",
                CallingConvention::AnyReg => "anyregcc",
                CallingConvention::PreserveMost => "preserve_mostcc",
                CallingConvention::PreserveAll => "preserve_allcc",
                CallingConvention::CXXFastTLS => "cxx_fast_tlscc",
                CallingConvention::Tail => "tailcc",
                CallingConvention::Swift => "swiftcc",
                CallingConvention::SwiftTail => "swifttailcc",
                //CallingConvention::PropirataryBullShit => "cfguard_checkcc",
                _ => unreachable!()
            })
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub enum VisibilityStyle {
    Default,
    Hidden,
    Protected
}

impl IRElement for VisibilityStyle {}

impl Display for VisibilityStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            VisibilityStyle::Default => "default",
            VisibilityStyle::Hidden => "hidden",
            VisibilityStyle::Protected => "protected"
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub enum DLLStorageClass {
    Import,
    Export
}

impl IRElement for DLLStorageClass {}

impl Display for DLLStorageClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            DLLStorageClass::Import => "dllimport",
            DLLStorageClass::Export => "dllexport"
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub enum ThreadLocalStorageModel {
    LocalDynamic,
    InitialExecution,
    LocalExecution,
    GeneralDynamic
}

impl IRElement for ThreadLocalStorageModel {}

impl Display for ThreadLocalStorageModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            ThreadLocalStorageModel::LocalDynamic => "localdynamic",
            ThreadLocalStorageModel::InitialExecution => "initialexec",
            ThreadLocalStorageModel::LocalExecution => "localexec",
            ThreadLocalStorageModel::GeneralDynamic => ""
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub enum RuntimePreemptionSpecifier {
    Preemptable,
    Local
}

impl IRElement for RuntimePreemptionSpecifier {}

impl Display for RuntimePreemptionSpecifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            RuntimePreemptionSpecifier::Preemptable => "dso_preemptable",
            RuntimePreemptionSpecifier::Local => "dso_local"
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub enum UnnamedAddress {
    Local,
    NonLocal
}

impl IRElement for UnnamedAddress {}

impl Display for UnnamedAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            UnnamedAddress::Local => "local_unnamed_address",
            UnnamedAddress::NonLocal => "unnamed_address"
        })
    }
}

