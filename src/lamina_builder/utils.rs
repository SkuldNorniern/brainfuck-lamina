//! Utility functions for the Lamina builder

use crate::lexer::AstNode;

/// Count the number of operations in the AST
pub fn count_operations(ast: &[AstNode]) -> (usize, usize) {
    let mut commands = 0;
    let mut loops = 0;

    for node in ast {
        match node {
            AstNode::Command(_) => commands += 1,
            AstNode::Loop(body) => {
                loops += 1;
                let (sub_commands, sub_loops) = count_operations(body);
                commands += sub_commands;
                loops += sub_loops;
            }
        }
    }

    (commands, loops)
}

