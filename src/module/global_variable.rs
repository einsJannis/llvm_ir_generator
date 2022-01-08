use std::fmt::{Display, Formatter, Write};
use std::rc::Rc;
use crate::{DLLStorageClass, Element, LinkageType, RuntimePreemptionSpecifier, ThreadLocalStorageModel, VisibilityStyle, WithName, WithReturnType};
use crate::constant::Constant;
use crate::metadata::Metadata;
use crate::module::{Module, ModuleElement};
use crate::module::comdat::Comdat;
use crate::reference::Reference;
use crate::types::Type;

#[derive(Clone, Debug)]
struct GlobalVariable {
    module: Rc<Module>,
    name: String,
    linkage: LinkageType,
    preemption_specifier: Option<RuntimePreemptionSpecifier>,
    visibility: Option<VisibilityStyle>,
    dll_storage_class: Option<DLLStorageClass>,
    thread_local: bool,
    thread_local_storage: Option<ThreadLocalStorageModel>,
    unnamed_addr: bool,
    local: bool,
    addr_space: Option<usize>,
    externally_initialized: bool,
    constant: bool,
    return_type: Box<dyn Type>,
    initializer_literal: Option<Box<dyn Constant>>,
    section: Option<String>,
    comdat: Option<Reference<Comdat>>,
    align: Option<usize>,
    metadata: Vec<Box<dyn Metadata>>
}

impl WithReturnType for GlobalVariable {
    fn return_type(&self) -> Box<dyn Type> {
        crate::types::pointer(self.return_type.clone(), None)
    }
}

impl ModuleElement for GlobalVariable {
    fn raw_name(&self) -> String {
        self.name.clone()
    }
    fn linkage_type(&self) -> LinkageType {
        self.linkage.clone()
    }
    fn module(&self) -> Rc<Module> {
        self.module.clone()
    }
}

impl Display for GlobalVariable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.name().fmt(f);
        f.write_str(" = ");
        if self.linkage != LinkageType::External {
            self.linkage.fmt(f);
            f.write_char(' ');
        }
        if let Some(&preemption_specifier) = self.preemption_specifier {
            preemption_specifier.fmt(f);
            f.write_char(' ');
        }
        if let Some(&visibility) = self.visibility {
            visibility.fmt(f);
            f.write_char(' ');
        }
        if let Some(&dll_storage_class) = self.dll_storage_class {
            dll_storage_class.fmt(f);
            f.write_char(' ');
        }
        if self.thread_local {
            f.write_str("thread_local");
            if let Some(&tls) = self.thread_local_storage {
                f.write_fmt(format_args!("({})", tls));
            }
            f.write_char(' ');
        }
        if self.unnamed_addr {
            f.write_str(match self.local {
                true => "local_unnamed_addr",
                false => "unnamed_addr"
            })
        }
        if let Some(addr_space) = self.addr_space {
            addr_space.fmt(f);
            f.write_char(' ');
        }
        if self.externally_initialized {
            f.write_str("external ")
        }
        f.write_str(match self.constant {
            true => "constant ",
            false => "global "
        });
        self.return_type.fmt(f);
        if let Some(&initializer_literal) = self.initializer_literal {
            f.write_char(' ');
            initializer_literal.fmt(f);
        }
        if let Some(&section) = self.section {
            f.write_fmt(format_args!(", {}", section))
        }
        if let Some(&comdat) = self.comdat {
            f.write_fmt(format_args!(", {}", comdat))
        }
        if let Some(&align) = self.align {
            f.write_fmt(format_args!(", align {}", align))
        }
        self.metadata.iter().for_each(|it| f.write_fmt(format_args!(", {}", it))?);
        Ok(())
    }
}
