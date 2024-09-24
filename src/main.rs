use compiler::{interpreter::Interpreter, parser::Parser, token::Lexer};

mod compiler;
fn main() {
    use std::env;
    use std::fs;
    use std::process;

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file>", args[0]);
        process::exit(1);
    }

    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let lexer = Lexer::new(contents);
    let parser = Parser::new(lexer);
    let mut interpreter = Interpreter::new(parser);

    let result = interpreter.interpret();
    println!("Result: {}", result);
}
