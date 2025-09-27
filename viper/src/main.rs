use viper_lex::cursor::Cursor;
pub struct Lexer<'a> {
    cursor: Cursor<'a>,
}

#[derive(Debug, Clone)]
pub enum TokenKind {
    Number(f64),
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
    EOF,
}

impl<'a> Lexer<'a> {
    pub fn new(code: &'a [char]) -> Self {
        Lexer {
            cursor: Cursor::new(code),
        }
    }

    pub fn next_token(&mut self) -> Token {
        // пропускаем пробелы
        while self.cursor.peek().is_whitespace() {
            self.cursor.current += 1;
        }

        let start_pos = self.cursor.current;
        let ch = self.cursor.peek();

        if self.cursor.is_at_end() {
            return Token::new(TokenKind::EOF, "".to_string(), start_pos);
        }

        match ch {
            '+' => {
                self.cursor.current += 1;
                Token::new(TokenKind::Plus, "+".to_string(), start_pos)
            }
            '-' => {
                self.cursor.current += 1;
                Token::new(TokenKind::Minus, "-".to_string(), start_pos)
            }
            '*' => {
                self.cursor.current += 1;
                Token::new(TokenKind::Star, "*".to_string(), start_pos)
            }
            '/' => {
                self.cursor.current += 1;
                Token::new(TokenKind::Slash, "/".to_string(), start_pos)
            }
            '(' => {
                self.cursor.current += 1;
                Token::new(TokenKind::LParen, "(".to_string(), start_pos)
            }
            ')' => {
                self.cursor.current += 1;
                Token::new(TokenKind::RParen, ")".to_string(), start_pos)
            }
            '0'..='9' | '.' => {
                let mut num_str = String::new();
                let mut dot_count = 0;

                loop {
                    match self.cursor.peek() {
                        c @ '0'..='9' => {
                            num_str.push(c);
                            self.cursor.current += 1;
                        }
                        '.' => {
                            if dot_count == 0 {
                                dot_count += 1;
                                num_str.push('.');
                                self.cursor.current += 1;
                            } else {
                                break; // вторая точка → конец числа
                            }
                        }
                        _ => break,
                    }
                }

                let value: f64 = num_str.parse().unwrap_or(0.0);
                Token::new(TokenKind::Number(value), num_str, start_pos)
            }
            _ => {
                self.cursor.current += 1; // пропускаем неизвестный символ
                Token::new(TokenKind::EOF, "".to_string(), start_pos)
            }
        }
    }
}
