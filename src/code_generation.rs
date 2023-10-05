use std::collections::HashMap;
use crate::parser::Statement;

#[derive(Debug)]
pub enum CodeGenerationError {
    UndefinedSymbol(String),
    InvalidInstruction(String),
    Other(String),
}

pub fn generate_code(statements: &[Statement], symbol_table: &HashMap<String, i64>) -> Result<Vec<String>, CodeGenerationError> {
    let mut generated_code = Vec::new();

    for statement in statements {
        match statement {
            Statement::Add(x, y) => {
                let line = format!("ADD {}, {}", x, y);
                generated_code.push(line);
            },
            Statement::Mov(x, y) => {
                let line = format!("MOV {}, {}", x, y);
                generated_code.push(line);
            },
            Statement::Sub(x, y) => {
                let line = format!("SUB {}, {}", x, y);
                generated_code.push(line);
            },
            Statement::Mul(x, y) => {
                let line = format!("MUL {}, {}", x, y);
                generated_code.push(line);
            },
            Statement::Start(address) => {
                let line = format!("START {}", address);
                generated_code.push(line);
            },
            Statement::End => {
                generated_code.push("END".to_string());
            },
            Statement::Byte(bytes) => {
                let line = format!("BYTE {:?}", bytes);
                generated_code.push(line);
            },
            Statement::Word(word) => {
                let line = format!("WORD {}", word);
                generated_code.push(line);
            },
            Statement::Label(label) => {
                if let Some(address) = symbol_table.get(label) {
                    let line = format!("{}:", address);
                    generated_code.push(line);
                } else {
                    return Err(CodeGenerationError::UndefinedSymbol(label.clone()));
                }
            },
        }
    }
    Ok(generated_code)
}
