pub mod error;

use error::Error;

pub trait SplitInto {
    fn split_into(&self, divisor: Self) -> Result<Vec<Self>, Error>
    where
        Self: Sized;
}

impl SplitInto for i64 {
    fn split_into(&self, divisor: i64) -> Result<Vec<i64>, Error> {
        let dividend = *self;

        match divisor {
            0 => return Err(Error::DivisorIsZero),
            1 => return Ok(vec![dividend]),
            _ => (),
        }

        if dividend == 0 {
            return Ok(vec![0; divisor.try_into().unwrap()]);
        }
        if divisor > dividend {
            return Err(Error::DivisorLargerThanDividend);
        }

        let quotient = dividend / divisor;
        let remainder = dividend.abs() % divisor;
        let adjustment = if dividend > 0 { 1 } else { -1 };

        let mut parts: Vec<i64> = vec![quotient; divisor.try_into().unwrap()];
        let start = parts.len() - remainder as usize;
        let finish = parts.len() - 1;

        for x in start..finish {
            parts[x] = parts[x] + adjustment;
        }

        Ok(parts)
    }
}
