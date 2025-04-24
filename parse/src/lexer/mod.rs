use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token<'a> {
    #[token("(", priority = 5)]
    ListOpen,

    #[token(")", priority = 5)]
    ListClose,

    #[token("'", priority = 5)]
    Quote,

    #[regex(r"-?[0-9]+", priority = 4)]
    Number(&'a str),

    #[regex("\"[^\"]*\"", priority = 4)]
    String(&'a str),

    #[regex(r"[^ \(\)]+", priority = 3)]
    Identifier(&'a str),
}
