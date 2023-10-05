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

pub fn populate_symbol_table(statements: &[Statement]) -> Result<HashMap<String, i64>, Vec<SemanticError>> {
    let mut symbol_table = HashMap::new();
    let mut address_counter = 0;
    let mut errors = Vec::new(); 

    for statement in statements.iter() {
        match statement {
            Statement::Label(label) => {
                if symbol_table.contains_key(label) {
                    errors.push(SemanticError::SymbolAlreadyDefined(label.clone()));
                }
                symbol_table.insert(label.clone(), address_counter);
            },
            _ => {
                address_counter += 1;
            },
        }
    }

    if errors.is_empty() {
        Ok(symbol_table)
    } else {
        Err(errors)
    }
}

pub fn analyze(statements: &[Statement], symbol_table: &HashMap<String, i64>) -> Result<(), Vec<SemanticError>> {
    let mut errors = Vec::new(); 
    let mut start_found = false;
    let mut end_found = false;

    for statement in statements.iter() {
        match statement {
            Statement::Add(_, _) | Statement::Mov(_, _) => {
                
            },
            Statement::Start(_) => {
                if start_found {
                    errors.push(SemanticError::InvalidDirectiveUsage("Duplicate START directive".to_string()));
                }
                start_found = true;
            },
            Statement::End => {
                if end_found {
                    errors.push(SemanticError::InvalidDirectiveUsage("Duplicate END directive".to_string()));
                }
                end_found = true;
            },
            Statement::Label(label) => {
                if !symbol_table.contains_key(label) {
                    errors.push(SemanticError::SymbolNotFound(label.clone()));
                }
            },
            _ => {}
        }
    }

    if start_found && !end_found {
        errors.push(SemanticError::InvalidDirectiveUsage("START directive found but no matching END".to_string()));
    }

    if !start_found && end_found {
        errors.push(SemanticError::InvalidDirectiveUsage("END directive found but no matching START".to_string()));
    }

    if errors.is_empty() {
        Ok(())
    } 
    else {
        Err(errors)
    }
}

pub fn report_errors(errors: Vec<SemanticError>) {
    for error in errors {
        match error {
            SemanticError::InvalidOperand(s) => println!("Invalid operand: {}", s),
            SemanticError::InvalidInstruction(s) => println!("Invalid instruction: {}", s),
            SemanticError::InvalidDirectiveUsage(s) => println!("Invalid directive usage: {}", s),
            SemanticError::SymbolAlreadyDefined(s) => println!("Symbol already defined: {}", s),
            SemanticError::SymbolNotFound(s) => println!("Symbol not found: {}", s),
        }
    }
}
