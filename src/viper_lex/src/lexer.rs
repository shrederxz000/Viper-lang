// imports
use crate::cursor::Cursor;
use crate::tokens::*;
use std::collections::HashMap;
use std::path::PathBuf;
use viper_common::{address::Address, error, errors::Error};

/// Lexer structure
pub struct Lexer<'file_path, 'cursor> {
    line: u64,
    column: u16,
    cursor: Cursor<'cursor>,
    file_path: &'file_path PathBuf,
    tokens: Vec<Token>,
    keywords: HashMap<&'static str, TokenKind>,
}
/// Lexer implementation
impl<'file_path, 'cursor> Lexer<'file_path, 'cursor> {
    /// Creates new lexer from
    ///
    /// * `code`: source code represented as `&'cursor [char]`
    /// * `file_path`: source file path
    ///
    pub fn new(code: &'cursor [char], file_path: &'file_path PathBuf) -> Self {
        // Keywords list
        let keywords_map = HashMap::from([
            ("def", TokenKind::Def),
            ("lambda", TokenKind::Lambda),
            ("if", TokenKind::If),
            ("else", TokenKind::Else),
            ("elif", TokenKind::Elif),
            ("match", TokenKind::Match),
            ("case", TokenKind::Case),
            ("return", TokenKind::Return),
            ("while", TokenKind::While),
            ("for", TokenKind::For),
            ("in", TokenKind::In),
            ("continue", TokenKind::Continue),
            ("break", TokenKind::Break),
            ("with", TokenKind::With),
            ("type", TokenKind::Type),
            ("class", TokenKind::Class),
            ("metaclass", TokenKind::MetaClass),
            ("struct", TokenKind::Struct),
            ("record", TokenKind::Record),
            ("enum", TokenKind::Enum),
            ("iface", TokenKind::Iface),
            ("try", TokenKind::Try),
            ("as", TokenKind::As),
            ("from", TokenKind::From),
            ("pass", TokenKind::Pass),
            ("raise", TokenKind::Raise),
            ("yield", TokenKind::Yield),
            ("async", TokenKind::Async),
            ("await", TokenKind::Await),
            ("assert", TokenKind::Assert),
            ("global", TokenKind::Global),
            ("local", TokenKind::Local),
            ("nonlocal", TokenKind::NonLocal),
            ("execpt", TokenKind::Execpt),
            ("finally", TokenKind::Finally),
            ("import", TokenKind::Import),
            ("pack", TokenKind::Pack),
            ("priv", TokenKind::Priv),
            ("prot", TokenKind::Prot),
            ("static", TokenKind::Static),
            ("override", TokenKind::Override),
            ("int", TokenKind::Int),
            ("float", TokenKind::Float),
            ("decimal", TokenKind::Decimal),
            ("complex", TokenKind::Complex),
            ("char", TokenKind::Char),
            ("string", TokenKind::String),
            ("bool", TokenKind::Bool),
            ("bin", TokenKind::Bin),
            ("hex", TokenKind::Hex),
            ("octal", TokenKind::Octal),
            ("none", TokenKind::None),
            ("list", TokenKind::List),
            ("array", TokenKind::Array),
            ("tuple", TokenKind::Tuple),
            ("dict", TokenKind::Dict),
            ("set", TokenKind::Set),
        ]);
        // Lexer
        Lexer {
            line: 1,
            column: 0,
            cursor: Cursor::new(code),
            file_path,
            tokens: vec![],
            keywords: keywords_map,
        }
    }

    /// Converts source code represented as `&'cursor [char]`
    /// To a `Vec<Token>` - tokens list.
    #[allow(clippy::nonminimal_bool)]
    pub fn lex(mut self) -> Vec<Token> {
        if !self.tokens.is_empty() {
            panic!("tokens len already > 0. report this error to the developer.")
        }
        while !self.cursor.is_at_end() {
            let ch = self.advance();
            match ch {
                '+' => {
                    if self.is_match('=') {
                        self.add_tk(TokenKind::AssignAdd, "+=");
                    } else {
                        self.add_tk(TokenKind::Plus, "+");
                    }
                }
                '-' => {
                    if self.is_match('=') {
                        self.add_tk(TokenKind::AssignSub, "-=");
                    } else if self.is_match('>') {
                        self.add_tk(TokenKind::Arrow, "->");
                    } else {
                        self.add_tk(TokenKind::Minus, "-");
                    }
                }
                '*' => {
                    if self.is_match('=') {
                        self.add_tk(TokenKind::AssignMul, "*=");
                    } else if self.is_match('*') {
                        if self.is_match('=') {
                            self.add_tk(TokenKind::AssignPow, "**=");
                        } else {
                            self.add_tk(TokenKind::DoubleStar, "**");
                        }
                    } else {
                        self.add_tk(TokenKind::Star, "*");
                    }
                }
                '/' => {
                    if self.is_match('=') {
                        self.add_tk(TokenKind::AssignDiv, "/=");
                    } else if self.is_match('#') {
                        self.add_tk(TokenKind::CommentBlockClose, "/#");
                    } else {
                        self.add_tk(TokenKind::Slash, "/");
                    }
                }
                '\\' => {
                    self.add_tk(TokenKind::BackSlash, "\\");
                }
                '@' => {
                    self.add_tk(TokenKind::At, "@");
                }
                '$' => {
                    self.add_tk(TokenKind::Dollar, "$");
                }
                '%' => {
                    self.add_tk(TokenKind::Percent, "%");
                }
                '^' => {
                    self.add_tk(TokenKind::LogXOR, "^");
                }
                '&' => {
                    if self.is_match('&') {
                        self.add_tk(TokenKind::DoubleAmper, "&&");
                    } else {
                        self.add_tk(TokenKind::LogAnd, "&");
                    }
                }
                '|' => {
                    if self.is_match('|') {
                        self.add_tk(TokenKind::DoublePipe, "||");
                    } else {
                        self.add_tk(TokenKind::LogOr, "|");
                    }
                }
                '=' => {
                    if self.is_match('=') {
                        self.add_tk(TokenKind::Eq, "==");
                    } else if self.is_match('>') {
                        self.add_tk(TokenKind::ArrowImpl, "=>");
                    } else {
                        self.add_tk(TokenKind::Assign, "=");
                    }
                }
                '\'' => {
                    self.add_tk(TokenKind::Qoute, "\'");
                }
                '\"' => {
                    if self.is_match('\"') {
                        if self.is_match('\"') {
                            self.add_tk(TokenKind::TripleQuote, "\"\"\"");
                        }
                    } else {
                        self.add_tk(TokenKind::DoubleQuote, "\"");
                    }
                }
                '!' => {
                    self.add_tk(TokenKind::Bang, "!");
                }
                '?' => {
                    self.add_tk(TokenKind::Question, "?");
                }
                '<' => {
                    if self.is_match('=') {
                        self.add_tk(TokenKind::LessEq, "<=");
                    } else if self.is_match('<') {
                        self.add_tk(TokenKind::LShift, "<<");
                    } else {
                        self.add_tk(TokenKind::Less, "<");
                    }
                }
                '>' => {
                    if self.is_match('=') {
                        self.add_tk(TokenKind::GreaterEq, ">=");
                    } else if self.is_match('>') {
                        self.add_tk(TokenKind::RShift, ">>");
                    } else {
                        self.add_tk(TokenKind::Greater, ">");
                    }
                }
                '~' => {
                    self.add_tk(TokenKind::LogNot, "~");
                }
                ';' => {
                    self.add_tk(TokenKind::Semicolon, ";");
                }
                ':' => {
                    if self.is_match('=') {
                        self.add_tk(TokenKind::Walrus, ":=");
                    } else {
                        self.add_tk(TokenKind::Colon, ":");
                    }
                }
                ',' => {
                    self.add_tk(TokenKind::Comma, ",");
                }
                '.' => {
                    if self.is_match('.') {
                        if self.is_match('.') {
                            self.add_tk(TokenKind::Ellipsys, "...");
                        } else {
                            self.add_tk(TokenKind::Range, "..");
                        }
                    } else {
                        self.add_tk(TokenKind::Dot, ".");
                    }
                }
                '(' => {
                    self.add_tk(TokenKind::LParen, "(");
                }
                ')' => {
                    self.add_tk(TokenKind::RParen, ")");
                }
                '{' => {
                    self.add_tk(TokenKind::LBrace, "{");
                }
                '}' => {
                    self.add_tk(TokenKind::RBrace, "}");
                }
                '[' => {
                    self.add_tk(TokenKind::LBracket, "[");
                }
                ']' => {
                    self.add_tk(TokenKind::RBracket, "]");
                }
                '#' => {
                    if self.is_match('/') {
                        self.add_tk(TokenKind::CommentBlockOpen, "#/");
                    } else {
                        self.add_tk(TokenKind::Comment, "#");
                    }
                }
                _ => {}
            }
        }
        self.tokens
    }
    /*
            while !self.cursor.is_at_end() {
                let ch = self.advance();
                match ch {
                    '+' => {
                        if self.is_match('=') {
                            self.add_tk(TokenKind::AssignAdd, "+=");
                        } else {
                            self.add_tk(TokenKind::Plus, "+");
                        }
                    }
                    '&' => {
                        if self.is_match('&') {
                            self.add_tk(TokenKind::DoubleAmper, "&&");
                        } else {
                            self.add_tk(TokenKind::LogAnd, "&");
                        }
                    }
                    '|' => {
                        if self.is_match('|') {
                            self.add_tk(TokenKind::DoublePipe, "||");
                        } else {
                            self.add_tk(TokenKind::LogOr, "|");
                        }
                    }
                    '^' => {
                        self.add_tk(TokenKind::LogXOR, "^");
                    }
                    '-' => {
                        if self.is_match('=') {
                            self.add_tk(TokenKind::AssignSub, "-=");
                        } else if self.is_match('>') {
                            self.add_tk(TokenKind::Arrow, "->");
                        } else {
                            self.add_tk(TokenKind::Minus, "-");
                        }
                    }
                    '*' => {
                        if self.is_match('=') {
                            self.add_tk(TokenKind::AssignMul, "*=");
                        }
                        if self.is_match('*') {
                            self.add_tk(TokenKind::DoubleStar, "**");
                        } else {
                            self.add_tk(TokenKind::Star, "*");
                        }
                    }
                    '%' => {
                        self.add_tk(TokenKind::Percent, "%");
                    }
                    '/' => {
                        // compound operator
                        if self.is_match('=') {
                            self.add_tk(TokenKind::AssignDiv, "/=");
                        }
                        // line comment
                        else if self.is_match('/') {
                            while !self.is_match('\n') && !self.cursor.is_at_end() {
                                self.advance();
                            }
                            self.new_line();
                        }
                        // multi-line comment
                        else if self.is_match('*') {
                            while !(self.cursor.peek() == '*' && self.cursor.next() == '/')
                                && !self.cursor.is_at_end()
                            {
                                if self.is_match('\n') {
                                    self.new_line();
                                    continue;
                                }
                                self.advance();
                            }
                            // *
                            self.advance();
                            // /
                            self.advance();
                        } else {
                            self.add_tk(TokenKind::Slash, "/");
                        }
                    }
                    '(' => {
                        self.add_tk(TokenKind::LParen, "(");
                    }
                    ')' => {
                        self.add_tk(TokenKind::RParen, ")");
                    }
                    '{' => {
                        self.add_tk(TokenKind::LBrace, "{");
                    }
                    '}' => {
                        self.add_tk(TokenKind::RBrace, "}");
                    }
                    '[' => {
                        self.add_tk(TokenKind::LBracket, "[");
                    }
                    ']' => {
                        self.add_tk(TokenKind::RBracket, "]");
                    }
                    ',' => {
                        self.add_tk(TokenKind::Comma, ",");
                    }
                    '.' => {
                        if self.is_match('.') {
                            self.add_tk(TokenKind::Range, "..");
                        } else {
                            self.add_tk(TokenKind::Dot, ".");
                        }
                    }
                    '?' => {
                        self.add_tk(TokenKind::Question, "?");
                    }
                    ':' => {
                        if self.is_match('=') {
                            self.add_tk(TokenKind::Walrus, ":=");
                        } else {
                            self.add_tk(TokenKind::Colon, ":")
                        }
                    }
                    '<' => {
                        if self.is_match('=') {
                            self.add_tk(TokenKind::LessEq, "<=");
                        } else {
                            self.add_tk(TokenKind::Less, "<");
                        }
                    }
                    '>' => {
                        if self.is_match('=') {
                            self.add_tk(TokenKind::GreaterEq, ">=");
                        } else {
                            self.add_tk(TokenKind::Greater, ">");
                        }
                    }
                    '!' => {
                        if self.is_match('=') {
                            self.add_tk(TokenKind::NotEq, "!=");
                        } else {
                            self.add_tk(TokenKind::Bang, "!");
                        }
                    }
                    '=' => {
                        if self.is_match('=') {
                            self.add_tk(TokenKind::Eq, "==");
                        } else {
                            self.add_tk(TokenKind::Assign, "=");
                        }
                    }
                    '\r' => {}
                    '\t' => {}
                    '\0' => {}
                    ' ' => {}
                    '\n' => {
                        self.new_line();
                    }
                    '\'' => {
                        let tk = self.scan_string();
                        self.tokens.push(tk)
                    }
                    _ => {
                        // numbers
                        if self.is_digit(ch) {
                            // different number types scanning
                            let tk;
                            if self.cursor.peek() == 'x' {
                                tk = self.scan_hexadecimal_number();
                            } else if self.cursor.peek() == 'o' {
                                tk = self.scan_octal_number();
                            } else if self.cursor.peek() == 'b' {
                                tk = self.scan_binary_number();
                            } else {
                                tk = self.scan_number(ch);
                            }
                            self.tokens.push(tk);
                        }
                        // identifier
                        else if self.is_id(ch) {
                            let token = self.scan_id_or_keyword(ch);
                            self.tokens.push(token);
                        }
                        // unexpected
                        else {
                            error!(Error::own(
                                Address::new(self.line, self.column, self.file_path.clone(),),
                                format!("unexpected char: {ch}"),
                                format!("delete char: {ch}"),
                            ));
                        }
                    }
                }
            }
    */

    /// Scans string. Implies quote is already ate. East ending quote.
    fn scan_string(&mut self) -> Token {
        // String text
        let mut text: String = String::new();
        let span_start = self.column;

        while self.cursor.peek() != '\'' {
            let ch = self.advance();

            if ch == '\\' && self.cursor.peek() == '\'' {
                text.push(self.advance());
            } else {
                text.push(ch);
            }

            if self.cursor.is_at_end() || self.is_match('\n') {
                error!(Error::new(
                    Address::new(self.line, self.column, self.file_path.clone(),),
                    "unclosed string quotes.",
                    "did you forget ' symbol?",
                ));
            }
        }

        self.advance();
        let span_end = self.column;

        Token {
            tk_type: TokenKind::StringLiteral,
            value: text,
            address: Address::span(self.line, span_start..span_end, self.file_path.clone()),
        }
    }

    /// Scans decimal and integer numbers
    ///
    /// # Arguments
    /// * `start`: starting char of token
    ///
    fn scan_number(&mut self, start: char) -> Token {
        // Start of span
        let span_start = self.column;
        // Number text
        let mut text: String = String::from(start);
        // If number is float
        let mut is_float: bool = false;

        while self.is_digit(self.cursor.peek()) || self.cursor.peek() == '.' {
            if self.cursor.peek() == '.' {
                if self.cursor.next() == '.' {
                    break;
                }
                if is_float {
                    error!(Error::new(
                        Address::new(self.line, self.column, self.file_path.clone(),),
                        "couldn't parse number with two dots",
                        "check your code.",
                    ));
                }
                is_float = true;
                text.push(self.advance());

                continue;
            }
            text.push(self.advance());
            if self.cursor.is_at_end() {
                break;
            }
        }

        let span_end = self.column;

        Token {
            tk_type: TokenKind::NumberLiteral,
            value: text,
            address: Address::span(self.line, span_start..span_end, self.file_path.clone()),
        }
    }

    /// Scans hexadecimal numbers `0x{pattern}`
    fn scan_hexadecimal_number(&mut self) -> Token {
        // Start of span
        let span_start = self.column;
        // Skip 'x'
        self.advance();
        // Number text
        let mut text: String = String::from("0x");

        while self.cursor.peek().is_digit(16) {
            text.push(self.advance());
            if self.cursor.is_at_end() {
                break;
            }
        }

        let span_end = self.column;

        Token {
            tk_type: TokenKind::NumberLiteral,
            value: text,
            address: Address::span(self.line, span_start..span_end, self.file_path.clone()),
        }
    }

    /// Scans octal numbers `0o{pattern}`
    fn scan_octal_number(&mut self) -> Token {
        // Start of span
        let span_start = self.column;
        // Skip 'o'
        self.advance();
        // Number text
        let mut text: String = String::from("0o");

        while self.cursor.peek().is_digit(8) {
            text.push(self.advance());
            if self.cursor.is_at_end() {
                break;
            }
        }

        let span_end = self.column;

        Token {
            tk_type: TokenKind::NumberLiteral,
            value: text,
            address: Address::span(self.line, span_start..span_end, self.file_path.clone()),
        }
    }

    /// Scans binary numbers `0b{pattern}`
    fn scan_binary_number(&mut self) -> Token {
        // Start of span
        let span_start = self.column;
        // Skip 'b'
        self.advance();
        // Number text
        let mut text: String = String::from("0b");

        while self.cursor.peek().is_digit(2) {
            text.push(self.advance());
            if self.cursor.is_at_end() {
                break;
            }
        }

        let span_end = self.column;

        Token {
            tk_type: TokenKind::NumberLiteral,
            value: text,
            address: Address::span(self.line, span_start..span_end, self.file_path.clone()),
        }
    }

    /// Scans identifier, and checks if it is keyword.
    /// Returns token with kind Identifier or Keyword.
    ///
    /// # Arguments
    ///
    /// * `start`: starting char of token
    ///
    fn scan_id_or_keyword(&mut self, start: char) -> Token {
        // Start of span
        let span_start = self.column;
        // Id/keyword text
        let mut text: String = String::from(start);

        while self.is_id(self.cursor.peek()) {
            text.push(self.advance());
            if self.cursor.is_at_end() {
                break;
            }
        }

        let tk_type: TokenKind = self
            .keywords
            .get(text.as_str())
            .cloned()
            .unwrap_or(TokenKind::Id);

        let span_end = self.column;

        Token {
            tk_type,
            value: text,
            address: Address::span(self.line, span_start..span_end, self.file_path.clone()),
        }
    }

    /// Adds 1 to `line` and resets to zero `column`
    fn new_line(&mut self) {
        self.line += 1;
        self.column = 0;
    }

    /// Eats character from cursor and returns it,
    /// adding 1 to `column` and `cursor.current`
    fn advance(&mut self) -> char {
        let ch: char = self.cursor.char_at(0);
        self.cursor.current += 1;
        self.column += 1;
        ch
    }

    /// Checking current character is equal to `ch`
    /// If current character is equal to `ch` advances it
    #[allow(clippy::wrong_self_convention)]
    fn is_match(&mut self, ch: char) -> bool {
        if !self.cursor.is_at_end() && self.cursor.char_at(0) == ch {
            self.advance();
            return true;
        }
        false
    }

    /// Creates token from tk_type and tk_value, then adds it to the tokens list
    fn add_tk(&mut self, tk_type: TokenKind, tk_value: &str) {
        self.tokens.push(Token::new(
            tk_type,
            tk_value.to_string(),
            Address::new(self.line, self.column, self.file_path.clone()),
        ));
    }

    /// Checks character is '0..9'
    fn is_digit(&self, ch: char) -> bool {
        ch.is_ascii_digit()
    }

    /// Checks character is 'a..z', 'A..Z', '_'
    fn is_letter(&self, ch: char) -> bool {
        ch.is_ascii_lowercase() || ch.is_ascii_uppercase() || (ch == '_')
    }

    /// Returns true if character is id.
    ///
    /// Character is id, if:
    /// - char is letter
    /// - char is digit
    /// - char is colon and next char is id
    fn is_id(&self, ch: char) -> bool {
        self.is_letter(ch) || self.is_digit(ch) || (ch == ':' && self.is_id(self.cursor.next()))
    }
}
