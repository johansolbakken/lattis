use crate::lexer::{Token, TokenType};

#[derive(Debug, PartialEq)]
pub enum NodeType {
    Set,
    DataPoint,
    Definition,
    DefinitionList,
    Union,
    SetDifference,
    DataflowEquation,
    DataFlowEquationList,
    Body,
    Root,
}

#[derive(Debug)]
pub struct Node {
    node_type: NodeType,
    children: Vec<Node>,
    token: Option<Token>,
}

impl Node {
    pub fn print(&self, depth: usize) {
        let indent = " ".repeat(depth * 4);
        println!("{}{:?}", indent, self.node_type);
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    cursor: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, cursor: 0 }
    }

    fn expect(&self, token_type: TokenType) -> bool {
        self.tokens[self.cursor].token_type == token_type
    }

    fn peek_type(&self) -> TokenType {
        self.tokens[self.cursor].token_type.clone()
    }

    pub fn parse(&mut self) -> Node {
        let mut root = Node {
            node_type: NodeType::Root,
            children: Vec::new(),
            token: None,
        };
        while self.cursor < self.tokens.len() {
            let node = self.parse_data_flow_equation_list();
            root.children.push(node);
        }
        root
    }

    fn parse_data_flow_equation_list(&mut self) -> Node {
        let dataflow_equation = self.parse_data_flow_equation();
        if self.cursor < self.tokens.len() {
            return dataflow_equation;
        }
        let sub_list = self.parse_data_flow_equation_list();
        let node = Node {
            node_type: NodeType::DataFlowEquationList,
            children: vec![dataflow_equation, sub_list],
            token: None,
        };
        node
    }

    fn parse_data_flow_equation(&mut self) -> Node {
        let L = self.parse_data_point();
        assert!(self.expect(TokenType::Equals));
        self.cursor += 1;
        let body = self.parse_body();
        let node = Node {
            node_type: NodeType::DataflowEquation,
            children: vec![L, body],
            token: None,
        };
        node
    }

    fn parse_data_point(&mut self) -> Node {
        assert!(self.expect(TokenType::DataPoint));
        let mut node = Node {
            node_type: NodeType::DataPoint,
            children: Vec::new(),
            token: Some(self.tokens[self.cursor].clone()),
        };
        self.cursor += 1;
        node
    }

    fn parse_body(&mut self) -> Node {
        todo!()
    }

    fn parse_set(&mut self) -> Node {
        assert!(self.expect(TokenType::SetOpen));
        self.cursor += 1;
        let definition_list = self.parse_definition_list();
        assert!(self.expect(TokenType::SetClose));
        self.cursor += 1;
        let node = Node {
            node_type: NodeType::Set,
            children: definition_list,
            token: None,
        };
        node
    }

    fn parse_definition_list(&mut self) -> Vec<Node> {
        let mut definitions = Vec::new();
        while self.cursor < self.tokens.len() {
            let definition = self.parse_definition();
            if definition.is_none() {
                break;
            }
            definitions.push(definition.unwrap());
            if self.cursor < self.tokens.len() && self.expect(TokenType::Comma) {
                self.cursor += 1;
            }
        }
        definitions
    }

    fn parse_definition(&mut self) -> Option<Node> {
        if self.expect(TokenType::Definition) {
            let mut node = Node {
                node_type: NodeType::Definition,
                children: Vec::new(),
                token: Some(self.tokens[self.cursor].clone()),
            };
            self.cursor += 1;
            Some(node)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    #[test]
    fn test_parse_data_point() {
        let text = "L1".to_string();
        let mut lexer = Lexer::new(text);
        let tokens = lexer.lex_all();
        let mut parser = Parser::new(tokens);
        let root = parser.parse_data_point();
        assert_eq!(root.node_type, NodeType::DataPoint);
    }

    #[test]
    fn test_parse_empty_set() {
        let text = "{}".to_string();
        let mut lexer = Lexer::new(text);
        let tokens = lexer.lex_all();
        let mut parser = Parser::new(tokens);
        let root = parser.parse_set();
        assert_eq!(root.node_type, NodeType::Set);
        assert_eq!(root.children.len(), 0);
    }

    #[test]
    fn test_parse_definition() {
        let text = "d1".to_string();
        let mut lexer = Lexer::new(text);
        let tokens = lexer.lex_all();
        let mut parser = Parser::new(tokens);
        let root = parser.parse_definition().unwrap();
        assert_eq!(root.node_type, NodeType::Definition);
    }

    #[test]
    fn test_parse_definition_list() {
        let text = "d1, d2, d3".to_string();
        let mut lexer = Lexer::new(text);
        let tokens = lexer.lex_all();
        let mut parser = Parser::new(tokens);
        let root = parser.parse_definition_list();
        assert_eq!(root.len(), 3);
    }

    #[test]
    fn test_parse_set_with_definition() {
        let text = "{d1}".to_string();
        let mut lexer = Lexer::new(text);
        let tokens = lexer.lex_all();
        let mut parser = Parser::new(tokens);
        let root = parser.parse_set();
        assert_eq!(root.node_type, NodeType::Set);
        assert_eq!(root.children.len(), 1);
    }

    #[test]
    fn test_parse_set_with_definitions() {
        let text = "{d1, d2, d3}".to_string();
        let mut lexer = Lexer::new(text);
        let tokens = lexer.lex_all();
        let mut parser = Parser::new(tokens);
        let root = parser.parse_set();
        assert_eq!(root.node_type, NodeType::Set);
        assert_eq!(root.children.len(), 3);
    }
}
