use crate::lexer::Token;

#[derive(Debug, Clone)]
pub enum Statement {
    Add(String, String),
    Mov(String, String),
    Sub(String, String),
    Mul(String, String),
    Start(i64),
    End,
    Byte(Vec<u8>),
    Word(i64),
    Label(String),
}

#[derive(Debug, Clone)]
pub enum ParseError {
    InvalidInstruction(String),
    InvalidOperand(String),
    MissingOperand(String),
    SyntaxError(String),
}

pub fn parse(tokens: &[Token]) -> Result<Vec<Statement>, ParseError> {
    let mut statements = Vec::new();
    let mut iter = tokens.iter().peekable();

    while let Some(token) = iter.next() {
        match token {
            Token::Keyword(k) if ["ADD", "MOV", "SUB", "MUL"].contains(&k.as_str()) => {
                if let Some(Token::Identifier(x)) = iter.next() {
                    if let Some(Token::Identifier(y)) = iter.next() {
                        match k.as_str() {
                            "ADD" => statements.push(Statement::Add(x.clone(), y.clone())),
                            "MOV" => statements.push(Statement::Mov(x.clone(), y.clone())),
                            "SUB" => statements.push(Statement::Sub(x.clone(), y.clone())),
                            "MUL" => statements.push(Statement::Mul(x.clone(), y.clone())),
                            _ => return Err(ParseError::InvalidInstruction(k.clone())),
                        }
                    } 
                    else {
                        return Err(ParseError::MissingOperand("Missing second operand".to_string()));
                    }
                } 
                else {
                    return Err(ParseError::MissingOperand("Missing first operand".to_string()));
                }
            },
            Token::Directive(d) if ["START", "END", "BYTE", "WORD"].contains(&d.as_str()) => {
                match d.as_str() {
                    "START" => {
                        if let Some(Token::Number(n)) = iter.next() {
                            statements.push(Statement::Start(*n));
                        } 
                        else {
                            return Err(ParseError::MissingOperand("Missing address for START".to_string()));
                        }
                    },
                    "END" => statements.push(Statement::End),
                    "BYTE" => {
                        if let Some(Token::Number(n)) = iter.next() {
                            statements.push(Statement::Byte(vec![*n as u8]));
                        }
                    },
                    "WORD" => {
                        if let Some(Token::Number(n)) = iter.next() {
                            statements.push(Statement::Word(*n));
                        }
                    },
                    _ => return Err(ParseError::InvalidInstruction(d.clone())),
                }
            },
            _ => return Err(ParseError::SyntaxError("Unexpected token".to_string())),
        }
    }
    Ok(statements)
}
