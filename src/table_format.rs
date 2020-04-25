use crate::bitstring_trait::*;
use crate::boolean_expression::BooleanExpression;

/// A helper struct that prints the truth table for a given boolean expression
pub struct TableFormat {
    header: String,
    row_separator: String,
    expression_length: usize,
}

impl TableFormat {
    pub fn new(exp: &str, bexp: &BooleanExpression) -> TableFormat {
        let variables = bexp.variables();
        let header = variables.join("|");
        let header = format!("|{}|{}|", header, exp);
        let row_separator = format!("{:-<1$}", "", header.len());
        TableFormat {
            header,
            row_separator,
            expression_length: exp.len(),
        }
    }

    #[inline]
    pub fn print_header(&self) {
        println!();
        println!("{}", self.row_separator);
        println!("{}", self.header);
        println!("{}", self.row_separator);
    }

    #[inline]
    pub fn print_row_separator(&self) {
        println!("{}", self.row_separator);
    }

    #[inline]
    pub fn print_evaluation<T>(&self, bexp: &BooleanExpression, input: T, eval_result: u8)
    where
        T: BitString,
    {
        let variables = bexp.variables();
        let number_of_vars = variables.len();
        for (i, var) in variables.iter().enumerate() {
            print!(
                "|{: >1$}",
                input.get_bit(number_of_vars - 1 - i as usize).unwrap(),
                var.len()
            );
        }
        println!("|{: >1$}|", eval_result, self.expression_length);
    }
}
