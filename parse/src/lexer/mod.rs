use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum Token<'a> {
    #[token("(", priority = 4)]
    ListOpen,

    #[token(")", priority = 4)]
    ListClose,

    #[token("'", priority = 4)]
    Quote,

    #[token("false", priority = 4)]
    BooleanFalse,

    #[token("true", priority = 4)]
    BooleanTrue,

    #[regex("-?[0-9]+", priority = 3)]
    Number(&'a str),

    #[regex("\"[^\"]*\"", priority = 3)]
    String(&'a str),

    #[regex("[^ ]+", priority = 2)]
    Identifier(&'a str),
}