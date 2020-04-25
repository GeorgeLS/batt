use logos::Logos;

#[derive(Logos, Debug, PartialEq, PartialOrd, Clone, Copy)]
/// The token of our minimal boolean algrebra expression language
/// IDENT token is an identifier (a boolean variable) and can be anything
/// group of alphabetical characters. The variables are case sensitive
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

impl Token {
    #[inline]
    /// Checks whether the token is a binary operator.
    /// The binary operators are:
    /// AND, OR and XOR
    pub fn is_binary_operator(self) -> bool {
        match self {
            Token::AND | Token::OR | Token::XOR => true,
            _ => false,
        }
    }
}
