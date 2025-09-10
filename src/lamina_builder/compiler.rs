//! Compiler functions for converting Brainfuck to various output formats

use crate::lexer::AstNode;
use super::config::BrainfuckConfig;
use super::ir_builder::BrainfuckIRBuilder;
use super::utils::count_operations;

/// Convert Brainfuck AST to Lamina IR
pub fn brainfuck_to_lamina_ir(ast: &[AstNode]) -> Result<String, String> {
    let builder = BrainfuckIRBuilder::new();
    let module = builder.build_ir(ast)?;
    Ok(module.to_string())
}

/// Convert Brainfuck AST to Lamina IR with custom configuration
pub fn brainfuck_to_lamina_ir_with_config(ast: &[AstNode], config: BrainfuckConfig) -> Result<String, String> {
    let builder = BrainfuckIRBuilder::with_config(config);
    let module = builder.build_ir(ast)?;
    Ok(module.to_string())
}

/// Convert Brainfuck AST to assembly code
pub fn brainfuck_to_assembly(ast: &[AstNode]) -> Result<String, String> {
    let builder = BrainfuckIRBuilder::new();
    let module = builder.build_ir(ast)?;
    
    // Convert module to IR string
    let ir_source = module.to_string();
    
    // Compile IR to assembly using Lamina
    let mut asm_buffer = Vec::new();
    match lamina::compile_lamina_ir_to_assembly(&ir_source, &mut asm_buffer) {
        Ok(_) => {
            // Convert assembly bytes to string
            match String::from_utf8(asm_buffer) {
                Ok(assembly) => Ok(assembly),
                Err(e) => Err(format!("Failed to convert assembly to string: {}", e))
            }
        }
        Err(e) => Err(format!("Lamina compilation failed: {}", e))
    }
}

/// Convert Brainfuck AST to assembly code with custom configuration
pub fn brainfuck_to_assembly_with_config(ast: &[AstNode], config: BrainfuckConfig) -> Result<String, String> {
    let builder = BrainfuckIRBuilder::with_config(config);
    let module = builder.build_ir(ast)?;
    
    // Convert module to IR string
    let ir_source = module.to_string();
    
    // Compile IR to assembly using Lamina
    let mut asm_buffer = Vec::new();
    match lamina::compile_lamina_ir_to_assembly(&ir_source, &mut asm_buffer) {
        Ok(_) => {
            // Convert assembly bytes to string
            match String::from_utf8(asm_buffer) {
                Ok(assembly) => Ok(assembly),
                Err(e) => Err(format!("Failed to convert assembly to string: {}", e))
            }
        }
        Err(e) => Err(format!("Lamina compilation failed: {}", e))
    }
}

/// Convert Brainfuck AST to binary executable
pub fn brainfuck_to_binary(ast: &[AstNode], output_path: &str) -> Result<String, String> {
    let builder = BrainfuckIRBuilder::new();
    let module = builder.build_ir(ast)?;

    // Convert module to IR string
    let ir_source = module.to_string();

    // Check if a .lamina file already exists (created by main.rs)
    let lamina_file = format!("{}.lamina", output_path);
    let lamina_file_exists = std::path::Path::new(&lamina_file).exists();

    if !lamina_file_exists {
        // Write IR to .lamina file only if it doesn't exist
        std::fs::write(&lamina_file, &ir_source)
            .map_err(|e| format!("Failed to write Lamina IR file: {}", e))?;
    }

    // Use the normal Lamina library to compile
    match compile_with_lamina_library(&ir_source, output_path) {
        Ok(_) => {
            // Only clean up if we created the file
            if !lamina_file_exists {
                let _ = std::fs::remove_file(&lamina_file);
            }
            Ok(format!("Binary executable created: {}", output_path))
        }
        Err(e) => {
            // Only clean up if we created the file
            if !lamina_file_exists {
                let _ = std::fs::remove_file(&lamina_file);
            }
            Err(e)
        }
    }
}

/// Convert Brainfuck AST to binary executable with custom configuration
pub fn brainfuck_to_binary_with_config(ast: &[AstNode], output_path: &str, config: BrainfuckConfig) -> Result<String, String> {
    let builder = BrainfuckIRBuilder::with_config(config);
    let module = builder.build_ir(ast)?;

    // Convert module to IR string
    let ir_source = module.to_string();

    // Check if a .lamina file already exists (created by main.rs)
    let lamina_file = format!("{}.lamina", output_path);
    let lamina_file_exists = std::path::Path::new(&lamina_file).exists();

    if !lamina_file_exists {
        // Write IR to .lamina file only if it doesn't exist
        std::fs::write(&lamina_file, &ir_source)
            .map_err(|e| format!("Failed to write Lamina IR file: {}", e))?;
    }

    // Use the normal Lamina library to compile
    match compile_with_lamina_library(&ir_source, output_path) {
        Ok(_) => {
            // Only clean up if we created the file
            if !lamina_file_exists {
                let _ = std::fs::remove_file(&lamina_file);
            }
            Ok(format!("Binary executable created: {}", output_path))
        }
        Err(e) => {
            // Only clean up if we created the file
            if !lamina_file_exists {
                let _ = std::fs::remove_file(&lamina_file);
            }
            Err(e)
        }
    }
}

/// Compile Lamina IR to executable using the Lamina library
fn compile_with_lamina_library(ir_source: &str, output_name: &str) -> Result<(), String> {
    use std::fs::File;
    use std::io::Write;

    // Compile IR to assembly using Lamina library
    let mut asm_buffer = Vec::new();
    match lamina::compile_lamina_ir_to_assembly(ir_source, &mut asm_buffer) {
        Ok(_) => {
            // Write assembly to file
            let asm_filename = format!("{}.s", output_name);
            let mut asm_file = File::create(&asm_filename)
                .map_err(|e| format!("Failed to create assembly file: {}", e))?;
            asm_file.write_all(&asm_buffer)
                .map_err(|e| format!("Failed to write assembly: {}", e))?;

            // Use system assembler and linker to create executable
            use std::process::Command;
            let output = Command::new("gcc")
                .arg(&asm_filename)
                .arg("-o")
                .arg(output_name)
                .arg("-no-pie")
                .output()
                .map_err(|e| format!("Failed to execute gcc: {}", e))?;

            if output.status.success() {
                // Clean up assembly file
                let _ = std::fs::remove_file(&asm_filename);
                Ok(())
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                Err(format!("GCC compilation failed: {}", stderr))
            }
        }
        Err(e) => Err(format!("Lamina compilation failed: {}", e))
    }
}

/// Generate a description of the IR that would be generated
pub fn brainfuck_to_ir_description(ast: &[AstNode]) -> Result<String, String> {
    let mut description = String::new();
    description.push_str("✅ Lamina IR Module Generated Successfully!\n");
    description.push_str("========================================\n\n");

    description.push_str("Module Structure:\n");
    description.push_str("- Main function: void main()\n");
    description.push_str("- Memory tape: heap-allocated array of 1,000 i8 values\n");
    description.push_str("- Data pointer: i32 index into tape\n");
    description.push_str("- I/O: Uses Lamina's writebyte/readbyte instructions\n\n");

    let (cmd_count, loop_count) = count_operations(ast);
    description.push_str(&format!("Operations to convert:\n"));
    description.push_str(&format!("- {} basic commands\n", cmd_count));
    description.push_str(&format!("- {} loops\n\n", loop_count));

    description.push_str("Memory Layout:\n");
    description.push_str("- tape: heap-allocated 1,000 i8 array (1,000 bytes)\n");
    description.push_str("- data_ptr: i32 variable tracking current position (0-999)\n");
    description.push_str("- temp variables: Generated as needed for operations\n\n");

    description.push_str("I/O Operations:\n");
    description.push_str("- Output: writebyte instruction writes i32 values to stdout\n");
    description.push_str("- Input: readbyte instruction reads i32 values from stdin\n\n");

    description.push_str("✅ Complete Brainfuck to Lamina IR conversion ready!\n");
    description.push_str("   All operations properly mapped to Lamina IR instructions.\n");

    Ok(description)
}

