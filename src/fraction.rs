use std::{
    fmt::Display,
    i64,
    ops::{Add, Div, Mul, Sub},
};

// NOTE:
// By the fundamental theorem of arithmetic, rational numbers in lowest
// terms are unique. So, by keeping `Rational`s in reduced form, we can
// derive `Eq` and `PartialEq`.
// #[derive(Debug, Eq, PartialEq)]

// All fractions can be represented with a positive denominator
// if both numer and denom are negative, then mutliply both with -1
// if only denom is negative then make numer negative instead
// FIXME:
// Perhaps it is smarter to use a sign bool/enum instead
// this probably makes the math simpler to think about at least
// pub enum Sign {
//     Plus,
//     Minus,
// }
#[derive(Debug, Clone)]
pub struct Fraction {
    numer: i64,
    denom: u64,
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

    pub fn simplify(&self) -> Self {
        let common_factor = gcd(self.numer.unsigned_abs(), self.denom);
        let numer = self.numer / common_factor as i64;
        let denom = self.denom / common_factor;
        Self { numer, denom }
    }
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

impl PartialEq for Fraction {
    // a/b == c/d <=> a*d == b*c
    fn eq(&self, other: &Self) -> bool {
        self.numer * other.denom as i64 == other.numer * self.denom as i64
    }
}

impl Add for Fraction {
    type Output = Fraction;

    fn add(self, rhs: Self) -> Fraction {
        let denom = lcm(self.denom, rhs.denom);
        let numer =
            self.numer * (denom / self.denom) as i64 + rhs.numer * (denom / rhs.denom) as i64;
        Self::new(numer, denom)
    }
}

impl Sub for Fraction {
    type Output = Fraction;

    fn sub(self, rhs: Self) -> Fraction {
        let denom = lcm(self.denom, rhs.denom);
        let numer =
            self.numer * (denom / self.denom) as i64 - rhs.numer * (denom / rhs.denom) as i64;
        Self::new(numer, denom)
    }
}

impl Mul for Fraction {
    type Output = Fraction;

    fn mul(self, rhs: Self) -> Fraction {
        Self::new(self.numer * rhs.numer, self.denom * rhs.denom)
    }
}

impl Div for Fraction {
    type Output = Fraction;

    fn div(self, rhs: Self) -> Self::Output {
        // i think this is right?
        let mut numer = self.numer * rhs.denom as i64;
        let denom;
        if rhs.numer < 0 {
            // case: rhs.numer < 0 or (self.numer < 0 and rhs.numer < 0)
            denom = rhs.numer.unsigned_abs() * self.denom;
            numer = -numer;
        } else {
            // case: all positive or self.numer < 0
            denom = rhs.numer as u64 * self.denom;
        }
        assert!(denom > 0);
        Self::new(numer, denom)
    }
}

// Return the greatest common divisor of a and b
#[allow(clippy::comparison_chain)]
fn gcd(a: u64, b: u64) -> u64 {
    assert!(a > 0 && b > 0, "a: {a}, b: {b}");
    if a == b {
        a
    } else if a < b {
        gcd(a, b - a)
    } else {
        gcd(a - b, b)
    }
}

// NOTE: Probably better
// fn gcd(a: u64, b: u64) -> u64 {
//     if a % b == 0 {
//         b
//     } else {
//         gcd(b, a % b)
//     }
// }

// NOTE: Rust recommended, probably fastest
// fn gcd(x: u64, y: u64) -> u64 {
//     let mut x = x;
//     let mut y = y;
//     while y != 0 {
//         let t = y;
//         y = x % y;
//         x = t;
//     }
//     x
// }

// Return the least common multiple of a and b
fn lcm(a: u64, b: u64) -> u64 {
    assert!(a > 0 && b > 0, "a: {a}, b: {b}");
    // abs(a*b)/gcd(a, b)
    a * b / gcd(a, b)
    // _lcm(a, b, a, b)
}

#[allow(clippy::comparison_chain)]
fn _lcm(a_sum: u64, b_sum: u64, a: u64, b: u64) -> u64 {
    if a_sum == b_sum {
        a_sum
    } else if a_sum < b_sum {
        _lcm(a_sum + a, b_sum, a, b)
    } else {
        _lcm(a_sum, b_sum + b, a, b)
    }
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
        assert_eq!(lcm(4, 6), 12);
    }

    #[test]
    fn gcd_test() {
        assert_eq!(gcd(4, 6), 2);
        assert_eq!(gcd(8, 12), 4);
        assert_eq!(gcd(12, 8), 4);
        assert_eq!(gcd(54, 24), 6);
    }

    #[test]
    fn equality() {
        let a = Fraction::from_int(5);
        let b = Fraction::new(10, 2);
        assert_eq!(a, b);
    }

    #[test]
    fn add() {
        let a = Fraction::from_int(5);
        let b = Fraction::from_int(13);
        assert_eq!(a + b, Fraction::from_int(18));
    }
}
