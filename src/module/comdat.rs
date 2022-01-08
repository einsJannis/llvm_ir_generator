use std::fmt::{Debug, Display, Formatter};
use crate::types::Type;
use crate::{Element, WithName, WithReturnType};

#[derive(Clone, Debug)]
enum SelectionKind {}

impl Display for SelectionKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Element for SelectionKind {}

#[derive(Clone, Debug)]
pub struct Comdat {
    name: String,
    selection_kind: SelectionKind
}

impl Display for Comdat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("${} = comdat {}", self.name, self.selection_kind))
    }
}

impl Element for Comdat {}

impl WithReturnType for Comdat {
    fn return_type(&self) -> Box<dyn Type> {
        crate::types::void()
    }
}

impl WithName for Comdat {
    fn name(&self) -> String {
        format!("${}", self.name)
    }
}
