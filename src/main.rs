use brainfuck_lamina::{parse_brainfuck, AstNode, Command, brainfuck_to_lamina_ir, brainfuck_to_binary, lamina_builder::utils::count_operations};
use std::env;
use std::fs;
use std::process;
use std::path::{Path, PathBuf};

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
    let parent = path.parent();

    match parent {
        Some(parent_path) if !parent_path.as_os_str().is_empty() => {
            format!("{}/{}.lamina", parent_path.display(), stem)
        }
        _ => {
            // File is in current directory - use absolute path
            let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
            format!("{}/{}.lamina", current_dir.display(), stem)
        }
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

    

    // Generate Lamina IR Module

    let lamina_filename = generate_lamina_filename(filename);

    // Generate and save Lamina IR to file first
    match brainfuck_to_lamina_ir(&ast) {
        Ok(ir_source) => {
            match fs::write(&lamina_filename, &ir_source) {
                Ok(_) => {
                    //println!("Lamina IR saved to: {}", lamina_filename);
                }
                Err(err) => {
                    println!("Failed to save Lamina IR: {}", err);
                }
            }
        }
        Err(err) => {
            println!("Lamina IR Generation Failed: {}", err);
        }
    }

    // Generate executable using Lamina toolchain
    let binary_filename = generate_binary_filename(filename);
    match brainfuck_to_binary(&ast, &binary_filename) {
        Ok(result) => {
            println!("{}", result);
        }
        Err(err) => {
            println!("Executable Generation Failed: {}", err);
            println!("Lamina IR is saved at: {}", lamina_filename);
            println!("Try manual compilation: lamina {} -o {}", lamina_filename, binary_filename);
        }
    }


}


