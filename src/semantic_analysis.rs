use crate::parser::Statement;

#[derive(Debug)]
pub enum SemanticError {
    InvalidOperand(String),
    InvalidInstruction(String),
}

pub fn analyze(statements: Vec<Statement>) -> Result<(), SemanticError> {
    for statement in &statements {
        match statement {
            Statement::Add(x, y) => {
                if !is_valid_operand(x) || !is_valid_operand(y) {
                    return Err(SemanticError::InvalidOperand(format!("Invalid operand in ADD: {}, {}", x, y)));
                }
            },
            Statement::Mov(x, y) => {
                if !is_valid_operand(x) || !is_valid_operand(y) {
                    return Err(SemanticError::InvalidOperand(format!("Invalid operand in MOV: {}, {}", x, y)));
                }
            },
        }
    }
    Ok(())
}

fn is_valid_operand(operand: &String) -> bool {
    operand == "X" || operand == "Y" || operand == "Z"
}
