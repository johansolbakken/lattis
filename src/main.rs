use lexer::Lexer;
use parser::Parser;

mod lexer;
mod parser;

fn main() {
    let text = std::fs::read_to_string("oppg.txt").unwrap();
    let mut lexer = Lexer::new(text);
    let tokens = lexer.lex_all();
    let mut parser = Parser::new(tokens);
    let root = parser.parse();
    let s = root.to_string(0);
    println!("{}", s);
}
