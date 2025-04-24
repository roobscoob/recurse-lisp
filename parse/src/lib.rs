use std::num::ParseIntError;

use lexer::Token;
use list::List;
use value::Value;

pub mod value;
pub mod list;
pub mod lexer;

pub enum ParseError {
    ClosedUnopenedList,
    UnterminatedList,
    UnvaluedQuote,
    InvalidInteger(ParseIntError),
}

pub fn parse<'a, I: Iterator<Item = Token<'a>>>(input: &mut I) -> Option<Result<Value, ParseError>> {
    match input.next()? {
        Token::BooleanFalse => Some(Ok(Value::Boolean(false))),
        Token::BooleanTrue => Some(Ok(Value::Boolean(true))),
        Token::String(str) => Some(Ok(Value::String(str.to_owned()))),
        Token::ListClose => Some(Err(ParseError::ClosedUnopenedList)),
        Token::Identifier(str) => Some(Ok(Value::Symbol(Box::new(Value::String(str.to_owned()))))),
        Token::Number(str) => match str.parse() {
            Ok(int) => Some(Ok(Value::Integer(int))),
            Err(e) => Some(Err(ParseError::InvalidInteger(e))),
        }
        Token::Quote => {
            match parse(input).ok_or(ParseError::UnvaluedQuote) {
                Err(e) => return Some(Err(e)),
                Ok(Err(e)) => return Some(Err(e)),
                Ok(Ok(v)) => Some(Ok(Value::Symbol(Box::new(v)))),
            }
        }
        Token::ListOpen => {
            let mut list = List::build();

            loop {
                match parse(input).ok_or(ParseError::UnterminatedList) {
                    Err(e) => return Some(Err(e)),
                    Ok(Err(ParseError::ClosedUnopenedList)) => break Some(Ok(Value::List(list.build()))),
                    Ok(Err(e)) => return Some(Err(e)),
                    Ok(Ok(v)) => list.with(v),
                }
            }
        }
    }
}