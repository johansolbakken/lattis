use lexer::Lexer;
use parser::Parser;

mod lexer;
mod node;
mod parser;

fn main() {
    let text = std::fs::read_to_string("oppg.txt").unwrap();
    let mut lexer = Lexer::new(text);
    let tokens = lexer.lex_all();
    let mut parser = Parser::new(tokens);
    let mut root = parser.parse();
    let root2 = node::simplify(&mut root);
    let s = root2.to_string(0);
    println!("{}", s);
}
