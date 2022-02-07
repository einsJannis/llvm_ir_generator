use core::fmt::{Display, Debug};
use crate::IRElement;

mod global_variable;

#[repr(transparent)]
pub struct Module(Vec<ModuleElement>);

pub enum ModuleElement {
    
}

#[derive(Debug)]
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

impl IRElement for LinkageType {}

#[derive(Debug)]
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

impl IRElement for CallingConvention {}

#[derive(Debug)]
#[repr(C)]
pub enum VisibilityStyle {
    Default,
    Hidden,
    Protected
}

impl Display for VisibilityStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            VisibilityStyle::Default => "default",
            VisibilityStyle::Hidden => "hidden",
            VisibilityStyle::Protected => "protected"
        })
    }
}

impl IRElement for VisibilityStyle {}

#[derive(Debug)]
#[repr(C)]
pub enum DLLStorageClass {
    Import,
    Export
}

impl Display for DLLStorageClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            DLLStorageClass::Import => "dllimport",
            DLLStorageClass::Export => "dllexport"
        })
    }
}

impl IRElement for DLLStorageClass {}

#[derive(Debug)]
#[repr(C)]
pub enum ThreadLocalStorageModel {
    LocalDynamic,
    InitialExecution,
    LocalExecution,
    GeneralDynamic
}

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

impl IRElement for ThreadLocalStorageModel {}

#[derive(Debug)]
#[repr(C)]
pub enum RuntimePreemptionSpecifier {
    Preemptable,
    Local
}

impl Display for RuntimePreemptionSpecifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            RuntimePreemptionSpecifier::Preemptable => "dso_preemptable",
            RuntimePreemptionSpecifier::Local => "dso_local"
        })
    }
}

impl IRElement for RuntimePreemptionSpecifier {}

#[derive(Debug)]
#[repr(C)]
pub enum UnnamedAddress {
    Local,
    NonLocal
}

impl Display for UnnamedAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            UnnamedAddress::Local => "local_unnamed_address",
            UnnamedAddress::NonLocal => "unnamed_address"
        })
    }
}

impl IRElement for UnnamedAddress {}

