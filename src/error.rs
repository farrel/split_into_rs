#[derive(Debug, PartialEq)]
pub enum Error {
    DivisorIsZero,
    DivisorLargerThanDividend,
    DivisorIsNegative,
}
