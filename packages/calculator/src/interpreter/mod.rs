use std::rc::Rc;

use crate::{
    error::Error,
    token::{Token, TokenType},
};

pub struct Interpreter {
    text: String,

    pos: usize,

    current_token: Rc<Option<Token>>,
}

impl Interpreter {
    pub fn new(text: String) -> Self {
        Interpreter {
            text,
            pos: 0,
            current_token: Rc::new(None),
        }
    }

    pub fn set_current_token(&mut self, new_token: Token) {
        self.current_token = Rc::new(Some(new_token));
    }

    pub fn get_next_token(&mut self) -> Result<Token, Error> {
        if self.pos > self.text.len() - 1 {
            return Ok(Token {
                token_type: TokenType::EOF,
                value: None,
            });
        }

        let char: char = self
            .text
            .chars()
            .nth(self.pos)
            .unwrap_or_else(|| panic!("Get self.text[{}] failed", self.pos));

        match char {
            '+' => {
                self.pos += 1;
                Ok(Token {
                    token_type: TokenType::Plus,
                    value: Some(char.to_string()),
                })
            }
            _ => match char.to_digit(10) {
                Some(v) => {
                    self.pos += 1;
                    Ok(Token {
                        token_type: TokenType::Integer,
                        value: Some(v.to_string()),
                    })
                }
                None => Err(Error::ParseError),
            },
        }
    }

    fn eat(&mut self, token_type: TokenType) -> Result<(), Error> {
        match *self.current_token.to_owned() {
            None => Err(Error::NullError)?,
            Some(Token {
                token_type: destruct_token_type,
                ..
            }) => {
                if destruct_token_type == token_type {
                    let new_token = self.get_next_token()?;
                    self.set_current_token(new_token);
                } else {
                    // Err(Error::ParseError)?;
                }
            }
        };

        Ok(())
    }

    pub fn expr(&mut self) -> Result<u32, Error> {
        let next_token = self.get_next_token()?;
        self.set_current_token(next_token);

        let Token { value: left, .. } = self.current_token.as_ref().clone().unwrap();
        self.eat(TokenType::Integer)?;

        let _op = self.current_token.as_ref().clone().unwrap();
        self.eat(TokenType::Plus)?;

        let Token { value: right, .. } = self.current_token.as_ref().clone().unwrap();
        self.eat(TokenType::Integer)?;

        match left {
            Some(left) => match right {
                Some(right) => Ok(left.parse::<u32>().unwrap() + right.parse::<u32>().unwrap()),
                _ => Err(Error::ParseError),
            },
            _ => Err(Error::ParseError),
        }
    }
}
