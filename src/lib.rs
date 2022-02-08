use core::fmt::{Display, Debug};

mod identifier;
mod module;
mod types;

trait IRElement: Display + Debug {}

