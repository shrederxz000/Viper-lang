// imports
use viper_common::address::Address;

/// Token kind
#[derive(Debug, Clone, Eq, PartialEq, Copy, Hash)]
#[allow(dead_code)]
pub enum TokenKind {
    // one char tokens
    Plus,        // +
    Minus,       // -
    Star,        // *
    Slash,       // /
    BackSlash,   // \
    At,          // @
    Dollar,      // $
    Percent,     // %
    LogXOR,      // ^
    LogAnd,      // &
    LogOr,       // |
    Assign,      // =
    Qoute,       // '
    DoubleQuote, // "
    Bang,        // !
    Question,    // ?
    Less,        // <
    Greater,     // >
    LogNot,      // ~
    // delimiters
    Semicolon, // ;
    Colon,     // :
    Comma,     // ,
    Dot,       // .
    LParen,    // (
    RParen,    // )
    LBrace,    // {
    RBrace,    // }
    LBracket,  // [
    RBracket,  // ]
    // two char tokens
    Arrow,       // ->
    ArrowImpl,   // =>
    Range,       // ..
    DoubleAmper, // &&
    DoubleStar,  // **
    DoublePipe,  // ||
    Walrus,      // :=
    Eq,          // ==
    NotEq,       // !=
    LessEq,      // <=
    GreaterEq,   // >=
    AssignAdd,   // +=
    AssignSub,   // -=
    AssignMul,   // *=
    AssignDiv,   // /=
    LShift,      // <<
    RShift,      // >>
    //tree char tokens
    TripleQuote, // """
    Ellipsys,    // ...
    AssignPow,   // **=
    // literals
    NumberLiteral,  // 12345.12345
    ComplexLiteral, // 24i
    CharLiteral,    // 'c'
    StringLiteral,  // "str"
    FStringLiteral, // f"str {var}"
    BstringLiteral, // b"str"
    BoolLiteral,    // true false
    BinLiteral,     // 0b111111
    HexLiteral,     // 0xFFFFF
    OctalLiteral,   // 0o7777
    NoneLiteral,    // none
    // idents
    Id,
    ContextId,
    // keywords
    Def,
    Lambda,
    If,
    Else,
    Elif,
    Match,
    Case,
    Return,
    While,
    For,
    In,
    Continue,
    Break,
    With,
    Type,
    Class,
    MetaClass,
    Struct,
    Record,
    Enum,
    Iface,
    Try,
    As,
    From,
    Pass,
    Raise,
    Yield,
    Async,
    Await,
    Assert,
    Global,
    Local,
    NonLocal,
    Execpt,
    Finally,
    Import,
    Pack,
    // modifiers
    Priv,
    Prot,
    Static,
    Override,
    // type hints
    Int,
    Float,
    Decimal,
    Complex,
    Char,
    String,
    Bool,
    Bin,
    Hex,
    Octal,
    None,
    List,
    Array,
    Tuple,
    Dict,
    Set,
    // etc
    Comment,           // #
    CommentBlockOpen,  // #/
    CommentBlockClose, // /#
    NewLine,           //  \n
    _ErrorToken_,      // error
    EOF,               // end of file
}

/// Token structure
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Token {
    pub tk_type: TokenKind,
    pub value: String,
    pub address: Address,
}
/// Token implementation
impl Token {
    /// Creates token from tk_type, value, address
    pub fn new(tk_type: TokenKind, value: String, address: Address) -> Token {
        Token {
            tk_type,
            value,
            address,
        }
    }
}
