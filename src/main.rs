mod lexer;
mod parser;
mod semantic_analysis;

fn main() {
    let line = "ADD X, Y";
    let tokens = lexer::tokenize_line(line);
    let statements = parser::parse(tokens);
    
    match semantic_analysis::analyze(statements) {
        Ok(_) => println!("Semantic analysis passed"),
        Err(error) => println!("Semantic analysis failed: {:?}", error),
    }
}
