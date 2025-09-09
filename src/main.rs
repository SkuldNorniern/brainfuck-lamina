use brainfuck_lamina::{parse_brainfuck, AstNode, Command, brainfuck_to_lamina_ir, brainfuck_to_binary, brainfuck_to_ir_description};
use std::env;
use std::fs;
use std::process;
use std::path::Path;

/// Print the AST in a human-readable format
fn print_ast(nodes: &[AstNode], indent: usize) {
    let indent_str = "  ".repeat(indent);

    for node in nodes {
        match node {
            AstNode::Command(cmd) => {
                println!("{}{}", indent_str, format_command(*cmd));
            }
            AstNode::Loop(body) => {
                println!("{}Loop [", indent_str);
                print_ast(body, indent + 1);
                println!("{}]", indent_str);
            }
        }
    }
}

/// Format a command for display
fn format_command(cmd: Command) -> &'static str {
    match cmd {
        Command::Right => "Right (>)",
        Command::Left => "Left (<)",
        Command::Increment => "Increment (+)",
        Command::Decrement => "Decrement (-)",
        Command::Output => "Output (.)",
        Command::Input => "Input (,)",
    }
}

/// Print usage information
fn print_usage() {
    eprintln!("Usage: brainfuck-lamina <filename>");
    eprintln!("  filename: Path to Brainfuck (.bf or .b) source file");
}

/// Generate the output filename for the .lamina file
fn generate_lamina_filename(input_filename: &str) -> String {
    let path = Path::new(input_filename);
    let stem = path.file_stem().unwrap_or_default().to_string_lossy();
    let parent = path.parent().unwrap_or(Path::new(""));
    
    if parent.to_string_lossy().is_empty() {
        format!("{}.lamina", stem)
    } else {
        format!("{}/{}.lamina", parent.display(), stem)
    }
}

/// Generate the output filename for the binary executable
fn generate_binary_filename(input_filename: &str) -> String {
    let path = Path::new(input_filename);
    let stem = path.file_stem().unwrap_or_default().to_string_lossy();
    let parent = path.parent().unwrap_or(Path::new(""));
    
    let binary_name = if cfg!(windows) {
        format!("{}.exe", stem)
    } else {
        stem.to_string()
    };
    
    if parent.to_string_lossy().is_empty() {
        binary_name
    } else {
        format!("{}/{}", parent.display(), binary_name)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // Check for correct number of arguments
    if args.len() != 2 {
        eprintln!("Error: Expected exactly one argument (filename)");
        print_usage();
        process::exit(1);
    }

    let filename = &args[1];

    // Read the file
    let source = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading file '{}': {}", filename, err);
            process::exit(1);
        }
    };

    // Parse the Brainfuck code
    let ast = match parse_brainfuck(&source) {
        Ok(nodes) => nodes,
        Err(err) => {
            eprintln!("Parse error in '{}': {}", filename, err);
            process::exit(1);
        }
    };

    // Print the AST
    println!("Brainfuck AST for '{}':", filename);
    println!("========================================");
    print_ast(&ast, 0);

    // Print summary statistics
    let (command_count, loop_count) = count_nodes(&ast);
    println!("========================================");
    println!("Summary: {} commands, {} loops", command_count, loop_count);

    // Generate Lamina IR Module
    println!("\n🔄 Lamina IR Generation");
    println!("========================================");

    let lamina_filename = generate_lamina_filename(filename);
    println!("Debug: Generated filename: '{}'", lamina_filename);

    // Generate and save Lamina IR to file first
    println!("Debug: About to call brainfuck_to_lamina_ir");
    match brainfuck_to_lamina_ir(&ast) {
        Ok(ir_source) => {
            println!("Debug: IR generation succeeded, IR length: {}", ir_source.len());
            println!("Debug: Target filename: {}", lamina_filename);
            println!("Debug: About to write {} bytes to file", ir_source.len());
            println!("Debug: First 200 chars of IR: {}", &ir_source[..200.min(ir_source.len())]);
            println!("Debug: Attempting to write to: {}", lamina_filename);
            match fs::write(&lamina_filename, &ir_source) {
                Ok(_) => {
                    println!("✅ Lamina IR saved to: {}", lamina_filename);
                    // Verify the file was actually created
                    match fs::metadata(&lamina_filename) {
                        Ok(metadata) => {
                            println!("Debug: File created successfully, size: {} bytes", metadata.len());
                        }
                        Err(err) => {
                            println!("Debug: File metadata check failed: {}", err);
                        }
                    }
                }
                Err(err) => {
                    println!("❌ Failed to save Lamina IR: {}", err);
                    println!("Debug: Error details: {:?}", err);
                }
            }
        }
        Err(err) => {
            println!("❌ Lamina IR Generation Failed: {}", err);
        }
    }

    // Generate executable using Lamina toolchain
    println!("\n🔄 Executable Generation");
    println!("========================================");

    let binary_filename = generate_binary_filename(filename);
    match brainfuck_to_binary(&ast, &binary_filename) {
        Ok(result) => {
            println!("✅ {}", result);
        }
        Err(err) => {
            println!("❌ Executable Generation Failed: {}", err);
            println!("💡 Lamina IR is saved at: {}", lamina_filename);
            println!("💡 Try manual compilation: lamina {} -o {}", lamina_filename, binary_filename);
        }
    }

    // Also show detailed description
    println!("\n📋 Detailed IR Description");
    println!("========================================");
    match brainfuck_to_ir_description(&ast) {
        Ok(description) => {
            println!("{}", description);
        }
        Err(err) => {
            println!("❌ Description Generation Failed: {}", err);
        }
    }
}


/// Count the total number of commands and loops in the AST
fn count_nodes(nodes: &[AstNode]) -> (usize, usize) {
    let mut commands = 0;
    let mut loops = 0;

    for node in nodes {
        match node {
            AstNode::Command(_) => commands += 1,
            AstNode::Loop(body) => {
                loops += 1;
                let (sub_commands, sub_loops) = count_nodes(body);
                commands += sub_commands;
                loops += sub_loops;
            }
        }
    }

    (commands, loops)
}
