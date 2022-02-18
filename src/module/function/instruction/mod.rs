use std::fmt::Display;

use crate::identifier::LocalIdentifier;

pub mod terminator;
pub mod unary_operator;
pub mod binary_operator;
pub mod bitwise_binary_operator;
pub mod vector_operator;
pub mod aggregate_operator;
pub mod memory_operator;

pub enum Instruction<'s> {
    Terminal(terminator::Instruction<'s>),
    UnaryOperator(unary_operator::Instruction<'s>),
    BinaryOperator(binary_operator::Instruction<'s>),
    BitwiseBinaryOperator(bitwise_binary_operator::Instruction<'s>),
    VectorOperator(vector_operator::Instruction<'s>),
    AggregateOperator(aggregate_operator::Instruction<'s>),
    MemoryOperator(memory_operator::Instruction<'s>),
}

impl Display for Instruction<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (match self {
            Instruction::Terminal(it) => it as &dyn Display,
            Instruction::UnaryOperator(it) => it as &dyn Display,
            Instruction::BinaryOperator(it) => it as &dyn Display,
            Instruction::BitwiseBinaryOperator(it) => it as &dyn Display,
            Instruction::VectorOperator(it) => it as &dyn Display,
            Instruction::AggregateOperator(it) => it as &dyn Display,
            Instruction::MemoryOperator(it) => it as &dyn Display,
        }).fmt(f)
    }
}

pub enum ReturningInstruction<'s> {
    Terminal(terminator::ReturningInstruction<'s>),
    UnaryOperator(unary_operator::ReturningInstruction<'s>),
}

impl Display for ReturningInstruction<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (match self {
            Self::Terminal(it) => it as &dyn Display,
            Self::UnaryOperator(it) => it as &dyn Display
        }).fmt(f)
    }
}

pub struct Variable<'s> {
    identifier: LocalIdentifier<'s>,
    instruction: ReturningInstruction<'s>
}

impl Display for Variable<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} = {}", self.identifier, self.instruction))
    }
}

