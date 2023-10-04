use crate::parser::Statement;

pub fn generate_code(statements: Vec<Statement>) -> Vec<String> {
    let mut code = Vec::new();

    for statement in statements {
        match statement {
            Statement::Add(x, y) => {
                code.push(format!("ADD {},{}", x, y));
            },
            Statement::Mov(x, y) => {
                code.push(format!("MOV {},{}", x, y));
            },
            Statement::Start(address) => {
                code.push(format!(".START {}", address));
            },
            Statement::End => {
                code.push(String::from(".END"));
            },
            Statement::Byte(bytes) => {
                code.push(format!(".BYTE {:?}", bytes));
            },
            Statement::Word(value) => {
                code.push(format!(".WORD {}", value));
            },
        }
    }

    code
}
