extern crate split_into;

use split_into::split_into;

#[test]
fn test_split_into_zero() {
    assert_eq!(Err("Divisor is zero"),        split_into(10,0));
}

#[test]
fn test_split_into() {
    assert_eq!(Ok(vec!(10)),                  split_into(10,1));
    assert_eq!(Ok(vec!(5,5)),                 split_into(10,2));
    assert_eq!(Ok(vec!(3,3,4)),               split_into(10,3));
    assert_eq!(Ok(vec!(2,2,3,3)),             split_into(10,4));
    assert_eq!(Ok(vec!(2,2,2,2,2)),           split_into(10,5));
    assert_eq!(Ok(vec!(1,1,2,2,2,2)),         split_into(10,6));
    assert_eq!(Ok(vec!(1,1,1,1,2,2,2)),       split_into(10,7));
    assert_eq!(Ok(vec!(1,1,1,1,1,1,2,2)),     split_into(10,8));
    assert_eq!(Ok(vec!(1,1,1,1,1,1,1,1,2)),   split_into(10,9));
    assert_eq!(Ok(vec!(1,1,1,1,1,1,1,1,1,1)), split_into(10,10));
}
