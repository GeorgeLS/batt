use crate::bitstring_trait::*;
use crate::token::*;

#[derive(Debug, PartialEq)]
/// This represents a token of a boolean expression.
/// These tokens are emitted by the parser which transforms identifiers into numerical ids.
/// The OPERATOR token contains a Token of the language and that token is always an operator
/// The RESULT boolean token is only used during the evaluation of a boolean expression and contains
/// the result of a boolean operation (i.e AND-ing two identifiers)
pub enum BooleanExpressionToken {
    IDENT(u32),
    OPERATOR(Token),
    RESULT(u8),
}

#[derive(Debug, PartialEq)]
/// This type represents a boolean expression that can be evaluated to a result.
/// This type has always immutable state so it can easily passed to multiple threads
/// for parallel evaluation.
pub struct BooleanExpression<'source> {
    variable_names: Vec<&'source str>,
    exp: Vec<BooleanExpressionToken>,
}

impl<'source> BooleanExpression<'source> {
    /// Creates a new boolean expression by a set of boolean expression tokens and a set of variable names extracted
    /// from the input expression by the parser.
    pub fn new(exp: Vec<BooleanExpressionToken>, variable_names: Vec<&'source str>) -> Self {
        Self {
            variable_names,
            exp,
        }
    }

    #[inline]
    /// Gets the expression's variables
    pub fn variables(&self) -> &Vec<&'source str> {
        &self.variable_names
    }

    /// Evaluates the expression.
    /// In order to evaluate the expression you must pass an object that implements the
    /// BitString trait that comes with this source code.
    /// The BitString trait defines some operations that someone can do to an object as it was a bitstring.
    /// We need that functionality because the parser maps the IDENT tokens (which are our identifiers in the expression)
    /// to numerical ids. For example let's say we have the following expression:
    /// "A && B || C"
    /// The parser will give us back a BooleanExpression object which is essentially a number of BooleanExpressionTokens
    /// in Reverse Polish Notation. See Parser for more details about that.
    /// The BooleanExpression in our case is: [IDENT(0), IDENT(1), OPERATOR(Token::AND), IDENT(2), OPERATOR(Token::OR)]
    /// As you can see variable A has been mapped to number 0, variable B to number 1 and variable C to number 2
    /// This is done so we can easily index a bitstring and get the value that we should assing to that variable.
    /// That was a design decision for the following reason:
    /// When you have a boolean expression of N variables then in order to generate the truth table for that expression you must 
    /// generate 2^N bit strings of length N that each bit can be either 1 or 0.
    /// So essentially you want to generate all the binary strings that represent numbers 0 through 2^(N - 1).
    /// This can be easily done by having a counter starting at 0 and increasing it until it gets to 2^N and each time
    /// use it's binary represenation to extract the values. So we have an implemenation of our BitString trait for the u32 primitive type. 
    pub fn evaluate<T>(&self, input: T) -> u8
    where
        T: BitString,
    {
        let mut stack = Vec::new();
        for token in &self.exp {
            match token {
                BooleanExpressionToken::IDENT(id) => {
                    stack.push(BooleanExpressionToken::RESULT(
                        input.get_bit(self.variable_names.len() - 1 - *id as usize).unwrap(),
                    ));
                }
                BooleanExpressionToken::OPERATOR(op) => match op {
                    Token::AND => {
                        // These are guaranted to match BooleanExpressionToken::Result(_)
                        let lhs = match stack.pop().unwrap() {
                            BooleanExpressionToken::RESULT(value) => value,
                            _ => 0,
                        };
                        let rhs = match stack.pop().unwrap() {
                            BooleanExpressionToken::RESULT(value) => value,
                            _ => 0,
                        };
                        stack.push(BooleanExpressionToken::RESULT(lhs & rhs));
                    }
                    Token::OR => {
                        // These are guaranted to match BooleanExpressionToken::Result(_)
                        let lhs = match stack.pop().unwrap() {
                            BooleanExpressionToken::RESULT(value) => value,
                            _ => 0,
                        };
                        let rhs = match stack.pop().unwrap() {
                            BooleanExpressionToken::RESULT(value) => value,
                            _ => 0,
                        };
                        stack.push(BooleanExpressionToken::RESULT(lhs | rhs));
                    }
                    Token::XOR => {
                        // These are guaranted to match BooleanExpressionToken::Result(_)
                        let lhs = match stack.pop().unwrap() {
                            BooleanExpressionToken::RESULT(value) => value,
                            _ => 0,
                        };
                        let rhs = match stack.pop().unwrap() {
                            BooleanExpressionToken::RESULT(value) => value,
                            _ => 0,
                        };
                        stack.push(BooleanExpressionToken::RESULT(lhs ^ rhs));
                    }
                    Token::NOT => {
                        // These are guaranted to match BooleanExpressionToken::Result(_)
                        let lhs = match stack.pop().unwrap() {
                            BooleanExpressionToken::RESULT(value) => value,
                            _ => 0,
                        };
                        stack.push(BooleanExpressionToken::RESULT(if lhs == 0 { 1 } else { 0 }));
                    }
                    _ => {}
                },
                _ => {}
            }
        }
        // This is guaranteed to be BooleanExpressionToken::RESULT(_)
        match stack.pop().unwrap() {
            BooleanExpressionToken::RESULT(value) => value,
            _ => 0,
        }
    }
}
