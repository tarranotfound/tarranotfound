use rust_cli_toolkit::calculator::calculate;

#[test]
fn adds_two_numbers() {
    assert_eq!(calculate(2.0, '+', 3.0).unwrap(), 5.0);
}

#[test]
fn subtracts_two_numbers() {
    assert_eq!(calculate(9.0, '-', 4.0).unwrap(), 5.0);
}

#[test]
fn multiplies_two_numbers() {
    assert_eq!(calculate(3.0, '*', 4.0).unwrap(), 12.0);
}

#[test]
fn divides_two_numbers() {
    assert_eq!(calculate(8.0, '/', 2.0).unwrap(), 4.0);
}

#[test]
fn rejects_division_by_zero() {
    assert!(calculate(1.0, '/', 0.0).is_err());
}
