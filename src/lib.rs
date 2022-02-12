use core::fmt::{Display, Debug};

pub mod identifier;
pub mod module;
pub mod types;
pub mod constant;
pub mod reference;

pub trait IRElement: Display + Debug {}

