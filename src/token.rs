/// Represents a single Brainfuck token/command
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token {
    /// Increment data pointer (>)
    Right,
    /// Decrement data pointer (<)
    Left,
    /// Increment current cell (+)
    Increment,
    /// Decrement current cell (-)
    Decrement,
    /// Output current cell as ASCII (.)
    Output,
    /// Input ASCII character to current cell (,)
    Input,
    /// Start of loop ([)
    LoopStart,
    /// End of loop (])
    LoopEnd,
    /// End of file/input
    Eof,
}

impl Token {
    /// Convert a character to its corresponding Brainfuck token
    ///
    /// Returns Some(Token) for valid Brainfuck commands, None for comments/ignored characters
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '>' => Some(Token::Right),
            '<' => Some(Token::Left),
            '+' => Some(Token::Increment),
            '-' => Some(Token::Decrement),
            '.' => Some(Token::Output),
            ',' => Some(Token::Input),
            '[' => Some(Token::LoopStart),
            ']' => Some(Token::LoopEnd),
            _ => None, // All other characters are ignored (comments)
        }
    }

    /// Get the character representation of this token
    pub fn as_char(&self) -> char {
        match self {
            Token::Right => '>',
            Token::Left => '<',
            Token::Increment => '+',
            Token::Decrement => '-',
            Token::Output => '.',
            Token::Input => ',',
            Token::LoopStart => '[',
            Token::LoopEnd => ']',
            Token::Eof => '\0',
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_char())
    }
}
