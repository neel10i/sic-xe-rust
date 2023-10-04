#[derive(Debug, Clone)]
pub enum Token {
    Keyword(String),
    Identifier(String),
    Operator(String),
    Number(i64),
    Directive(String), 
}

pub fn tokenize_line(line: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut iter = line.chars().peekable();
    
    while let Some(&c) = iter.peek() {
        if c.is_whitespace() {
            iter.next();
        } 
        else if c.is_alphabetic() {
            let mut identifier = String::new();
            while let Some(&c) = iter.peek() {
                if c.is_alphabetic() || c.is_digit(10) {
                    identifier.push(c);
                    iter.next();
                } else {
                    break;
                }
            }
            
            if identifier == "MOV" || identifier == "ADD" {
                tokens.push(Token::Keyword(identifier));
            } else {
                tokens.push(Token::Identifier(identifier));
            }
        } else if c.is_digit(10) {
            let mut number = 0;
            while let Some(&c) = iter.peek() {
                if c.is_digit(10) {
                    number = number * 10 + c.to_digit(10).unwrap() as i64;
                    iter.next();
                } else {
                    break;
                }
            }
            tokens.push(Token::Number(number));
        } 
        else if c == '+' || c == '-' {
            let operator: String = iter.next().unwrap().to_string();
            tokens.push(Token::Operator(operator));
        } 
        else if c == '.' {
            let mut directive = String::new();
            iter.next();
            while let Some(&c) = iter.peek() {
                if c.is_alphabetic() {
                    directive.push(c);
                    iter.next();
                } 
                else {
                    break;
                }
            }
            tokens.push(Token::Directive(directive));
        } 
        else {
            iter.next();
        }
    }
    tokens
}

