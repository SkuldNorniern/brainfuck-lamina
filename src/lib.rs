//! Brainfuck compiler powered by Lamina
//!
//! This crate provides a Brainfuck compiler implementation using the Lamina
//! compiler framework as the backend.

pub mod token;
pub mod lexer;
pub mod lamina_builder;

// Re-export commonly used types
pub use lexer::{parse_brainfuck, AstNode, Command, Lexer, LexerError};
pub use token::Token;
pub use lamina_builder::{
    brainfuck_to_lamina_ir,
    brainfuck_to_lamina_ir_with_config,
    brainfuck_to_assembly,
    brainfuck_to_assembly_with_config,
    brainfuck_to_binary,
    brainfuck_to_binary_with_config,
    brainfuck_to_ir_description,
    BrainfuckConfig,
    BrainfuckIRBuilder
};
