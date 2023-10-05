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

        let statements_result = parser::parse(&tokens);
        
        if let Ok(statements) = statements_result {
            println!("Parsed Statements: {:?}", statements);
            
            if statements.is_empty() && !tokens.is_empty() {
                println!("Tokens found but no statements parsed");
                continue;
            }

            let symbol_table_result = semantic_analysis::populate_symbol_table(&statements);

            if let Ok(symbol_table) = symbol_table_result {
                if let Err(e) = semantic_analysis::analyze(&statements, &symbol_table) {
                    println!("Semantic analysis failed: {:?}", e);
                } else {
                    println!("Semantic analysis passed");

                    let generated_code_result = code_generation::generate_code(&statements, &symbol_table);

                    if let Ok(generated_code) = generated_code_result {
                        println!("Generated Code: {:?}", generated_code);
                        
                        if generated_code.is_empty() && !statements.is_empty() {
                            println!("Statements found but no code generated");
                        }
                    } else {
                        println!("Code generation failed");
                    }
                }
            } else {
                println!("Failed to populate symbol table");
            }
        } else {
            println!("Parsing failed");
        }
    }

    Ok(())
}
