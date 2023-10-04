use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

mod lexer;
mod parser;
mod semantic_analysis;
mod code_generation;

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut output = File::create("output.txt")?;

    for line in reader.lines() {
        let line = line?;

        let tokens = lexer::tokenize_line(&line);
        println!("Tokens: {:?}", tokens);

        let statements = parser::parse(tokens);
        println!("Parsed Statements: {:?}", statements);

        match semantic_analysis::analyze(statements.clone()) {
            Ok(_) => println!("Semantic analysis passed"),
            Err(error) => println!("Semantic analysis failed: {:?}", error),
        }

        let generated_code = code_generation::generate_code(statements);
        println!("Generated Code: {:?}", generated_code);

        for line in &generated_code {
            writeln!(output, "{}", line)?;
        }
    }

    Ok(())
}
