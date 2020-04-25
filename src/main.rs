#[macro_use]
extern crate lazy_static;

mod bitstring_trait;
mod boolean_expression;
mod parser;
mod table_format;
mod token;

use logos::Logos;
use parser::Parser;
use std::io;
use table_format::TableFormat;
use token::*;

fn main() {
    let mut exp = String::new();
    io::stdin()
        .read_line(&mut exp)
        .expect("Something went wrong when reading input from stdin");

    let exp = exp.trim();
    if let Some(bexp) = Parser::new(Token::lexer(&exp)).parse() {
        let variables = bexp.variables();
        let number_of_vars = variables.len();

        let table_format = TableFormat::new(&exp, &bexp);
        table_format.print_header();
        for i in 0..(2 << (number_of_vars - 1)) as u128 {
            let res = bexp.evaluate(i);
            table_format.print_evaluation(&bexp, i, res);
            table_format.print_row_separator();
        }
    }
}
