static DIVISOR_IS_ZERO: &'static str = "Divisor is zero";

pub fn split_into(dividend: uint, divisor: uint) -> Result<Vec<uint>, &'static str> {
    if divisor == 0 { return Err(DIVISOR_IS_ZERO)};

    let quotient = dividend / divisor;
    let remainder = dividend % divisor;

    let mut parts: Vec<uint> = Vec::from_elem(divisor as uint, quotient);

    for x in range(0, remainder) {
        *parts.get_mut(x) = parts[x] + 1;
    }

    parts.reverse();
    return Ok(parts);
}
