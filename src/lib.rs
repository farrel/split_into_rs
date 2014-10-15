static DIVISOR_IS_ZERO: &'static str              = "Divisor is zero";
static DIVISOR_LARGER_THAN_DIVIDEND: &'static str = "Divisor is larger than the dividend";

pub fn split_into(dividend: int, divisor: uint) -> Result<Vec<int>, &'static str> {
    if divisor == 0                     { return Err(DIVISOR_IS_ZERO) };
    if dividend == 0                    { return Ok(Vec::from_elem(divisor,0i))};
    if divisor > dividend.abs() as uint { return Err(DIVISOR_LARGER_THAN_DIVIDEND) }

    let quotient  = dividend / divisor as int;
    let remainder = dividend.abs() as uint % divisor;

    let mut parts: Vec<int> = Vec::from_elem(divisor, quotient);

    let adjustment = if dividend > 0 { 1i } else { -1i }; 

    for x in range(0, remainder as uint) {
        *parts.get_mut(x) = parts[x] + adjustment;
    }

    parts.reverse();
    return Ok(parts);
}
