use brainfuck_lamina::{parse_brainfuck, AstNode, Command};
use std::env;
use std::fs;
use std::process;

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
