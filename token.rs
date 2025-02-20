use phf;
use phf::phf_map;

#[derive(Debug, Clone, Copy)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    StringLiteral,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Equal,
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    And,
    Or,
}

impl TokenType {
    pub fn as_binary_operator(&self) -> Option<BinaryOperator> {
        match self {
            TokenType::Plus => Some(BinaryOperator::Add),
            TokenType::Minus => Some(BinaryOperator::Subtract),
            TokenType::Star => Some(BinaryOperator::Multiply),
            TokenType::Slash => Some(BinaryOperator::Divide),
            TokenType::EqualEqual => Some(BinaryOperator::Equal),
            TokenType::BangEqual => Some(BinaryOperator::NotEqual),
            TokenType::Greater => Some(BinaryOperator::Greater),
            TokenType::GreaterEqual => Some(BinaryOperator::GreaterEqual),
            TokenType::Less => Some(BinaryOperator::Less),
            TokenType::LessEqual => Some(BinaryOperator::LessEqual),
            TokenType::And => Some(BinaryOperator::And),
            TokenType::Or => Some(BinaryOperator::Or),
            _ => None,
        }
    }
}


#[derive(Debug, Clone)]
pub enum Literal {
    Identifier(String),
    Str(String),
    Number(f64),
    Boolean(bool),
}

#[derive(Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<Literal>,
    line: u32,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Option<Literal>, line: u32) -> Self {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}


pub static KEYWORDS: phf::Map<&'static str, TokenType> = phf_map! {
    "and"    => TokenType::And,
    "or"     => TokenType::Or,
    "nil"    => TokenType::Nil,
    "class"  => TokenType::Class,
    "for"    => TokenType::For,
    "while"  => TokenType::While,
    "var"    => TokenType::Var,
    "fun"    => TokenType::Fun,
    "if"     => TokenType::If,
    "else"   => TokenType::Else,
    "print"  => TokenType::Print,
    "return" => TokenType::Return,
    "super"  => TokenType::Super,
    "this"   => TokenType::This,
    "true"   => TokenType::True,
    "false"  => TokenType::False,
};
