use crate::boolean_expression::*;
use crate::token::*;
use colored::*;
use logos;

lazy_static! {
    static ref ERROR_TAG: colored::ColoredString = "[ERROR]: ".red();
}

pub struct Parser<'source> {
    lex: logos::Lexer<'source, Token>,
}

impl<'source> Parser<'source> {
    pub fn new(lex: logos::Lexer<'source, Token>) -> Self {
        Self { lex }
    }

    pub fn parse(&mut self) -> Option<BooleanExpression> {
        let mut stack = Vec::new();
        let mut res = Vec::new();
        let mut prev_token: Option<(Token, logos::Span)> = None;
        while let Some(t) = self.lex.next() {
            let t_span = self.lex.span();

            if t == Token::Error {
                self.report_token_error(t_span, "Unknown token");
                return None;
            }

            match t {
                Token::IDENT => {
                    prev_token = Some((t, t_span));
                    res.push(t)
                }
                Token::LPAREN => {
                    prev_token = Some((t, t_span.clone()));
                    stack.push((t, t_span));
                }
                Token::RPAREN => {
                    prev_token = Some((t, t_span.clone()));
                    let mut seen_lparen = false;
                    while let Some((st, _)) = stack.pop() {
                        if st == Token::LPAREN {
                            seen_lparen = true;
                            break;
                        }
                        res.push(st);
                    }
                    if !seen_lparen {
                        self.report_token_error(t_span, "Unmatched opening parenthesis");
                        return None;
                    }
                }
                _ => {
                    // These are all the operators
                    while let Some((top, _)) = stack.last() {
                        if t == Token::NOT {
                            break;
                        }
                        if *top <= t {
                            res.push(*top);
                            stack.pop();
                        } else {
                            break;
                        }
                    }
                    stack.push((t, t_span.clone()));
                    if !self.check_operator_context(prev_token, t, t_span.clone()) {
                        return None;
                    }
                    prev_token = Some((t, t_span));
                }
            }
        }

        while let Some((t, t_span)) = stack.pop() {
            if t == Token::LPAREN {
                self.report_token_error(t_span, "Unclosed right parenthesis");
                return None;
            }
            res.push(t);
        }

        Some(BooleanExpression::new(res))
    }

    fn check_operator_context(
        &self,
        prev_token: Option<(Token, logos::Span)>,
        op: Token,
        op_span: logos::Span,
    ) -> bool {
        let mut cloned_lex = self.lex.clone();
        let next_token = cloned_lex.next();
        let next_token_span = cloned_lex.span();

        match next_token {
            Some(next_token) => {
                if next_token == Token::Error {
                    self.report_token_error(next_token_span, "Unknown token");
                    return false;
                }

                if next_token == Token::RPAREN {
                    self.report_token_error(next_token_span, "Invalid operand");
                    return false;
                }

                if next_token != Token::NOT
                    && next_token != Token::IDENT
                    && next_token != Token::LPAREN
                {
                    self.report_token_error(next_token_span, "Invalid operator");
                    return false;
                }

                if op != Token::NOT {
                    if prev_token.is_none() {
                        self.report_token_error(op_span, "Missing left hand side operand");
                        return false;
                    }
                    let prev_token = prev_token.unwrap();

                    if prev_token.0 != Token::IDENT && prev_token.0 != Token::RPAREN {
                        self.report_token_error(prev_token.1, "Invalid operand");
                        return false;
                    }
                }

                return true;
            }
            None => {
                self.report_token_error(
                    op_span,
                    "Right hand side operand of expression is missing",
                );
                return false;
            }
        }
    }

    fn report_token_error(&self, token_span: logos::Span, msg: &str) {
        let source = self.lex.source();
        eprintln!(
            "{}{}{}{}",
            *ERROR_TAG,
            &source[..token_span.start],
            &source[token_span.start..token_span.end].red(),
            &source[token_span.end..]
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
        assert_eq!(exp.unwrap().exp, vec![Token::IDENT, Token::NOT]);
    }

    #[test]
    fn test_binary_operator_and() {
        let exp = Parser::new(Token::lexer("A && B")).parse();
        assert!(exp.is_some());
        assert_eq!(
            exp.unwrap().exp,
            vec![Token::IDENT, Token::IDENT, Token::AND]
        );
    }

    #[test]
    fn test_binary_operator_or() {
        let exp = Parser::new(Token::lexer("A || B")).parse();
        assert!(exp.is_some());
        assert_eq!(
            exp.unwrap().exp,
            vec![Token::IDENT, Token::IDENT, Token::OR]
        );
    }

    #[test]
    fn test_binary_operator_xor() {
        let exp = Parser::new(Token::lexer("A ^ B")).parse();
        assert!(exp.is_some());
        assert_eq!(
            exp.unwrap().exp,
            vec![Token::IDENT, Token::IDENT, Token::XOR]
        );
    }

    #[test]
    fn test_parenthesis_erasure() {
        let exp = Parser::new(Token::lexer("(A) && (B)")).parse();
        assert!(exp.is_some());
        assert_eq!(
            exp.unwrap().exp,
            vec![Token::IDENT, Token::IDENT, Token::AND]
        );
    }
}
