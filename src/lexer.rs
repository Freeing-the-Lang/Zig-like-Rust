#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Ident(String),
    Number(i64),
    Equal,
    LParen,
    RParen,
    Comma,
    Semicolon,
    Const,
    EOF,
}

pub fn tokenize(src: &str) -> Vec<Token> {
    let mut tokens = vec![];
    let chars: Vec<char> = src.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];

        if c.is_whitespace() {
            i += 1;
            continue;
        }

        if c.is_ascii_digit() {
            let mut n = String::new();
            while i < chars.len() && chars[i].is_ascii_digit() {
                n.push(chars[i]);
                i += 1;
            }
            tokens.push(Token::Number(n.parse().unwrap()));
            continue;
        }

        if c.is_ascii_alphabetic() {
            let mut id = String::new();
            while i < chars.len() && (chars[i].is_ascii_alphanumeric() || chars[i]=='_') {
                id.push(chars[i]);
                i += 1;
            }

            if id == "const" {
                tokens.push(Token::Const);
            } else {
                tokens.push(Token::Ident(id));
            }
            continue;
        }

        match c {
            '=' => tokens.push(Token::Equal),
            '(' => tokens.push(Token::LParen),
            ')' => tokens.push(Token::RParen),
            ',' => tokens.push(Token::Comma),
            ';' => tokens.push(Token::Semicolon),
            _   => {}
        }

        i += 1;
    }

    tokens.push(Token::EOF);
    tokens
}
