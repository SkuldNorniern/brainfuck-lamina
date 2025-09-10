//! Configuration for Brainfuck compilation

/// Configuration for Brainfuck compilation
#[derive(Debug, Clone)]
pub struct BrainfuckConfig {
    /// Size of the memory tape (number of cells)
    pub tape_size: usize,
    /// Size of each memory cell in bytes (usually 1 for Brainfuck)
    pub cell_size: usize,
}

impl Default for BrainfuckConfig {
    fn default() -> Self {
        Self {
            tape_size: 30000, // normal brainfuck tape size
            cell_size: 1,     // 8-bit cells
        }
    }
}

impl BrainfuckConfig {
    /// Create a new configuration with custom values
    pub fn new(tape_size: usize, cell_size: usize) -> Self {
        Self {
            tape_size,
            cell_size,
        }
    }

    /// Create a configuration optimized for small programs
    pub fn small() -> Self {
        Self {
            tape_size: 1000,
            cell_size: 1,
        }
    }

    /// Create a configuration optimized for large programs
    pub fn large() -> Self {
        Self {
            tape_size: 100000,
            cell_size: 1,
        }
    }
}
