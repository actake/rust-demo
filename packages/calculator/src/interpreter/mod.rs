use std::cell::RefCell;

use crate::{
    error::Error,
    token::{Token, TokenType},
};

pub struct Interpreter {
    text: String,

    pos: RefCell<usize>,

    current_token: RefCell<Option<Token>>,
}

impl Interpreter {
    pub fn new(text: String) -> Self {
        Interpreter {
            text,
            pos: RefCell::new(0),
            current_token: RefCell::new(None),
        }
    }

    pub fn set_current_token(&self, new_token: Token) {
        self.current_token.borrow_mut().replace(new_token);
    }

    pub fn get_next_token(&self) -> Result<Token, Error> {
        let mut pos_value = self.pos.borrow_mut();
        if *pos_value > self.text.len() - 1 {
            return Ok(Token {
                token_type: TokenType::EOF,
                value: None,
            });
        }

        let char: char = self
            .text
            .chars()
            .nth(*pos_value)
            .unwrap_or_else(|| panic!("Get self.text[{}] failed", *pos_value));

        match char {
            '+' => {
                *pos_value += 1;
                Ok(Token {
                    token_type: TokenType::Plus,
                    value: Some(char.to_string()),
                })
            }
            _ => match char.to_digit(10) {
                Some(v) => {
                    *pos_value += 1;
                    Ok(Token {
                        token_type: TokenType::Integer,
                        value: Some(v.to_string()),
                    })
                }
                None => Err(Error::ParseToDigitError),
            },
        }
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

        let _op = self.current_token.borrow().clone().unwrap();
        self.eat(TokenType::Plus)?;

        let Token { value: right, .. } = self.current_token.borrow().clone().unwrap();
        self.eat(TokenType::Integer)?;

        match left {
            Some(left) => match right {
                Some(right) => Ok(left.parse::<u32>().unwrap() + right.parse::<u32>().unwrap()),
                _ => Err(Error::DestructError),
            },
            _ => Err(Error::DestructError),
        }
    }
}
