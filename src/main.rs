use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

mod code_generation;
mod lexer;
mod parser;
mod semantic_analysis;

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        println!("Line: {}", line);

        let tokens = lexer::tokenize_line(&line);
        println!("Tokens: {:?}", tokens);

        let statements = parser::parse(tokens);
        println!("Parsed Statements: {:?}", statements);

        let symbol_table: HashMap<String, i64> = semantic_analysis::populate_symbol_table(&statements)
            .expect("Failed to populate symbol table");

        if let Err(e) = semantic_analysis::analyze(&statements, &symbol_table) {
            println!("Semantic analysis failed: {:?}", e);
        } 
        else {
            println!("Semantic analysis passed");
            
            let generated_code = code_generation::generate_code(&statements, &symbol_table);
            println!("Generated Code: {:?}", generated_code);
        }
    }

    Ok(())
}
