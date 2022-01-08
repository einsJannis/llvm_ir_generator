mod module;
mod types;
mod metadata;
mod constant;
mod reference;
use std::fmt::{Debug, Display, Formatter, LowerHex, Write};
use std::rc::Rc;
use crate::types::WithReturnType;

trait Element : Clone + Debug + Display {}

trait WithName : Element {
    fn name(&self) -> String;
}

impl<T> Element for T where T: WithName {}

#[derive(Clone, Debug)]
enum LinkageType {
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
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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

impl Element for LinkageType {}

#[derive(Clone, Debug)]
enum CallingConvention {
    C,
    Fast,
    Cold,
    GHC,
    HiPE,
    WebKit,
    Dynamic,
    PreserveMost,
    PreserveAll,
    CXXFastTLS,
    TailCallable,
    Swift,
    SwiftTailCallable,
    WindowsControlFlowGuard,
    Numbered(usize)
}

impl Display for CallingConvention {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CallingConvention::C => f.write_str("ccc"),
            CallingConvention::Fast => f.write_str("fastcc"),
            CallingConvention::Cold => f.write_str("coldcc"),
            CallingConvention::GHC => f.write_str("cc 10"),
            CallingConvention::HiPE => f.write_str("cc 11"),
            CallingConvention::WebKit => f.write_str("webkit_jscc"),
            CallingConvention::Dynamic => f.write_str("anyregcc"),
            CallingConvention::PreserveMost => f.write_str("preserve_mostcc"),
            CallingConvention::PreserveAll => f.write_str("preserve_allcc"),
            CallingConvention::CXXFastTLS => f.write_str("cxx_fast_tlscc"),
            CallingConvention::TailCallable => f.write_str("tailcc"),
            CallingConvention::Swift => f.write_str("swiftcc"),
            CallingConvention::SwiftTailCallable => f.write_str("swifttailcc"),
            CallingConvention::WindowsControlFlowGuard => f.write_str("cfguard_checkcc"),
            CallingConvention::Numbered(n) => f.write_fmt(format_args!("cc {}", n))
        }
    }
}

impl Element for CallingConvention {}

#[derive(Clone, Debug)]
enum VisibilityStyle {
    Default,
    Hidden,
    Protected
}

impl Display for VisibilityStyle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            VisibilityStyle::Default => "default",
            VisibilityStyle::Hidden => "hidden",
            VisibilityStyle::Protected => "protected"
        })
    }
}

impl Element for VisibilityStyle {}

#[derive(Clone, Debug)]
enum DLLStorageClass {
    DLLImport,
    DLLExport
}

impl Display for DLLStorageClass {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            DLLStorageClass::DLLImport => "dllimport",
            DLLStorageClass::DLLExport => "dllexport"
        })
    }
}

impl Element for DLLStorageClass {}

#[derive(Clone, Debug)]
enum ThreadLocalStorageModel {
    LocalDynamic,
    InitialExec,
    LocalExec
}

impl Display for ThreadLocalStorageModel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            ThreadLocalStorageModel::LocalDynamic => "localdynamic",
            ThreadLocalStorageModel::InitialExec => "initialexec",
            ThreadLocalStorageModel::LocalExec => "localexec"
        })
    }
}

impl Element for ThreadLocalStorageModel {}

#[derive(Clone, Debug)]
enum RuntimePreemptionSpecifier {
    DSOPreemptable,
    DSOLocal
}


impl Display for RuntimePreemptionSpecifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            RuntimePreemptionSpecifier::DSOPreemptable => "dso_preemptable",
            RuntimePreemptionSpecifier::DSOLocal => "dso_local"
        })
    }
}

impl Element for RuntimePreemptionSpecifier {}

trait Value : WithReturnType {}
