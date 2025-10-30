use crate::token::{Token, TokenType};
struct Lexer {
    input: String,
    cursor: usize,
    line: usize,
    column: usize,
    current_ch: char,
}

impl Lexer {
    pub fn tokenize(&mut self, input: String) -> Option<Vec<Token>> {
        let current;
        if input.is_empty() {
            current = '\0';
        } else {
            current = input.as_bytes()[0] as char;
        }
        Self {
            input,
            cursor: 0,
            line: 1,
            column: 1,
            current_ch: current,
        };
        let mut tokens: Vec<Token> = Vec::new();
        // let mut buf: String = String::new();

        let mut token_line: usize = self.line;
        let mut token_column: usize = self.column;

        while current != '\0' {
            if current == '\0' {
                break;
            }
            match current {
                '+' => {
                    tokens.push(Token::new(
                        TokenType::PLUS,
                        "+".to_string(),
                        token_column,
                        token_line,
                    ));
                    self.advance();
                }
                '-' => {
                    tokens.push(Token::new(
                        TokenType::MINUS,
                        "-".to_string(),
                        token_column,
                        token_line,
                    ));
                    self.advance();
                }
                '*' => {
                    tokens.push(Token::new(
                        TokenType::MUL,
                        "*".to_string(),
                        token_column,
                        token_line,
                    ));
                    self.advance();
                }
                '/' => {
                    tokens.push(Token::new(
                        TokenType::DIV,
                        "/".to_string(),
                        token_column,
                        token_line,
                    ));
                    self.advance();
                }
                '=' => {
                    tokens.push(Token::new(
                        TokenType::ASSIGN,
                        "=".to_string(),
                        token_column,
                        token_line,
                    ));
                    self.advance();
                }
                '(' => {
                    tokens.push(Token::new(
                        TokenType::LPAREN,
                        "(".to_string(),
                        token_column,
                        token_line,
                    ));
                    self.advance();
                }
                ')' => {
                    tokens.push(Token::new(
                        TokenType::RPAREN,
                        ")".to_string(),
                        token_column,
                        token_line,
                    ));
                    self.advance();
                }
                ';' => {
                    tokens.push(Token::new(
                        TokenType::SEMICOLON,
                        ";".to_string(),
                        token_column,
                        token_line,
                    ));
                    self.advance();
                }
                ':' => {
                    tokens.push(Token::new(
                        TokenType::COLON,
                        ":".to_string(),
                        token_column,
                        token_line,
                    ));
                    self.advance();
                }
                '"' => {
                    tokens.push(Token::new(
                        TokenType::STRING_LITERAL,
                        self.consume_string('"')?,
                        token_column,
                        token_line,
                    ));
                }
                ' ' => {
                    self.advance();
                }
                '\n' => {
                    self.column = 1;
                    self.line += 1;
                }
                '\r' => {
                    self.column = 1;
                    self.line += 1;
                }
                '\t' => {
                    self.advance(); //эмулирую 4 пробела
                    self.advance();
                    self.advance();
                    self.advance();
                }
                _ => {
                    if self.current_ch.is_alphabetic() {
                        tokens.push(self.consume_id_or_keyword(token_column, token_line));
                    } else {
                        eprintln!(
                            "Unexpected token:{} at column:{}, line:{}",
                            self.current_ch, self.column, self.line
                        )
                    }
                }
            }
        }
        tokens.push(Token::new(
            TokenType::EOF,
            "\0".to_string(),
            token_column,
            token_line,
        ));
        return Some(tokens);
    }
    fn advance(&mut self) {
        self.cursor += 1;
        self.column += 1;
        if self.cursor >= self.input.len() {
            self.current_ch = '\0';
        } else {
            self.current_ch = self.input.as_bytes()[self.cursor] as char;
            if self.current_ch == '\n' || self.current_ch == '\r' {
                //при переходе строки сбрасываем колонку до первого символа и прибавляем строку
                self.column = 1;
                self.line += 1;
            }
        }
    }
    fn skip_whitespace(&mut self) {
        while self.current_ch == ' ' {
            self.advance();
        }
    }
    pub fn peek(&mut self) -> char {
        let next_pos: usize = self.cursor.clone() + 1;
        let res: char;
        if next_pos < self.input.len() {
            res = self.input.as_bytes()[next_pos].clone() as char;
        } else {
            res = '\0';
        }
        return res;
    }
    fn consume_number(&mut self) -> String {
        self.advance(); // пропустим открывающую кавычку
        let mut buf: String = String::new(); // создаем буфер
        while self.current_ch.is_numeric() {
            buf.push(self.current_ch); // добавляем значение
            self.advance();
        }
        return buf; // отдаем буфер
    }
    fn consume_string(&mut self, delimiter: char) -> Option<String> {
        self.advance();
        let mut buf: String = String::new();
        while self.current_ch != delimiter && self.current_ch != '\0' {
            buf.push(self.current_ch);
        }
        if self.current_ch == delimiter {
            self.advance();
        } else {
            eprintln!(
                "Unterminated string literal at line column:{}, line:{}",
                self.column, self.line
            );
        }
        return Some(buf);
    }
    fn consume_id_or_keyword(&mut self, tk_column: usize, tk_line: usize) -> Token {
        let mut buf: String = String::new();
        while self.current_ch.is_alphanumeric() {
            buf.push(self.current_ch);
            self.advance();
        }
        let result: &str = &buf;
        let tk_type: TokenType;

        match result {
            "let" => tk_type = TokenType::LET,
            "print" => tk_type = TokenType::PRINT,
            "string" => tk_type = TokenType::STRING,
            "number" => tk_type = TokenType::NUMBER,
            _ => tk_type = TokenType::ID,
        }
        return Token::new(tk_type, result.to_string(), tk_column, tk_line);
    }
}
