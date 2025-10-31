//viper_lex/src/lexer.rs
use crate::token::{Token, TokenType};
#[derive(Debug, Hash)]
pub struct Lexer {
    input: String,
    cursor: usize,
    line: usize,
    column: usize,
    current_ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let current = input.chars().nth(0).unwrap_or('\0');
        Self {
            input,
            cursor: 0,
            line: 1,
            column: 1,
            current_ch: current,
        }
    }
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();

        while self.current_ch != '\0' {
            let token_line = self.line;
            let token_column = self.column;

            match self.current_ch {
                '+' => self.push_simple_token(
                    TokenType::PLUS,
                    "+",
                    &mut tokens,
                    token_column,
                    token_line,
                ),
                '-' => self.push_simple_token(
                    TokenType::MINUS,
                    "-",
                    &mut tokens,
                    token_column,
                    token_line,
                ),
                '*' => self.push_simple_token(
                    TokenType::MUL,
                    "*",
                    &mut tokens,
                    token_column,
                    token_line,
                ),
                '/' => self.push_simple_token(
                    TokenType::DIV,
                    "/",
                    &mut tokens,
                    token_column,
                    token_line,
                ),
                '=' => self.push_simple_token(
                    TokenType::ASSIGN,
                    "=",
                    &mut tokens,
                    token_column,
                    token_line,
                ),
                '(' => self.push_simple_token(
                    TokenType::LPAREN,
                    "(",
                    &mut tokens,
                    token_column,
                    token_line,
                ),
                ')' => self.push_simple_token(
                    TokenType::RPAREN,
                    ")",
                    &mut tokens,
                    token_column,
                    token_line,
                ),
                ';' => self.push_simple_token(
                    TokenType::SEMICOLON,
                    ";",
                    &mut tokens,
                    token_column,
                    token_line,
                ),
                ':' => self.push_simple_token(
                    TokenType::COLON,
                    ":",
                    &mut tokens,
                    token_column,
                    token_line,
                ),

                '"' => {
                    let s = self.consume_string('"');
                    tokens.push(Token::new(
                        TokenType::STRING_LITERAL,
                        s,
                        token_column,
                        token_line,
                    ));
                }

                c if c.is_numeric() => {
                    let num = self.consume_number();
                    tokens.push(Token::new(
                        TokenType::NUMBER_LITERAL,
                        num,
                        token_column,
                        token_line,
                    ));
                }

                ' ' | '\t' => self.advance(),
                '\n' | '\r' => {
                    self.advance();
                    self.line += 1;
                    self.column = 1;
                }

                _ if self.current_ch.is_alphabetic() => {
                    tokens.push(self.consume_id_or_keyword(token_column, token_line));
                }

                _ => {
                    eprintln!(
                        "Unexpected token '{}' at line {}, column {}",
                        self.current_ch, self.line, self.column
                    );
                    self.advance();
                }
            }
        }
        tokens.push(Token::new(
            TokenType::EOF,
            "\0".to_string(),
            self.column,
            self.line,
        ));
        tokens
    }

    fn push_simple_token(
        &mut self,
        tk_type: TokenType,
        value: &str,
        tokens: &mut Vec<Token>,
        column: usize,
        line: usize,
    ) {
        tokens.push(Token::new(tk_type, value.to_string(), column, line));
        self.advance();
    }

    fn advance(&mut self) {
        self.cursor += 1;

        if let Some(ch) = self.input.chars().nth(self.cursor) {
            self.current_ch = ch;
        } else {
            self.current_ch = '\0';
        }
        self.column += 1;
    }

    pub fn peek(&self) -> char {
        self.input.chars().nth(self.cursor + 1).unwrap_or('\0')
    }

    fn consume_number(&mut self) -> String {
        let mut buf = String::new();
        while self.current_ch.is_ascii_digit() {
            buf.push(self.current_ch);
            self.advance();
        }
        buf
    }

    fn consume_string(&mut self, delimiter: char) -> String {
        self.advance(); // пропускаем открывающую кавычку
        let mut buf = String::new();
        while self.current_ch != delimiter && self.current_ch != '\0' {
            buf.push(self.current_ch);
            self.advance();
        }
        if self.current_ch == delimiter {
            self.advance(); // закрывающая кавычка
        } else {
            eprintln!(
                "Unterminated string literal at line {}, column {}",
                self.line, self.column
            );
        }
        buf
    }

    fn consume_id_or_keyword(&mut self, tk_column: usize, tk_line: usize) -> Token {
        let mut buf = String::new();
        while self.current_ch.is_alphanumeric() {
            buf.push(self.current_ch);
            self.advance();
        }

        let tk_type = match buf.as_str() {
            "let" => TokenType::LET,
            "print" => TokenType::PRINT,
            "string" => TokenType::STRING,
            "number" => TokenType::NUMBER,
            _ => TokenType::ID,
        };

        Token::new(tk_type, buf, tk_column, tk_line)
    }
}
