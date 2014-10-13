#[test]
fn test_split_into() {
    assert_eq!(Vec::from_elem(0,0),       split_into(10,0));
    assert_eq!(vec!(10),                  split_into(10,1));
    assert_eq!(vec!(5,5),                 split_into(10,2));
    assert_eq!(vec!(3,3,4),               split_into(10,3));
    assert_eq!(vec!(2,2,3,3),             split_into(10,4));
    assert_eq!(vec!(2,2,2,2,2),           split_into(10,5));
    assert_eq!(vec!(1,1,2,2,2,2),         split_into(10,6));
    assert_eq!(vec!(1,1,1,1,2,2,2),       split_into(10,7));
    assert_eq!(vec!(1,1,1,1,1,1,2,2),     split_into(10,8));
    assert_eq!(vec!(1,1,1,1,1,1,1,1,2),   split_into(10,9));
    assert_eq!(vec!(1,1,1,1,1,1,1,1,1,1), split_into(10,10));
}

fn split_into(dividend: uint, divisor: uint) -> Vec<uint> {
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
