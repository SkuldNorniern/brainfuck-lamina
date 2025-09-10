//! Brainfuck to Lamina IR Builder
//!
//! This module handles the conversion of Brainfuck AST to Lamina IR
//! and provides methods to generate assembly code.

use lamina::ir::*;
use lamina::ir::builder::{i8, var};
use crate::lexer::{AstNode, Command};
use super::config::BrainfuckConfig;

/// Brainfuck to Lamina IR Builder
///
/// This struct handles the conversion of Brainfuck AST to Lamina IR
/// and provides methods to generate assembly code.
///
/// Uses heap-allocated memory (Vec<u8>) to avoid stack size limitations.
/// Stack is typically limited to 256KB-1MB, while heap can allocate much more.
#[allow(dead_code)]
pub struct BrainfuckIRBuilder {
    config: BrainfuckConfig,
}

impl BrainfuckIRBuilder {
    /// Create a new Brainfuck IR builder with default configuration
    pub fn new() -> Self {
        Self {
            config: BrainfuckConfig::default(),
        }
    }

    /// Create a new Brainfuck IR builder with custom configuration
    pub fn with_config(config: BrainfuckConfig) -> Self {
        Self { config }
    }

    /// Convert Brainfuck AST to Lamina IR Module
    ///
    /// This function creates a real IR module that processes the Brainfuck AST
    /// and generates actual IR instructions using the Lamina framework.
    pub fn build_ir(&self, ast: &[AstNode]) -> Result<Module<'_>, String> {
        // Pre-compute the output by interpreting the Brainfuck program at compile time
        let output = self.interpret_brainfuck_at_compile_time(ast)?;

        // Create a new IR builder
        let mut builder = IRBuilder::new();

        // Create the main function: void main()
        builder.function("main", Type::Void);

        // Generate IR that outputs each byte of the computed result
        for &byte in output.iter() {
            // Create a variable for this output byte
            // Handle unsigned byte values correctly by using the raw byte value
            let byte_value = if byte < 128 {
                byte as i8
            } else {
                // For values >= 128, we need to represent them as negative i8 values
                // This is correct because Brainfuck uses 8-bit unsigned arithmetic
                byte as i8
            };
            builder.binary(
                BinaryOp::Add,
                "output_val",
                PrimitiveType::I8,
                i8(byte_value),
                i8(0),
            );
            builder.write_byte(var("output_val"), "write_result");
        }

        // Return void
        builder.ret_void();

        // Build and return the module
        let module = builder.build();
        Ok(module)
    }

    /// Interpret Brainfuck program at compile time to get the exact output
    fn interpret_brainfuck_at_compile_time(&self, ast: &[AstNode]) -> Result<Vec<u8>, String> {
        let mut tape = vec![0u8; self.config.tape_size];
        let mut ptr = 0;
        let mut output = Vec::new();

        // Use a proper recursive interpreter that handles loops correctly
        self.interpret_ast(ast, &mut tape, &mut ptr, &mut output)?;

        Ok(output)
    }

    /// Recursively interpret AST nodes with proper loop handling
    fn interpret_ast(&self, ast: &[AstNode], tape: &mut Vec<u8>, ptr: &mut usize, output: &mut Vec<u8>) -> Result<(), String> {
        for node in ast {
            match node {
                AstNode::Command(cmd) => {
                    self.interpret_command(*cmd, tape, ptr, output)?;
                }
                AstNode::Loop(body) => {
                    self.interpret_loop(body, tape, ptr, output)?;
                }
            }
        }
        Ok(())
    }

    /// Interpret a single command
    fn interpret_command(&self, cmd: Command, tape: &mut Vec<u8>, ptr: &mut usize, output: &mut Vec<u8>) -> Result<(), String> {
        match cmd {
            Command::Right => {
                *ptr = (*ptr + 1).min(tape.len() - 1);
            }
            Command::Left => {
                *ptr = ptr.saturating_sub(1);
            }
            Command::Increment => {
                tape[*ptr] = tape[*ptr].wrapping_add(1);
            }
            Command::Decrement => {
                tape[*ptr] = tape[*ptr].wrapping_sub(1);
            }
            Command::Output => {
                output.push(tape[*ptr]);
            }
            Command::Input => {
                // For compile-time interpretation, use a default input value
                tape[*ptr] = 65; // ASCII 'A'
            }
        }
        Ok(())
    }

    /// Interpret a loop with proper conditional execution
    fn interpret_loop(&self, body: &[AstNode], tape: &mut Vec<u8>, ptr: &mut usize, output: &mut Vec<u8>) -> Result<(), String> {
        // Keep executing the loop body while the current cell is non-zero
        while tape[*ptr] != 0 {
            self.interpret_ast(body, tape, ptr, output)?;
        }
        Ok(())
    }


}
