extern crate split_into;

use split_into::SplitInto;

#[test]
fn test_split_into_zero_divisor() {
    assert_eq!(Err("Divisor is zero"),        0.split_into(0));
}

#[test]
fn test_split_into_positive() {
    assert_eq!(Ok(vec!(10)),                  10.split_into(1));
    assert_eq!(Ok(vec!(5,5)),                 10.split_into(2));
    assert_eq!(Ok(vec!(3,3,4)),               10.split_into(3));
    assert_eq!(Ok(vec!(2,2,3,3)),             10.split_into(4));
    assert_eq!(Ok(vec!(2,2,2,2,2)),           10.split_into(5));
    assert_eq!(Ok(vec!(1,1,2,2,2,2)),         10.split_into(6));
    assert_eq!(Ok(vec!(1,1,1,1,2,2,2)),       10.split_into(7));
    assert_eq!(Ok(vec!(1,1,1,1,1,1,2,2)),     10.split_into(8));
    assert_eq!(Ok(vec!(1,1,1,1,1,1,1,1,2)),   10.split_into(9));
    assert_eq!(Ok(vec!(1,1,1,1,1,1,1,1,1,1)), 10.split_into(10));
}

#[test]
fn test_split_into_negative() {
    assert_eq!(Ok(vec!(-10)),                           (-10).split_into(1));
    assert_eq!(Ok(vec!(-5,-5)),                         (-10).split_into(2));
    assert_eq!(Ok(vec!(-3,-3,-4)),                      (-10).split_into(3));
    assert_eq!(Ok(vec!(-2,-2,-3,-3)),                   (-10).split_into(4));
    assert_eq!(Ok(vec!(-2,-2,-2,-2,-2)),                (-10).split_into(5));
    assert_eq!(Ok(vec!(-1,-1,-2,-2,-2,-2)),             (-10).split_into(6));
    assert_eq!(Ok(vec!(-1,-1,-1,-1,-2,-2,-2)),          (-10).split_into(7));
    assert_eq!(Ok(vec!(-1,-1,-1,-1,-1,-1,-2,-2)),       (-10).split_into(8));
    assert_eq!(Ok(vec!(-1,-1,-1,-1,-1,-1,-1,-1,-2)),    (-10).split_into(9));
    assert_eq!(Ok(vec!(-1,-1,-1,-1,-1,-1,-1,-1,-1,-1)), (-10).split_into(10));
}

#[test]
fn test_split_into_zero_dividend() {
    assert_eq!(Ok(vec!(0)),                   0.split_into(1));
    assert_eq!(Ok(vec!(0,0)),                 0.split_into(2));
    assert_eq!(Ok(vec!(0,0,0)),               0.split_into(3));
    assert_eq!(Ok(vec!(0,0,0,0)),             0.split_into(4));
    assert_eq!(Ok(vec!(0,0,0,0,0)),           0.split_into(5));
    assert_eq!(Ok(vec!(0,0,0,0,0,0)),         0.split_into(6));
    assert_eq!(Ok(vec!(0,0,0,0,0,0,0)),       0.split_into(7));
    assert_eq!(Ok(vec!(0,0,0,0,0,0,0,0)),     0.split_into(8));
    assert_eq!(Ok(vec!(0,0,0,0,0,0,0,0,0)),   0.split_into(9));
    assert_eq!(Ok(vec!(0,0,0,0,0,0,0,0,0,0)), 0.split_into(10));
}
