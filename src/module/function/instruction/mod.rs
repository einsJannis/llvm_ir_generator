use std::fmt::Display;

pub mod terminator;

pub enum Instruction<'s> {
    Terminal(terminator::Instruction<'s>)
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (match self {
            Instruction::Terminal(it) => it as &dyn Display,
        }).fmt(f)
    }
}

