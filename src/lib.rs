//! Brainfuck compiler powered by Lamina
//!
//! This crate provides a Brainfuck compiler implementation using the Lamina
//! compiler framework as the backend.

pub mod token;
pub mod lexer;

// Re-export commonly used types
pub use lexer::{parse_brainfuck, AstNode, Command, Lexer, LexerError};
pub use token::Token;
