use std::fmt::Display;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum TokenType {
    EOF,
    Integer,
    Plus,
    Minus,
    Multiply,
    Division,
}

impl From<TokenType> for String {
    fn from(value: TokenType) -> Self {
        match value {
            TokenType::EOF => String::from("EOF"),
            TokenType::Integer => String::from("Integer"),
            TokenType::Plus => String::from("Plus"),
            TokenType::Minus => String::from("Minus"),
            TokenType::Multiply => String::from("Multiply"),
            TokenType::Division => String::from("Division"),
        }
    }
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::EOF => write!(f, "TokenType::EOF"),
            TokenType::Plus => write!(f, "TokenType::Plus"),
            TokenType::Integer => write!(f, "TokenType::Integer"),
            TokenType::Minus => write!(f, "TokenType::Minus"),
            TokenType::Multiply => write!(f, "TokenType::Multiply"),
            TokenType::Division => write!(f, "TokenType::Division"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub value: Option<String>,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self.value.clone() {
            None => String::from("None"),
            Some(v) => v,
        };

        write!(f, "Token({}, {})", self.token_type, value)
    }
}
