//viper_lex/src/token.rs
#[derive(Debug, Clone, Copy, Hash)]
pub enum TokenType {
    PLUS,
    MINUS,
    DIV,
    MUL,

    NUMBER_LITERAL,
    STRING_LITERAL,

    NUMBER,
    STRING,
    LPAREN,
    RPAREN,
    ASSIGN,
    SEMICOLON,
    COLON,

    LET,
    ID,
    PRINT,
    EOF,
}

#[derive(Debug, Clone, Hash)]
pub struct Token {
    tk_type: TokenType,
    value: String,
    line: usize,
    column: usize,
}

impl Token {
    pub fn new(tk_type: TokenType, value: String, column: usize, line: usize) -> Self {
        Self {
            tk_type,
            value,
            line,
            column,
        }
    }
    pub fn get_type(&self) -> TokenType {
        return self.tk_type;
    }
    pub fn get_value(&self) -> &String {
        return &self.value;
    }
    pub fn get_pos(&self) -> [usize; 2] {
        let pos: [usize; 2] = [self.column, self.line];
        return pos;
    }
}
