extern crate split_into;

use split_into::error::Error;
use split_into::SplitInto;

#[test]
fn test_split_into_zero_divisor() {
    assert_eq!(Err(Error::DivisorIsZero), 0.split_into(0));
}

macro_rules! split_test {
    ($($type:ty),*) => {
        $(
            let number: $type = 10;
            assert_eq!(Ok(vec!(10)), number.split_into(1));
            assert_eq!(Ok(vec!(5, 5)), number.split_into(2));
            assert_eq!(Ok(vec!(3, 3, 4)), number.split_into(3));
            assert_eq!(Ok(vec!(2, 2, 3, 3)), number.split_into(4));
            assert_eq!(Ok(vec!(2, 2, 2, 2, 2)), number.split_into(5));
            assert_eq!(Ok(vec!(1, 1, 2, 2, 2, 2)), number.split_into(6));
            assert_eq!(Ok(vec!(1, 1, 1, 1, 2, 2, 2)), number.split_into(7));
            assert_eq!(Ok(vec!(1, 1, 1, 1, 1, 1, 2, 2)), number.split_into(8));
            assert_eq!(Ok(vec!(1, 1, 1, 1, 1, 1, 1, 1, 2)), number.split_into(9));
            assert_eq!(
                Ok(vec!(1, 1, 1, 1, 1, 1, 1, 1, 1, 1)),
                number.split_into(10)
            );
        )*
    }
}

#[test]
fn test_split_into_positive() {
    split_test!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
}

#[test]
fn test_split_into_negative() {
    assert_eq!(Ok(vec!(-10)), (-10).split_into(1));
    assert_eq!(Ok(vec!(-5, -5)), (-10).split_into(2));
    assert_eq!(Ok(vec!(-3, -3, -4)), (-10).split_into(3));
    assert_eq!(Ok(vec!(-2, -2, -3, -3)), (-10).split_into(4));
    assert_eq!(Ok(vec!(-2, -2, -2, -2, -2)), (-10).split_into(5));
    assert_eq!(Ok(vec!(-1, -1, -2, -2, -2, -2)), (-10).split_into(6));
    assert_eq!(Ok(vec!(-1, -1, -1, -1, -2, -2, -2)), (-10).split_into(7));
    assert_eq!(
        Ok(vec!(-1, -1, -1, -1, -1, -1, -2, -2)),
        (-10).split_into(8)
    );
    assert_eq!(
        Ok(vec!(-1, -1, -1, -1, -1, -1, -1, -1, -2)),
        (-10).split_into(9)
    );
    assert_eq!(
        Ok(vec!(-1, -1, -1, -1, -1, -1, -1, -1, -1, -1)),
        (-10).split_into(10)
    );
}

#[test]
fn test_split_into_zero_dividend() {
    assert_eq!(Ok(vec!(0)), 0.split_into(1));
    assert_eq!(Ok(vec!(0, 0)), 0.split_into(2));
    assert_eq!(Ok(vec!(0, 0, 0)), 0.split_into(3));
    assert_eq!(Ok(vec!(0, 0, 0, 0)), 0.split_into(4));
    assert_eq!(Ok(vec!(0, 0, 0, 0, 0)), 0.split_into(5));
    assert_eq!(Ok(vec!(0, 0, 0, 0, 0, 0)), 0.split_into(6));
    assert_eq!(Ok(vec!(0, 0, 0, 0, 0, 0, 0)), 0.split_into(7));
    assert_eq!(Ok(vec!(0, 0, 0, 0, 0, 0, 0, 0)), 0.split_into(8));
    assert_eq!(Ok(vec!(0, 0, 0, 0, 0, 0, 0, 0, 0)), 0.split_into(9));
    assert_eq!(Ok(vec!(0, 0, 0, 0, 0, 0, 0, 0, 0, 0)), 0.split_into(10));
}

#[test]
fn test_split_into_negative_divisor() {
    assert_eq!(Err(Error::DivisorIsNegative), 10.split_into(-2));
}
