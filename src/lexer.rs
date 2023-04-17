#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Unkown,
    SetOpen,       // {
    SetClose,      // }
    DataPoint,     // L
    Definition,    // d
    Equals,        // =
    Union,         // U
    SetDifference, // \
    Comma,         // ,
    NewLine,       // \n
    Eof,           // EOF
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
}

pub struct Lexer {
    text: String,
    cursor: usize,
}

impl Lexer {
    pub fn new(text: String) -> Lexer {
        Lexer { text, cursor: 0 }
    }

    fn current(&self) -> char {
        self.text
            .get(self.cursor..self.cursor + 1)
            .unwrap()
            .chars()
            .next()
            .unwrap()
    }

    pub fn lex_all(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while self.cursor < self.text.len() {
            let token = self.lex();
            tokens.push(token);
        }
        tokens
    }

    fn lex(&mut self) -> Token {
        if self.cursor >= self.text.len() {
            return Token {
                token_type: TokenType::Eof,
                lexeme: "EOF".to_string(),
            };
        }

        if self.current() == ';' {
            self.cursor += 1;
            return Token {
                token_type: TokenType::NewLine,
                lexeme: ";".to_string(),
            };
        }

        self.skip_whitespace();

        if self.cursor >= self.text.len() {
            return Token {
                token_type: TokenType::Eof,
                lexeme: "EOF".to_string(),
            };
        }

        if self.current() == '{' {
            self.cursor += 1;
            return Token {
                token_type: TokenType::SetOpen,
                lexeme: "{".to_string(),
            };
        }

        if self.current() == '}' {
            self.cursor += 1;
            return Token {
                token_type: TokenType::SetClose,
                lexeme: "}".to_string(),
            };
        }

        if self.current() == 'L' {
            self.cursor += 1;
            return Token {
                token_type: TokenType::DataPoint,
                lexeme: "L".to_string() + &self.lex_number(),
            };
        }

        if self.current() == 'd' {
            self.cursor += 1;
            return Token {
                token_type: TokenType::Definition,
                lexeme: "d".to_string() + &self.lex_number(),
            };
        }

        if self.current() == '=' {
            self.cursor += 1;
            return Token {
                token_type: TokenType::Equals,
                lexeme: "=".to_string(),
            };
        }

        if self.current() == 'U' {
            self.cursor += 1;
            return Token {
                token_type: TokenType::Union,
                lexeme: "U".to_string(),
            };
        }

        if self.current() == '/' {
            self.cursor += 1;
            return Token {
                token_type: TokenType::SetDifference,
                lexeme: "/".to_string(),
            };
        }

        if self.current() == ',' {
            self.cursor += 1;
            return Token {
                token_type: TokenType::Comma,
                lexeme: ",".to_string(),
            };
        }

        let token = Token {
            token_type: TokenType::Unkown,
            lexeme: self.current().to_string(),
        };
        self.cursor += 1;
        return token;
    }

    fn lex_number(&mut self) -> String {
        let mut number = String::new();
        while self.cursor < self.text.len() && self.current().is_ascii_digit() {
            number.push(self.current() as char);
            self.cursor += 1;
        }
        number
    }

    fn skip_whitespace(&mut self) {
        while self.cursor < self.text.len() && self.current().is_ascii_whitespace() {
            self.cursor += 1;
        }
    }
}
