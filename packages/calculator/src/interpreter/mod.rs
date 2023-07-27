use std::cell::RefCell;

use crate::{
    error::Error,
    token::{Token, TokenType},
};

pub struct Interpreter {
    text: String,

    pos: RefCell<usize>,

    current_token: RefCell<Option<Token>>,

    current_char: RefCell<Option<char>>,
}

impl Interpreter {
    pub fn new(text: String) -> Self {
        Interpreter {
            text: text.clone(),
            pos: RefCell::new(0),
            current_token: RefCell::new(None),
            current_char: RefCell::new(text.chars().next()),
        }
    }

    pub fn set_current_token(&self, new_token: Token) {
        self.current_token.borrow_mut().replace(new_token);
    }

    pub fn advance(&self) {
        let mut pos_value = self.pos.borrow_mut();

        *pos_value += 1;

        if *pos_value > self.text.len() - 1 {
            *self.current_char.borrow_mut() = None;
        } else {
            self.current_char
                .borrow_mut()
                .replace(self.text.chars().nth(*pos_value).unwrap());
        }
    }

    pub fn skip_whitespace(&self) {
        while let Some(char) = *self.current_char.clone().borrow() {
            if !char.is_whitespace() {
                break;
            }
            self.advance()
        }
    }

    pub fn integer(&self) -> Option<String> {
        let mut result = String::new();
        while let Some(char) = *self.current_char.clone().borrow() {
            if !char.is_ascii_digit() {
                break;
            }

            result += char.to_string().as_str();
            self.advance();
        }

        Some(result)
    }

    pub fn get_next_token(&self) -> Result<Token, Error> {
        while let Some(char) = *self.current_char.clone().borrow() {
            if char.is_whitespace() {
                self.skip_whitespace();
                continue;
            }

            if char.is_ascii_digit() {
                return Ok(Token {
                    token_type: TokenType::Integer,
                    value: self.integer(),
                });
            }

            if char == '+' {
                self.advance();

                return Ok(Token {
                    token_type: TokenType::Plus,
                    value: Some('+'.to_string()),
                });
            }

            if char == '-' {
                self.advance();

                return Ok(Token {
                    token_type: TokenType::Minus,
                    value: Some('-'.to_string()),
                });
            }
        }

        Ok(Token {
            token_type: TokenType::EOF,
            value: None,
        })
    }

    fn eat(&self, token_type: TokenType) -> Result<(), Error> {
        match self.current_token.take() {
            None => Err(Error::NullTokenError)?,
            Some(Token {
                token_type: destruct_token_type,
                ..
            }) => {
                if destruct_token_type == token_type {
                    let new_token = self.get_next_token()?;
                    self.set_current_token(new_token);
                } else {
                    Err(Error::TokenTypeMatchError)?;
                }
            }
        };

        Ok(())
    }

    pub fn expr(&self) -> Result<u32, Error> {
        let next_token = self.get_next_token()?;
        self.set_current_token(next_token);

        let Token { value: left, .. } = self.current_token.borrow().clone().unwrap();
        self.eat(TokenType::Integer)?;

        let op = self.current_token.borrow().clone().unwrap();
        self.eat(op.token_type)?;

        let Token { value: right, .. } = self.current_token.borrow().clone().unwrap();
        self.eat(TokenType::Integer)?;

        left.and_then(|left| {
            right.map(|right| {
                let left_num = left.parse::<u32>().unwrap();
                let right_num = right.parse::<u32>().unwrap();

                (left_num, right_num)
            })
        })
        .map(|(left, right)| match op.token_type {
            TokenType::Plus => Ok(left + right),
            TokenType::Minus => Ok(left - right),
            _ => Err(Error::TokenTypeMatchError)?,
        })
        .ok_or(Error::DestructError)?
    }
}
