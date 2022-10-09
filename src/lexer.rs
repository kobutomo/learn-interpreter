use std::char;

use regex::Regex;

use crate::token;

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
}

pub fn new(input: String) -> Lexer {
    return Lexer {
        input,
        position: 0,
        read_position: 0,
    };
}

impl Lexer {
    pub fn next_token(&mut self) -> token::Token {
        use token::*;
        let char = match self.input.chars().nth(self.read_position) {
            Some(c) => c,
            None => '\0',
        };

        let tok = match char {
            '=' => match self.input.chars().nth(self.read_position + 1).unwrap() {
                '=' => {
                    self.position = self.read_position;
                    self.read_position += 1;
                    Token {
                        token_type: TokenType::Operator(Operator::Eq),
                        literal: "==".to_string(),
                    }
                }
                _ => Token {
                    token_type: TokenType::Operator(Operator::Assign),
                    literal: char.to_string(),
                },
            },
            '!' => match self.input.chars().nth(self.read_position + 1).unwrap() {
                '=' => {
                    self.position = self.read_position;
                    self.read_position += 1;
                    Token {
                        token_type: TokenType::Operator(Operator::NotEq),
                        literal: "!=".to_string(),
                    }
                }
                _ => Token {
                    token_type: TokenType::Operator(Operator::Not),
                    literal: char.to_string(),
                },
            },
            '+' => Token {
                token_type: TokenType::Operator(Operator::Plus),
                literal: char.to_string(),
            },
            '-' => Token {
                token_type: TokenType::Operator(Operator::Minus),
                literal: char.to_string(),
            },
            '/' => Token {
                token_type: TokenType::Operator(Operator::Div),
                literal: char.to_string(),
            },
            '*' => Token {
                token_type: TokenType::Operator(Operator::Mul),
                literal: char.to_string(),
            },
            '<' => Token {
                token_type: TokenType::Operator(Operator::Lt),
                literal: char.to_string(),
            },
            '>' => Token {
                token_type: TokenType::Operator(Operator::Gt),
                literal: char.to_string(),
            },
            ';' => Token {
                token_type: TokenType::Delimiter(Delimiter::SemiColon),
                literal: char.to_string(),
            },
            ',' => Token {
                token_type: TokenType::Delimiter(Delimiter::Comma),
                literal: char.to_string(),
            },
            '(' => Token {
                token_type: TokenType::Paren(Paren::LParen),
                literal: char.to_string(),
            },
            ')' => Token {
                token_type: TokenType::Paren(Paren::RParen),
                literal: char.to_string(),
            },
            '{' => Token {
                token_type: TokenType::Paren(Paren::LBrace),
                literal: char.to_string(),
            },
            '}' => Token {
                token_type: TokenType::Paren(Paren::RBrace),
                literal: char.to_string(),
            },
            '\0' => Token {
                token_type: TokenType::Special(Special::EOF),
                literal: "".to_string(),
            },
            ' ' | '\n' | '\t' | '\r' => {
                // 飛ばして次のトークンへ
                self.position = self.read_position;
                self.read_position += 1;
                let tok = self.next_token();
                // TODO: もうちょっといい感じにしたい（無駄にインクリメントした分を戻している）
                self.read_position -= 1;
                self.position = self.read_position;

                tok
            }
            _ => {
                if self.is_letter(char) {
                    let mut literal = "".to_string();
                    // skip(n) するとインデックスが n の値からイテレーションが開始する
                    for (i, c) in self.input.chars().enumerate().skip(self.read_position) {
                        if self.is_letter(c) {
                            self.position = self.read_position;
                            self.read_position = i;
                            literal += &c.to_string();
                        } else {
                            break;
                        }
                    }
                    Token {
                        token_type: get_token_type(&literal),
                        literal,
                    }
                } else {
                    Token {
                        token_type: TokenType::Special(Special::Illegal),
                        literal: "".to_string(),
                    }
                }
            }
        };
        self.position = self.read_position;
        self.read_position += 1;
        return tok;
    }

    fn is_letter(&self, c: char) -> bool {
        let re = Regex::new(r"[0-9a-zA-Z_]").unwrap();
        return re.is_match(&c.to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::new;
    use crate::token;

    #[test]
    fn next_token() {
        use token::*;
        let input = "
let five = 5;
let ten = 10;

let add = fn(x, y) {
    x + y;
};

let result = add(five, ten);
!-/*5;
5 < 10 > 5;

if (5 < 10) {
    return true;
} else {
    return false;
}

10 == 10;
10 != 9;
";

        let mut lexer = new(input.to_string());
        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::KeyWord(KeyWord::Let),
                literal: "let".to_string(),
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Value(Value::Ident),
                literal: "five".to_string(),
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Operator(Operator::Assign),
                literal: "=".to_string(),
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Value(Value::Int),
                literal: "5".to_string(),
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Delimiter(Delimiter::SemiColon),
                literal: ";".to_string(),
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::KeyWord(KeyWord::Let),
                literal: "let".to_string(),
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Value(Value::Ident),
                literal: "ten".to_string(),
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Operator(Operator::Assign),
                literal: "=".to_string(),
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Value(Value::Int),
                literal: "10".to_string(),
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Delimiter(Delimiter::SemiColon),
                literal: ";".to_string(),
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::KeyWord(KeyWord::Let),
                literal: "let".to_string(),
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Value(Value::Ident),
                literal: "add".to_string(),
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Operator(Operator::Assign),
                literal: "=".to_string(),
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::KeyWord(KeyWord::Function),
                literal: "fn".to_string(),
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Paren(Paren::LParen),
                literal: "(".to_string(),
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Value(Value::Ident),
                literal: "x".to_string(),
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Delimiter(Delimiter::Comma),
                literal: ",".to_string(),
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Value(Value::Ident),
                literal: "y".to_string(),
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Paren(Paren::RParen),
                literal: ")".to_string(),
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Paren(Paren::LBrace),
                literal: "{".to_string(),
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Value(Value::Ident),
                literal: "x".to_string(),
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Operator(Operator::Plus),
                literal: "+".to_string(),
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Value(Value::Ident),
                literal: "y".to_string(),
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Delimiter(Delimiter::SemiColon),
                literal: ";".to_string(),
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Paren(Paren::RBrace),
                literal: "}".to_string(),
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Delimiter(Delimiter::SemiColon),
                literal: ";".to_string(),
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::KeyWord(KeyWord::Let),
                literal: "let".to_string(),
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Value(Value::Ident),
                literal: "result".to_string(),
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Operator(Operator::Assign),
                literal: "=".to_string(),
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Value(Value::Ident),
                literal: "add".to_string(),
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Paren(Paren::LParen),
                literal: "(".to_string(),
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Value(Value::Ident),
                literal: "five".to_string(),
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Delimiter(Delimiter::Comma),
                literal: ",".to_string(),
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Value(Value::Ident),
                literal: "ten".to_string(),
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Paren(Paren::RParen),
                literal: ")".to_string(),
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Delimiter(Delimiter::SemiColon),
                literal: ";".to_string(),
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Operator(Operator::Not),
                literal: "!".to_string(),
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Operator(Operator::Minus),
                literal: "-".to_string(),
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Operator(Operator::Div),
                literal: "/".to_string(),
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Operator(Operator::Mul),
                literal: "*".to_string(),
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Value(Value::Int),
                literal: "5".to_string(),
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Delimiter(Delimiter::SemiColon),
                literal: ";".to_string(),
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Value(Value::Int),
                literal: "5".to_string(),
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Operator(Operator::Lt),
                literal: "<".to_string(),
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Value(Value::Int),
                literal: "10".to_string(),
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Operator(Operator::Gt),
                literal: ">".to_string(),
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Value(Value::Int),
                literal: "5".to_string(),
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Delimiter(Delimiter::SemiColon),
                literal: ";".to_string(),
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::KeyWord(KeyWord::If),
                literal: "if".to_string(),
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Paren(Paren::LParen),
                literal: "(".to_string(),
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Value(Value::Int),
                literal: "5".to_string(),
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Operator(Operator::Lt),
                literal: "<".to_string(),
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Value(Value::Int),
                literal: "10".to_string(),
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Paren(Paren::RParen),
                literal: ")".to_string(),
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Paren(Paren::LBrace),
                literal: "{".to_string(),
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::KeyWord(KeyWord::Return),
                literal: "return".to_string(),
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::KeyWord(KeyWord::True),
                literal: "true".to_string(),
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Delimiter(Delimiter::SemiColon),
                literal: ";".to_string(),
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Paren(Paren::RBrace),
                literal: "}".to_string(),
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::KeyWord(KeyWord::Else),
                literal: "else".to_string(),
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Paren(Paren::LBrace),
                literal: "{".to_string(),
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::KeyWord(KeyWord::Return),
                literal: "return".to_string(),
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::KeyWord(KeyWord::False),
                literal: "false".to_string(),
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Delimiter(Delimiter::SemiColon),
                literal: ";".to_string(),
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Paren(Paren::RBrace),
                literal: "}".to_string(),
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Value(Value::Int),
                literal: "10".to_string(),
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Operator(Operator::Eq),
                literal: "==".to_string(),
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Value(Value::Int),
                literal: "10".to_string(),
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Delimiter(Delimiter::SemiColon),
                literal: ";".to_string(),
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Value(Value::Int),
                literal: "10".to_string(),
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Operator(Operator::NotEq),
                literal: "!=".to_string(),
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Value(Value::Int),
                literal: "9".to_string(),
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Delimiter(Delimiter::SemiColon),
                literal: ";".to_string(),
            }
        );

        // EOF
        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Special(Special::EOF),
                literal: "".to_string(),
            }
        );
    }
}
