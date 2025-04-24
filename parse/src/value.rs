use crate::list::List;

#[derive(Debug)]
pub enum Value {
    Boolean(bool),
    Integer(i64),
    String(String),
    List(List),
    Symbol(Box<Value>),
}
