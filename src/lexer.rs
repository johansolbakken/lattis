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
    text: Vec<u8>,
    cursor: usize,
}

impl Lexer {
    pub fn new(text: String) -> Lexer {
        Lexer {
            text: text.into_bytes(),
            cursor: 0,
        }
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
        // remove spaces from left
        self.skip_whitespace();

        if self.cursor >= self.text.len() {
            return Token {
                token_type: TokenType::Eof,
                lexeme: "EOF".to_string(),
            };
        }

        if self.text[self.cursor] == b'{' {
            self.cursor += 1;
            return Token {
                token_type: TokenType::SetOpen,
                lexeme: "{".to_string(),
            };
        }

        if self.text[self.cursor] == b'}' {
            self.cursor += 1;
            return Token {
                token_type: TokenType::SetClose,
                lexeme: "}".to_string(),
            };
        }

        if self.text[self.cursor] == b'L' {
            self.cursor += 1;
            return Token {
                token_type: TokenType::DataPoint,
                lexeme: "L".to_string() + &self.lex_number(),
            };
        }

        if self.text[self.cursor] == b'd' {
            self.cursor += 1;
            return Token {
                token_type: TokenType::Definition,
                lexeme: "d".to_string() + &self.lex_number(),
            };
        }

        if self.text[self.cursor] == b'=' {
            self.cursor += 1;
            return Token {
                token_type: TokenType::Equals,
                lexeme: "=".to_string(),
            };
        }

        if self.text[self.cursor] == b'U' {
            self.cursor += 1;
            return Token {
                token_type: TokenType::Union,
                lexeme: "U".to_string(),
            };
        }

        if self.text[self.cursor] == b'\\' {
            self.cursor += 1;
            return Token {
                token_type: TokenType::SetDifference,
                lexeme: "\\".to_string(),
            };
        }

        if self.text[self.cursor] == b',' {
            self.cursor += 1;
            return Token {
                token_type: TokenType::Comma,
                lexeme: ",".to_string(),
            };
        }

        if self.text[self.cursor] == b'\n' {
            self.cursor += 1;
            return Token {
                token_type: TokenType::NewLine,
                lexeme: "\n".to_string(),
            };
        }

        let token = Token {
            token_type: TokenType::Unkown,
            lexeme: self.text[self.cursor].to_string(),
        };
        self.cursor += 1;
        return token;
    }

    fn lex_number(&mut self) -> String {
        let mut number = String::new();
        while self.cursor < self.text.len() && self.text[self.cursor].is_ascii_digit() {
            number.push(self.text[self.cursor] as char);
            self.cursor += 1;
        }
        number
    }

    fn skip_whitespace(&mut self) {
        while self.cursor < self.text.len()
            && self.text[self.cursor].is_ascii_whitespace()
            && self.text[self.cursor] != b'\n'
        {
            self.cursor += 1;
        }
    }
}
