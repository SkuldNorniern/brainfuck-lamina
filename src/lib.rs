//! Brainfuck compiler powered by Lamina
//!
//! This crate provides a Brainfuck compiler implementation using the Lamina
//! compiler framework as the backend.

pub mod lamina_builder;
pub mod lexer;
pub mod token;

// Re-export commonly used types
pub use lamina_builder::{
    BrainfuckConfig, BrainfuckIRBuilder, brainfuck_to_assembly, brainfuck_to_assembly_with_config,
    brainfuck_to_binary, brainfuck_to_binary_with_config, brainfuck_to_lamina_ir,
    brainfuck_to_lamina_ir_with_config,
};
pub use lexer::{AstNode, Command, Lexer, LexerError, parse_brainfuck};
pub use token::Token;
