#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Ident(String),

    Plus,
    Minus,
    Mul,
    Div,
    Pow,
    And,
    Or,
    Shl,
    Shr,
    Assign,

    LParen,
    RParen,
    Comma,
}

pub struct Lexer;

impl Lexer {
    pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        let mut chars = input.chars().peekable();

        while let Some(&c) = chars.peek() {
            match c {
                '0'..='9' | '.' => {
                    let mut num = String::new();
                    while let Some(&d) = chars.peek() {
                        if d.is_ascii_digit() || d == '.' || d == '_' {
                            if d != '_' {
                                num.push(d);
                            }
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    tokens.push(Token::Number(num.parse().unwrap()));
                }

                'a'..='z' | 'A'..='Z' => {
                    let mut ident = String::new();
                    while let Some(&d) = chars.peek() {
                        if d.is_alphanumeric() {
                            ident.push(d);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    tokens.push(Token::Ident(ident));
                }

                '+' => {
                    tokens.push(Token::Plus);
                    chars.next();
                }
                '-' => {
                    tokens.push(Token::Minus);
                    chars.next();
                }
                '*' => {
                    tokens.push(Token::Mul);
                    chars.next();
                }
                '/' => {
                    tokens.push(Token::Div);
                    chars.next();
                }
                '^' => {
                    tokens.push(Token::Pow);
                    chars.next();
                }

                '&' => {
                    tokens.push(Token::And);
                    chars.next();
                }
                '|' => {
                    tokens.push(Token::Or);
                    chars.next();
                }

                '<' => {
                    chars.next();
                    if chars.peek() == Some(&'<') {
                        chars.next();
                        tokens.push(Token::Shl);
                    }
                }

                '>' => {
                    chars.next();
                    if chars.peek() == Some(&'>') {
                        chars.next();
                        tokens.push(Token::Shr);
                    }
                }

                '=' => {
                    tokens.push(Token::Assign);
                    chars.next();
                }
                '(' => {
                    tokens.push(Token::LParen);
                    chars.next();
                }
                ')' => {
                    tokens.push(Token::RParen);
                    chars.next();
                }
                ',' => {
                    tokens.push(Token::Comma);
                    chars.next();
                }

                ' ' => {
                    chars.next();
                }

                _ => return Err(format!("Unexpected char: {}", c)),
            }
        }

        Ok(tokens)
    }
}
