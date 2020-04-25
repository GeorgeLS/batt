use crate::boolean_expression::*;
use crate::token::*;
use colored::*;
use std::{collections::HashMap, iter::Peekable};

lazy_static! {
    static ref ERROR_TAG: colored::ColoredString = "[ERROR]: ".red();
}

pub struct Parser<'source> {
    source: &'source str,
    lex: Peekable<logos::SpannedIter<'source, Token>>,
    ident_map: HashMap<&'source str, u32>,
    next_ident_id: u32,
}

impl<'source> Parser<'source> {
    pub fn new(lex: logos::Lexer<'source, Token>) -> Self {
        Self {
            source: lex.source(),
            lex: lex.spanned().peekable(),
            ident_map: HashMap::new(),
            next_ident_id: 0,
        }
    }

    pub fn parse(&mut self) -> Option<BooleanExpression<'source>> {
        // This function checks if the expression is a valid boolean expression
        // and converts it into reversed polish notation.
        let mut stack = Vec::new();
        let mut res = Vec::new();
        let mut variables = Vec::new();
        let mut prev_token: Option<Token> = None;

        while let Some((token, span)) = self.lex.next() {
            if token == Token::Error {
                self.report_token_error(span, "Unknown token");
                return None;
            }

            match token {
                Token::IDENT => {
                    if let Some(next_token_span) =
                        self.any_of_matches_next(&[Token::IDENT, Token::LPAREN, Token::NOT])
                    {
                        self.report_token_error(
                            next_token_span,
                            "Expected binary operator or right parenthesis.",
                        );
                        return None;
                    }
                    prev_token = Some(token);
                    let ident_str = &self.source[span.start..span.end];
                    if !self.ident_map.contains_key(ident_str) {
                        self.ident_map.insert(ident_str, self.next_ident_id);
                        variables.push(ident_str);
                        self.next_ident_id += 1;
                    }
                    res.push(BooleanExpressionToken::IDENT(
                        *self.ident_map.get(ident_str).unwrap(),
                    ));
                }
                Token::LPAREN => {
                    if let Some(next_token_span) =
                        self.any_of_matches_next(&[Token::AND, Token::OR, Token::XOR])
                    {
                        self.report_token_error(
                            next_token_span,
                            "Expected parenthesis, variable or unary operator",
                        );
                        return None;
                    }
                    prev_token = Some(token);

                    stack.push((token, span));
                }
                Token::RPAREN => {
                    let mut seen_lparen = false;
                    while let Some((top, _)) = stack.pop() {
                        if top == Token::LPAREN {
                            seen_lparen = true;
                            break;
                        }
                        res.push(BooleanExpressionToken::OPERATOR(top));
                    }
                    if !seen_lparen {
                        self.report_token_error(span, "Unmatched left parenthesis");
                        return None;
                    }
                    if let Some(next_token_span) =
                        self.any_of_matches_next(&[Token::IDENT, Token::NOT, Token::LPAREN])
                    {
                        self.report_token_error(
                            next_token_span,
                            "Expected binary operator or right parenthesis",
                        );
                        return None;
                    }
                    prev_token = Some(token);
                }
                _ => {
                    // These are all the operators
                    while let Some((top, _)) = stack.last() {
                        if token == Token::NOT {
                            // Special case for unary operators such as NOT.
                            // We want to keep them in the stack
                            break;
                        }
                        if *top <= token {
                            res.push(BooleanExpressionToken::OPERATOR(*top));
                            stack.pop();
                        } else {
                            break;
                        }
                    }
                    stack.push((token, span.clone()));

                    if prev_token.is_none() && token.is_binary_operator() {
                        self.report_token_error(
                            span,
                            "Missing left hand side of binary expression",
                        );
                        return None;
                    }

                    if let Some(next_token_span) = self.any_of_matches_next(&[
                        Token::AND,
                        Token::OR,
                        Token::XOR,
                        Token::RPAREN,
                    ]) {
                        self.report_token_error(
                            next_token_span,
                            "Expected variable, left parenthesis or unary operator",
                        );
                        return None;
                    } else if self.lex.peek().is_none() {
                        self.report_token_error(
                            span,
                            "Missing right hand side of binary expression",
                        );
                        return None;
                    }

                    prev_token = Some(token);
                }
            }
        }

        while let Some((token, span)) = stack.pop() {
            if token == Token::LPAREN {
                self.report_token_error(span, "Unmatched right parenthesis");
                return None;
            }
            res.push(BooleanExpressionToken::OPERATOR(token));
        }

        Some(BooleanExpression::new(res, variables))
    }

    fn any_of_matches_next(&mut self, tokens: &[Token]) -> Option<logos::Span> {
        if let Some((next, span)) = self.lex.peek().map(|(t, s)| (*t, s.clone())) {
            for token in tokens {
                if *token == next {
                    return Some(span);
                }
            }
            None
        } else {
            None
        }
    }

    fn report_token_error(&self, token_span: logos::Span, msg: &str) {
        eprintln!(
            "{}{}{}{}",
            *ERROR_TAG,
            &self.source[..token_span.start],
            &self.source[token_span.start..token_span.end].red(),
            &self.source[token_span.end..]
        );
        eprintln!(
            "{}{}",
            format!("{: <1$}", "", token_span.start + ERROR_TAG.len()),
            "^".yellow()
        );
        eprintln!(
            "{} {}",
            format!("{:-<1$}┆", "", token_span.start + ERROR_TAG.len()).yellow(),
            msg.red()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use logos::Logos;

    #[test]
    fn test_unary_operator_not() {
        let exp = Parser::new(Token::lexer("!A")).parse();
        assert!(exp.is_some());
        assert_eq!(
            exp.unwrap(),
            BooleanExpression::new(
                vec![
                    BooleanExpressionToken::IDENT(0),
                    BooleanExpressionToken::OPERATOR(Token::NOT)
                ],
                vec!["A"]
            )
        );
    }

    #[test]
    fn test_binary_operator_and() {
        let exp = Parser::new(Token::lexer("A && B")).parse();
        assert!(exp.is_some());
        assert_eq!(
            exp.unwrap(),
            BooleanExpression::new(
                vec![
                    BooleanExpressionToken::IDENT(0),
                    BooleanExpressionToken::IDENT(1),
                    BooleanExpressionToken::OPERATOR(Token::AND)
                ],
                vec!["A", "B"]
            )
        );
    }

    #[test]
    fn test_binary_operator_or() {
        let exp = Parser::new(Token::lexer("A || B")).parse();
        assert!(exp.is_some());
        assert_eq!(
            exp.unwrap(),
            BooleanExpression::new(
                vec![
                    BooleanExpressionToken::IDENT(0),
                    BooleanExpressionToken::IDENT(1),
                    BooleanExpressionToken::OPERATOR(Token::OR)
                ],
                vec!["A", "B"]
            )
        );
    }

    #[test]
    fn test_binary_operator_xor() {
        let exp = Parser::new(Token::lexer("A ^ B")).parse();
        assert!(exp.is_some());
        assert_eq!(
            exp.unwrap(),
            BooleanExpression::new(
                vec![
                    BooleanExpressionToken::IDENT(0),
                    BooleanExpressionToken::IDENT(1),
                    BooleanExpressionToken::OPERATOR(Token::XOR)
                ],
                vec!["A", "B"]
            )
        );
    }

    #[test]
    fn test_parenthesis_erasure() {
        let exp = Parser::new(Token::lexer("((A) && (B))")).parse();
        assert!(exp.is_some());
        assert_eq!(
            exp.unwrap(),
            BooleanExpression::new(
                vec![
                    BooleanExpressionToken::IDENT(0),
                    BooleanExpressionToken::IDENT(1),
                    BooleanExpressionToken::OPERATOR(Token::AND)
                ],
                vec!["A", "B"]
            )
        );
    }

    #[test]
    fn test_operator_precedence() {
        let exp = Parser::new(Token::lexer("A && !B || C")).parse();
        assert!(exp.is_some());
        assert_eq!(
            exp.unwrap(),
            BooleanExpression::new(
                vec![
                    BooleanExpressionToken::IDENT(0),
                    BooleanExpressionToken::IDENT(1),
                    BooleanExpressionToken::OPERATOR(Token::NOT),
                    BooleanExpressionToken::OPERATOR(Token::AND),
                    BooleanExpressionToken::IDENT(2),
                    BooleanExpressionToken::OPERATOR(Token::OR)
                ],
                vec!["A", "B", "C"]
            )
        );
    }

    #[test]
    fn test_same_identifier() {
        let exp = Parser::new(Token::lexer("A && B || !A")).parse();
        assert!(exp.is_some());
        assert_eq!(
            exp.unwrap(),
            BooleanExpression::new(
                vec![
                    BooleanExpressionToken::IDENT(0),
                    BooleanExpressionToken::IDENT(1),
                    BooleanExpressionToken::OPERATOR(Token::AND),
                    BooleanExpressionToken::IDENT(0),
                    BooleanExpressionToken::OPERATOR(Token::NOT),
                    BooleanExpressionToken::OPERATOR(Token::OR),
                ],
                vec!["A", "B"]
            )
        );
    }
}
