use logos::Logos;

#[derive(Logos, Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum Token {
    #[token("!")]
    NOT = 0,
    #[token("&&")]
    AND = 1,
    #[token("||")]
    OR = 2,
    #[token("^")]
    XOR = 3,
    #[token("(")]
    LPAREN,
    #[token(")")]
    RPAREN,
    #[regex("[a-zA-Z]+")]
    IDENT,

    #[regex(r"[ \t\n\f]+", logos::skip)]
    #[error]
    Error,
}
