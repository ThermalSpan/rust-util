
use std::ops::Rem;
use std::fmt;
use std::error::Error;

#[derive(Debug)]
struct FactorError {
    description: String,
}

impl fmt::Display for FactorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}

impl Error for FactorError {
    fn description(&self) -> &str {
        &self.description
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}

fn gcd(a: u32, b: u32) -> u32 {
    let mut r: [u32; 3] = [0; 3];

    r[1] = a.rem(b);
    r[0] = b.rem(r[1]);

    while r[0] != 0 {
        r[2] = r[1];
        r[1] = r[0];
        r[0] = r[2].rem(r[1]);
    }

    r[1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gcd_1() {
        assert_eq!(gcd(6, 9), 3);
    }
    
    #[test]
    fn gcd_2() {
    }

    #[test]
    fn gcd_3() {
        assert_eq!(gcd(55, 15), 5);
    }

    #[test]
    fn gcd_4() {
        assert_eq!(gcd(906,755), 151);
        assert_eq!(gcd(1661,906), 151);
        assert_eq!(gcd(25821,1661), 151);
        assert_eq!(gcd(163231,135749), 151);
    }
}
