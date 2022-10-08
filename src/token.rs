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
        _ => match num {
            Ok(_) => TokenType::Value(Value::Int),
            Err(_) => TokenType::Value(Value::Ident),
        },
    }
}
