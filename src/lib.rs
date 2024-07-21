use std::{iter::Peekable, str::Split};

pub mod fraction;
use fraction::Fraction;

// May fail if input is not correctly formed
// pub fn evaluate_expression(exp: &str) -> Result<Fraction, ParseIntError> {
pub fn evaluate_expression(exp: &str) -> Fraction {
    let tokens = Tokenizer::new(exp);
    parse_exp(tokens)
}

fn parse_exp(tokens: Tokenizer) -> Fraction {
    todo!()
}

fn parse_mult(tokens: Tokenizer) -> Fraction {
    todo!()
}

fn parse_factor(tokens: Tokenizer) -> Fraction {
    todo!()
}

struct Tokenizer<'a> {
    tokens: Peekable<Split<'a, char>>,
}

impl<'a> Tokenizer<'a> {
    fn new(exp: &'a str) -> Self {
        let tokens = exp.split(' ').peekable();
        Self { tokens }
    }

    fn has_next(&mut self) -> bool {
        self.tokens.peek().is_some()
    }

    fn next(&mut self) -> Option<&str> {
        // self.tokens.next().map(str::to_string)
        self.tokens.next()
    }

    fn head(&mut self) -> Option<&str> {
        // self.tokens.peek().map(|s| s.to_string())
        self.tokens.peek().cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenizer_test() {
        let mut t = Tokenizer::new("hello how are you");
        assert!(t.has_next());
        assert_eq!(t.head(), Some("hello"));
    }
}
