use lexer::Lexer;
use parser::Parser;

mod lexer;
mod node;
mod parser;
mod analysis;

fn generate_graph() -> parser::Node {
    let text = std::fs::read_to_string("oppg.txt").unwrap();
    let mut lexer = Lexer::new(text);
    let tokens = lexer.lex_all();
    let mut parser = Parser::new(tokens);
    let mut root = parser.parse();
    node::simplify(&mut root)
}

fn main() {
    let root = generate_graph();
    let iterations = analysis::reaching_definitions(&root);
}
