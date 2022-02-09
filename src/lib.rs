use core::fmt::{Display, Debug};

pub mod identifier;
pub mod module;
pub mod types;

pub trait IRElement: Display + Debug {}

