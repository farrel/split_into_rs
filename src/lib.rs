pub mod error;

use error::Error;

pub trait SplitInto {
    fn split_into(&self, divisor: Self) -> Result<Vec<Self>, Error>
    where
        Self: Sized;
}

macro_rules! split_into {
    ($($type:ty),*) => {
        $(
        impl SplitInto for $type {
            fn split_into(&self, divisor: Self) -> Result<Vec<Self>, Error> {
                let dividend = *self;

                match divisor {
                    0 => return Err(Error::DivisorIsZero),
                    1 => return Ok(vec![dividend]),
                    _ => (),
                }

                if dividend == 0 {
                    return Ok(vec![0; divisor.try_into().unwrap()]);
                }
                if divisor > dividend.abs() {
                    return Err(Error::DivisorLargerThanDividend);
                }

                let quotient = dividend / divisor;
                let remainder = dividend.abs() % divisor;
                let adjustment = if dividend > 0 { 1 } else { -1 };

                let mut parts: Vec<Self> = vec![quotient; divisor.try_into().unwrap()];
                if remainder == 0 {
                    return Ok(parts);
                }
                let start = parts.len() - remainder as usize;
                let finish = parts.len();

                for x in start..finish {
                    parts[x] += adjustment;
                }

                Ok(parts)
            }
        })*
    };
}

split_into!(i8, i16, i32, i64, isize);
