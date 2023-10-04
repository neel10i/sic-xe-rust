use crate::lexer::Token;

#[derive(Debug)]
pub enum Statement {
    Add(String, String),
    Mov(String, String),
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
            _ => {}
        }
    }
    statements
}
