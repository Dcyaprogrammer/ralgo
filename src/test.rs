#[cfg(test)]
use crate::Expression;

#[test]
fn test_1(){
    let s = Expression::from_str("1");
    assert_eq!(s.to_string(), "1");
}

#[test]
fn test_2(){
    let s = Expression::from_str("1 + 2 * 3");
    assert_eq!(s.to_string(), "(+ 1 (* 2 3))");
}