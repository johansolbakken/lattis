use crate::lexer::{Token, TokenType};

#[derive(Debug)]
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

pub struct Node {
    node_type: NodeType,
    children: Vec<Node>,
    data: String,
}

impl Node {
    pub fn print(&self, depth: usize) {
        let indent = " ".repeat(depth * 4);
        println!("{}{:?} {}", indent, self.node_type, self.data);
        for child in &self.children {
            child.print(depth + 1);
        }
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
            data: String::new(),
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
            data: String::new(),
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
            data: String::new(),
        };
        node
    }

    fn parse_data_point(&mut self) -> Node {
        assert!(self.expect(TokenType::DataPoint));
        let mut node = Node {
            node_type: NodeType::DataPoint,
            children: Vec::new(),
            data: String::new(),
        };
        node.data = self.tokens[self.cursor].lexeme.clone();
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
            data: String::new(),
        };
        node
    }

    fn parse_definition_list(&mut self) -> Vec<Node> {
        let mut definitions = Vec::new();
        while self.expect(TokenType::Definition) {
            let definition = self.parse_definition();
            match definition {
                Some(node) => definitions.push(node),
                None => break,
            }
            if self.expect(TokenType::Comma) {
                self.cursor += 1;
            }
        }
        definitions
    }

    fn parse_definition(&mut self) -> Option<Node> {
        if self.expect(TokenType::Definition) {
            self.cursor += 1;
            let mut node = Node {
                node_type: NodeType::Definition,
                children: Vec::new(),
                data: String::new(),
            };
            node.data = self.tokens[self.cursor].lexeme.clone();
            self.cursor += 1;
            Some(node)
        } else {
            None
        }
    }
}
