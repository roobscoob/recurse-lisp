use crate::list::List;

#[derive(Debug)]
pub enum Value {
    Boolean(bool),
    Integer(i64),
    String(String),
    List(List),

    // i don't super understand symbols so this is probably wrong :(
    Symbol(Box<Value>),
}