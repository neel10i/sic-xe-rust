use crate::parser::Statement;

#[derive(Debug)]
pub enum SemanticError {
    InvalidOperand(String),
    InvalidInstruction(String),
    InvalidDirectiveUsage(String), 
}

pub fn analyze(statements: Vec<Statement>) -> Result<(), SemanticError> {
    let mut start_found = false;
    let mut end_found = false;

    for statement in statements.iter() {
        match statement {
            Statement::Add(_, _) => {
                
            },
            Statement::Mov(_, _) => {
               
            },
            Statement::Start(_) => {
                if start_found {
                    return Err(SemanticError::InvalidDirectiveUsage("Duplicate START directive".to_string()));
                }
                start_found = true;
            },
            Statement::End => {
                if end_found {
                    return Err(SemanticError::InvalidDirectiveUsage("Duplicate END directive".to_string()));
                }
                end_found = true;
            },
            Statement::Byte(_) => {

            },
            Statement::Word(_) => {
                
            },
        }
    }

    if start_found && !end_found {
        return Err(SemanticError::InvalidDirectiveUsage("START directive found but no matching END".to_string()));
    }

    if !start_found && end_found {
        return Err(SemanticError::InvalidDirectiveUsage("END directive found but no matching START".to_string()));
    }

    Ok(())
}
