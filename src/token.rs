#[derive(Debug, PartialEq)]
pub enum Special {
    Illegal,
    EOF,
}

#[derive(Debug, PartialEq)]
pub enum Value {
    Ident,
    Int,
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    Assign,
    Plus,
    Minus,
    Mul,
    Div,
    Lt,
    Gt,
    Eq,
    NotEq,
    Not,
}

#[derive(Debug, PartialEq)]
pub enum Delimiter {
    Comma,
    SemiColon,
}

#[derive(Debug, PartialEq)]
pub enum Paren {
    /// (
    LParen,
    /// )
    RParen,
    /// {
    LBrace,
    /// }
    RBrace,
}

#[derive(Debug, PartialEq)]
pub enum KeyWord {
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Special(Special),
    Value(Value),
    Operator(Operator),
    Delimiter(Delimiter),
    Paren(Paren),
    KeyWord(KeyWord),
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

pub fn get_token_type(literal: &String) -> TokenType {
    let num: Result<usize, _> = literal.parse();
    match literal.as_str() {
        "fn" => TokenType::KeyWord(KeyWord::Function),
        "let" => TokenType::KeyWord(KeyWord::Let),
        "true" => TokenType::KeyWord(KeyWord::True),
        "false" => TokenType::KeyWord(KeyWord::False),
        "if" => TokenType::KeyWord(KeyWord::If),
        "else" => TokenType::KeyWord(KeyWord::Else),
        "return" => TokenType::KeyWord(KeyWord::Return),
        _ => match num {
            Ok(_) => TokenType::Value(Value::Int),
            Err(_) => TokenType::Value(Value::Ident),
        },
    }
}
