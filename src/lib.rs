use core::fmt::{Display, Debug};

pub mod identifier;
pub mod module;
pub mod types;
pub mod constant;

pub trait IRElement: Display + Debug {}

