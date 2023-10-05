use std::collections::HashMap;
use crate::parser::Statement;

#[derive(Debug)]
pub enum SemanticError {
    InvalidOperand(String),
    InvalidInstruction(String),
    InvalidDirectiveUsage(String),
    SymbolAlreadyDefined(String),
    SymbolNotFound(String),
}

pub fn populate_symbol_table(statements: &[Statement]) -> Result<HashMap<String, i64>, SemanticError> {
    let mut symbol_table = HashMap::new();
    let mut address_counter = 0;

    for statement in statements.iter() {
        match statement {
            Statement::Label(label) => {
                if symbol_table.contains_key(label) {
                    return Err(SemanticError::SymbolAlreadyDefined(label.clone()));
                }
                symbol_table.insert(label.clone(), address_counter);
            },
            
            _ => {
                address_counter += 1;
            },
        }
    }

    Ok(symbol_table)
}

pub fn analyze(statements: &[Statement], symbol_table: &HashMap<String, i64>) -> Result<(), SemanticError> {
    let mut start_found = false;
    let mut end_found = false;

    for statement in statements.iter() {
        match statement {
            Statement::Add(_, _) | Statement::Mov(_, _) => {
                
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
            Statement::Label(label) => {
                if !symbol_table.contains_key(label) {
                    return Err(SemanticError::SymbolNotFound(label.clone()));
                }
            },
            _ => {}
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
