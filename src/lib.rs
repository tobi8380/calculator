use std::{
    error::Error,
    f64,
    fmt::{write, Display},
    num::ParseIntError,
};

// May fail if input is not correctly formed
pub fn evaluate_expression(exp: &str) -> Result<Fraction, ParseIntError> {
    todo!();
}

// All fractions can be represented with a positive denominator
// if both numer and denom are negative, then mutliply both with -1
// if only denom is negative then make numer negative instead
pub struct Fraction {
    numer: i64,
    denom: u64,
}

impl Display for Fraction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.denom == 1 {
            write!(f, "{}", self.numer)
        } else {
            write!(f, "{} / {}", self.numer, self.denom)
        }
    }
}

impl Fraction {
    pub fn new(numer: i64, denom: u64) -> Self {
        if denom == 0 {
            panic!("Fraction should have nonzero denominator");
        }
        Self { numer, denom }
    }

    pub fn from_int(int: i64) -> Self {
        Self {
            numer: int,
            denom: 1,
        }
    }

    pub fn to_float(&self) -> f64 {
        self.numer as f64 / self.denom as f64
    }

    pub fn numer(&self) -> i64 {
        self.numer
    }

    pub fn denom(&self) -> u64 {
        self.denom
    }

    pub fn add(&self, other: Self) -> Fraction {
        // wait, this is not how to add fractions lmao
        // probably instead: denom = lcm(self.denom, other.denom)
        // extend fraction by
        todo!();
        // let denom = lcm(self.denom, other.denom);
        // let numer = self.numer * (denom / self.denom) + other.numer * (denom / other.denom);
        // Self::new(numer, denom)
    }

    pub fn subtract(&self, other: Self) -> Fraction {
        // neither is this how to subtract!
        Self::new(self.numer - other.numer, self.denom - other.denom)
    }

    pub fn multiply(&self, other: Self) -> Fraction {
        // but this is how you multiply
        Self::new(self.numer * other.numer, self.denom * other.denom)
    }

    pub fn divide(&self, other: Self) -> Fraction {
        // i think this is right?
        let mut numer = self.numer * other.denom as i64;
        let denom;
        if other.numer < 0 {
            // case: other.numer < 0 or (self.numer < 0 and other.numer < 0)
            denom = other.numer.unsigned_abs() * self.denom;
            numer = -numer;
        } else {
            // case: all positive or self.numer < 0
            denom = other.numer as u64 * self.denom;
        }
        assert!(denom > 0);
        Self::new(numer, denom)
    }
}

// Return the least common multiple of a and b
fn lcm(a: u64, b: u64) -> u64 {
    _lcm(a, b, a, b)
}

fn _lcm(a_sum: u64, b_sum: u64, a: u64, b: u64) -> u64 {
    println!("{a_sum}, {a}, {b_sum}, {b}");
    if a_sum == b_sum {
        a_sum
    } else if a_sum < b_sum {
        _lcm(a_sum + a, b_sum, a, b)
    } else {
        _lcm(a_sum, b_sum + b, a, b)
    }
}

struct Calculator {
    tokens: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lcm_test() {
        assert_eq!(lcm(3, 6), 6);
        assert_eq!(lcm(6, 3), 6);
        assert_eq!(lcm(6, 5), 30);
        assert_eq!(lcm(5, 6), 30);
    }
}
