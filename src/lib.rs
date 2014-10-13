pub fn split_into(dividend: uint, divisor: uint) -> Vec<uint> {
    if divisor == 0 { return Vec::from_elem(0, 0)};

    let quotient = dividend / divisor;
    let remainder = dividend % divisor;

    let mut parts: Vec<uint> = Vec::from_elem(divisor, quotient);

    for x in range(0, remainder) {
        *parts.get_mut(x) = parts[x] + 1;
    }

    parts.reverse();
    return parts;
}
