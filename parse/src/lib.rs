use std::num::ParseIntError;

use lexer::Token;
use list::List;
use value::Value;

pub mod lexer;
pub mod list;
pub mod value;

#[derive(Debug)]
pub enum ParseError {
    ClosedUnopenedList,
    UnterminatedList,
    UnvaluedQuote,
    LexerError,
    InvalidInteger(ParseIntError),
}

pub fn parse<'a, I: Iterator<Item = Result<Token<'a>, ()>>>(
    input: &mut I,
) -> Option<Result<Value, ParseError>> {
    match input.next()? {
        Err(_) => Some(Err(ParseError::LexerError)),
        Ok(Token::String(str)) => Some(Ok(Value::String(str.to_owned()))),
        Ok(Token::ListClose) => Some(Err(ParseError::ClosedUnopenedList)),
        Ok(Token::Identifier(str)) => {
            Some(Ok(Value::Symbol(Box::new(Value::String(str.to_owned())))))
        }
        Ok(Token::Number(str)) => match str.parse() {
            Ok(int) => Some(Ok(Value::Integer(int))),
            Err(e) => Some(Err(ParseError::InvalidInteger(e))),
        },
        Ok(Token::Quote) => match parse(input).ok_or(ParseError::UnvaluedQuote) {
            Err(e) => return Some(Err(e)),
            Ok(Err(e)) => return Some(Err(e)),
            Ok(Ok(v)) => Some(Ok(Value::Symbol(Box::new(v)))),
        },
        Ok(Token::ListOpen) => {
            let mut list = List::build();

            loop {
                match parse(input).ok_or(ParseError::UnterminatedList) {
                    Err(e) => return Some(Err(e)),
                    Ok(Err(ParseError::ClosedUnopenedList)) => {
                        break Some(Ok(Value::List(list.build())));
                    }
                    Ok(Err(e)) => return Some(Err(e)),
                    Ok(Ok(v)) => list.with(v),
                }
            }
        }
    }
}
