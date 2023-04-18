use super::{FieldElement, ONE, PRIME, ZERO};

#[test]
fn test_addition() {
    // ---- Test addition with no carry ----------------------------------------

    let a = FieldElement::new(15);
    let b = FieldElement::new(35);

    let result = a + b;
    assert_eq!(result.value, 50);

    // ------ Test addition with field overflow -----------------------------------------

    let a = FieldElement::new(PRIME) - ONE;
    let b = FieldElement::new(25);

    let result = a + b;
    assert_eq!(result.value, 24);

    // ------ Test addition with u64 overflow -----------------------------------------

    let a = FieldElement::new(PRIME - 1);
    let b = FieldElement::new(PRIME - 2);

    let result = a + b;
    assert_eq!(result.value, PRIME - 3);
}

#[test]
fn test_subtraction() {
    // ---- Test subtraction with no carry ----------------------------------------

    let a = FieldElement::new(15);
    let b = FieldElement::new(35);

    let result = b - a;
    assert_eq!(result.value, 20);

    // ------ Test subtraction with field underflow -----------------------------------------

    let a = FieldElement::new(15);
    let b = FieldElement::new(35);

    let result = a - b;
    assert_eq!(result.value, PRIME - 20);
}

#[test]
fn test_negation() {
    let a = FieldElement::new(15);

    let result = -a;
    assert_eq!(result.value, PRIME - 15);
    assert_eq!(a + result, ZERO);
}
