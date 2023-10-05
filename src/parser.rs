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

pub fn parse(tokens: Vec<Token>) -> Vec<Statement> {
    let mut statements = Vec::new();
    let mut iter = tokens.iter().peekable();

    while let Some(token) = iter.next() {
        match token {
            Token::Keyword(k) if k == "ADD" => {
                if let Some(Token::Identifier(x)) = iter.next() {
                    if let Some(Token::Identifier(y)) = iter.next() {
                        statements.push(Statement::Add(x.clone(), y.clone()));
                    }
                }
            },
            Token::Keyword(k) if k == "MOV" => {
                if let Some(Token::Identifier(x)) = iter.next() {
                    if let Some(Token::Identifier(y)) = iter.next() {
                        statements.push(Statement::Mov(x.clone(), y.clone()));
                    }
                }
            },
            Token::Keyword(k) if k == "SUB" => {
                if let Some(Token::Identifier(x)) = iter.next() {
                    if let Some(Token::Identifier(y)) = iter.next() {
                        statements.push(Statement::Sub(x.clone(), y.clone()));
                    }
                }
            },
            Token::Keyword(k) if k == "MUL" => {
                if let Some(Token::Identifier(x)) = iter.next() {
                    if let Some(Token::Identifier(y)) = iter.next() {
                        statements.push(Statement::Mul(x.clone(), y.clone()));
                    }
                }
            },
            Token::Directive(d) if d == "START" => {
                if let Some(Token::Number(n)) = iter.next() {
                    statements.push(Statement::Start(*n));
                }
            },
            Token::Directive(d) if d == "END" => {
                statements.push(Statement::End);
            },
            Token::Directive(d) if d == "BYTE" => {
                if let Some(Token::Number(n)) = iter.next() {
                    statements.push(Statement::Byte(vec![*n as u8]));
                }
            },
            Token::Directive(d) if d == "WORD" => {
                if let Some(Token::Number(n)) = iter.next() {
                    statements.push(Statement::Word(*n));
                }
            },
            _ => {}
        }
    }
    statements
}
