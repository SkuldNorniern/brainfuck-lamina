//! Lamina IR Builder for Brainfuck Compilation
//!
//! This module provides functionality to convert Brainfuck AST into Lamina IR
//! and generate assembly code using the Lamina compiler framework.

pub mod compiler;
pub mod config;
pub mod ir_builder;
pub mod utils;

// Re-export commonly used types and functions
pub use compiler::{
    brainfuck_to_assembly, brainfuck_to_assembly_with_config, brainfuck_to_binary,
    brainfuck_to_binary_with_config, brainfuck_to_lamina_ir, brainfuck_to_lamina_ir_with_config,
};
pub use config::BrainfuckConfig;
pub use ir_builder::BrainfuckIRBuilder;


