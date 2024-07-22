use std::{iter::Peekable, result, str::Split};

pub mod fraction;
use fraction::Fraction;

// May fail if input is not correctly formed
// pub fn evaluate_expression(exp: &str) -> Result<Fraction, ParseIntError> {
pub fn eval_expression(exp: &str) -> Fraction {
    let mut tokens = Tokenizer::new(exp);
    parse_exp(&mut tokens)
}

fn parse_exp(tokens: &mut Tokenizer) -> Fraction {
    let mut result;
    // Handle potential unary minus sign
    if let Some("-") = tokens.head() {
        tokens.next();
        result = -parse_mult(tokens);
    } else {
        result = parse_mult(tokens);
    };
    loop {
        match tokens.head() {
            Some("+") => {
                tokens.next();
                result = result + parse_mult(tokens)
            }
            Some("-") => {
                tokens.next();
                result = result - parse_mult(tokens)
            }
            _ => break,
            // Some(")") => {
            //     tokens.next();
            //     break;
            // }
            // Some(invalid) => {
            //     panic!("Malformed expression contained unexpected token: {invalid}");
            // }
            // None => {
            //     break;
            // }
        };
    }
    result
}

fn parse_mult(tokens: &mut Tokenizer) -> Fraction {
    let mut result = parse_factor(tokens);
    loop {
        match tokens.head() {
            Some("*") => {
                tokens.next();
                result = result * parse_factor(tokens)
            }
            Some("/") => {
                tokens.next();
                result = result / parse_factor(tokens)
            }
            _ => break,
        };
    }
    result
}

// Power is not easy/possible if we want to keep fractions,
// because raising to fractional powers leads to both numerator and
// denominator potentially becoming fractional, which im not sure how to handle
//
// fn parse_pow(tokens: &mut Tokenizer) -> Fraction {
//     let t = parse_factor(tokens);
//     match tokens.head() {
//         Some("^") => {
//             tokens.next();
//             t.pow(parse_pow(tokens))
//         }
//         _ => t,
//     }
// }

fn parse_factor(tokens: &mut Tokenizer) -> Fraction {
    match tokens.next() {
        Some("(") => {
            let res = parse_exp(tokens);
            match tokens.head() {
                Some(")") => {
                    tokens.next();
                    res
                }
                _ => panic!("Malformed expression"),
            }
        }
        Some(int) => match int.parse::<u64>() {
            Ok(value) => Fraction::from_int(value as i64),
            // TODO: Bubble error instead?
            Err(_) => panic!("Factor should be an integer"),
        },
        None => panic!("Ill formed expression"),
    }
}

struct Tokenizer<'a> {
    tokens: Peekable<Split<'a, char>>,
}

// TODO: Make this able to work without spaces separating
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
        assert_eq!(t.next(), Some("hello"));
        assert_eq!(t.head(), Some("how"));
        assert_eq!(t.next(), Some("how"));
        assert_eq!(t.head(), Some("are"));
        assert_eq!(t.next(), Some("are"));
        assert_eq!(t.head(), Some("you"));
        assert_eq!(t.next(), Some("you"));
        assert!(!t.has_next());
        assert_eq!(t.head(), None);
        assert_eq!(t.next(), None);
    }

    #[test]
    fn simple_expressions() {
        assert_eq!(eval_expression("2 + 2"), Fraction::from_int(4));
        assert_eq!(eval_expression("2 + 6 + 3 + 9"), Fraction::from_int(20));
        assert_eq!(eval_expression("2 * 3 + 5"), Fraction::from_int(11));
        assert_eq!(eval_expression("5 - 8"), Fraction::from_int(-3));
        assert_eq!(
            eval_expression("3 * 7 + 4 * ( 4 - 2 )"),
            Fraction::from_int(29)
        );
        assert_eq!(
            eval_expression("9 / 3 * 9 - 8 * 2 / ( 15 / 3 * 8 )"),
            Fraction::new(266, 10)
        );
    }
}
