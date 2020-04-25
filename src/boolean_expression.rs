use crate::token::*;

pub struct BooleanExpression {
    pub exp: Vec<Token>,
}

impl BooleanExpression {
    pub fn new(exp: Vec<Token>) -> Self {
        Self { exp }
    }
}
