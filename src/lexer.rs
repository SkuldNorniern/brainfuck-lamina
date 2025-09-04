use std::iter::Peekable;
use std::str::Chars;

/// Abstract Syntax Tree node types for Brainfuck
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AstNode {
    /// A basic Brainfuck command
    Command(Command),
    /// A loop containing nested nodes
    Loop(Vec<AstNode>),
}

/// Basic Brainfuck commands (excluding loop constructs)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    Right,      // >
    Left,       // <
    Increment,  // +
    Decrement,  // -
    Output,     // .
    Input,      // ,
}

/// Represents a position in the source code for error reporting
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

impl Position {
    pub fn new() -> Self {
        Self { line: 1, column: 1 }
    }

    pub fn advance(&mut self, c: char) {
        if c == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
    }
}

/// Error type for lexer operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LexerError {
    UnmatchedClosingBracket(Position),
    UnexpectedEndOfInput(Position),
}

impl std::fmt::Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexerError::UnmatchedClosingBracket(pos) => {
                write!(f, "Unmatched closing bracket ']' at line {}, column {}", pos.line, pos.column)
            }
            LexerError::UnexpectedEndOfInput(pos) => {
                write!(f, "Unexpected end of input while parsing loop at line {}, column {}", pos.line, pos.column)
            }
        }
    }
}

impl std::error::Error for LexerError {}

/// Result type for lexer operations
pub type Result<T> = std::result::Result<T, LexerError>;

/// Brainfuck lexer that converts source code into an AST
pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
    position: Position,
}

impl<'a> Lexer<'a> {
    /// Create a new lexer from source code
    pub fn new(source: &'a str) -> Self {
        Self {
            chars: source.chars().peekable(),
            position: Position::new(),
        }
    }

    /// Parse the entire source code into an AST
    pub fn parse(mut self) -> Result<Vec<AstNode>> {
        let mut nodes = Vec::new();

        while let Some(&c) = self.chars.peek() {
            if let Some(command) = self.parse_command(c) {
                self.chars.next();
                self.position.advance(c);
                nodes.push(AstNode::Command(command));
            } else if c == '[' {
                self.chars.next();
                self.position.advance(c);
                let loop_body = self.parse_loop()?;
                nodes.push(AstNode::Loop(loop_body));
            } else if c == ']' {
                // This should be handled by parse_loop, not here
                return Err(LexerError::UnmatchedClosingBracket(self.position));
            } else {
                // Skip comments and whitespace
                self.chars.next();
                self.position.advance(c);
            }
        }

        Ok(nodes)
    }

    /// Parse a single command character
    fn parse_command(&self, c: char) -> Option<Command> {
        match c {
            '>' => Some(Command::Right),
            '<' => Some(Command::Left),
            '+' => Some(Command::Increment),
            '-' => Some(Command::Decrement),
            '.' => Some(Command::Output),
            ',' => Some(Command::Input),
            _ => None,
        }
    }

    /// Parse a loop construct and its body
    fn parse_loop(&mut self) -> Result<Vec<AstNode>> {
        let mut nodes = Vec::new();

        while let Some(&c) = self.chars.peek() {
            if let Some(command) = self.parse_command(c) {
                self.chars.next();
                self.position.advance(c);
                nodes.push(AstNode::Command(command));
            } else if c == '[' {
                self.chars.next();
                self.position.advance(c);
                let nested_loop = self.parse_loop()?;
                nodes.push(AstNode::Loop(nested_loop));
            } else if c == ']' {
                self.chars.next();
                self.position.advance(c);
                return Ok(nodes);
            } else {
                // Skip comments and whitespace
                self.chars.next();
                self.position.advance(c);
            }
        }

        Err(LexerError::UnexpectedEndOfInput(self.position))
    }
}

/// Convenience function to parse Brainfuck source code into an AST
pub fn parse_brainfuck(source: &str) -> Result<Vec<AstNode>> {
    let lexer = Lexer::new(source);
    lexer.parse()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_commands() {
        let source = "+-><.,";
        let ast = parse_brainfuck(source).unwrap();

        assert_eq!(ast.len(), 6);
        assert_eq!(ast[0], AstNode::Command(Command::Increment));
        assert_eq!(ast[1], AstNode::Command(Command::Decrement));
        assert_eq!(ast[2], AstNode::Command(Command::Right));
        assert_eq!(ast[3], AstNode::Command(Command::Left));
        assert_eq!(ast[4], AstNode::Command(Command::Output));
        assert_eq!(ast[5], AstNode::Command(Command::Input));
    }

    #[test]
    fn test_parse_simple_loop() {
        let source = "[+-]";
        let ast = parse_brainfuck(source).unwrap();

        assert_eq!(ast.len(), 1);
        match &ast[0] {
            AstNode::Loop(body) => {
                assert_eq!(body.len(), 2);
                assert_eq!(body[0], AstNode::Command(Command::Increment));
                assert_eq!(body[1], AstNode::Command(Command::Decrement));
            }
            _ => panic!("Expected loop"),
        }
    }

    #[test]
    fn test_parse_nested_loops() {
        let source = "[+[+-]]";
        let ast = parse_brainfuck(source).unwrap();

        assert_eq!(ast.len(), 1);
        match &ast[0] {
            AstNode::Loop(outer_body) => {
                assert_eq!(outer_body.len(), 2);
                assert_eq!(outer_body[0], AstNode::Command(Command::Increment));
                match &outer_body[1] {
                    AstNode::Loop(inner_body) => {
                        assert_eq!(inner_body.len(), 2);
                        assert_eq!(inner_body[0], AstNode::Command(Command::Increment));
                        assert_eq!(inner_body[1], AstNode::Command(Command::Decrement));
                    }
                    _ => panic!("Expected nested loop"),
                }
            }
            _ => panic!("Expected loop"),
        }
    }

    #[test]
    fn test_parse_with_comments() {
        let source = "Hello + World - Test";
        let ast = parse_brainfuck(source).unwrap();

        assert_eq!(ast.len(), 2);
        assert_eq!(ast[0], AstNode::Command(Command::Increment));
        assert_eq!(ast[1], AstNode::Command(Command::Decrement));
    }

    #[test]
    fn test_unmatched_closing_bracket() {
        let source = "+]";
        let result = parse_brainfuck(source);
        assert!(matches!(result, Err(LexerError::UnmatchedClosingBracket(_))));
    }

    #[test]
    fn test_unexpected_end_of_input() {
        let source = "[+";
        let result = parse_brainfuck(source);
        assert!(matches!(result, Err(LexerError::UnexpectedEndOfInput(_))));
    }
}
