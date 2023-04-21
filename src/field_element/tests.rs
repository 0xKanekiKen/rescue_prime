use super::{errors::FieldError, FieldElement, ONE, PRIME, ZERO};

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

#[test]
fn test_mul() {
    // test multiplication by zero and one
    let r: FieldElement = FieldElement::new(5);
    assert_eq!(ZERO, r * ZERO);
    assert_eq!(r, r * ONE);

    // test basic multiplication
    assert_eq!(FieldElement::from(15u8), FieldElement::from(5u8) * FieldElement::from(3u8));

    // test multiplication which is guaranted to overflow
    let m = PRIME;
    let t = FieldElement::from(m - 1);
    assert_eq!(ONE, t * t);
    assert_eq!(FieldElement::from(m - 2), t * FieldElement::from(2u8));
    assert_eq!(FieldElement::from(m - 4), t * FieldElement::from(4u8));

    let t = (m + 1) / 2;
    assert_eq!(ONE, FieldElement::from(t) * FieldElement::from(2u8));
}

#[test]
fn inv() {
    // --------- test inverse of identity elements ----------------------------

    assert_eq!(ONE, ONE.inv());

    // --------- test inverses of field elements -------------------------------

    let r: FieldElement = FieldElement::new(5);
    assert_eq!(FieldElement::new(14757395255531667457), r.inv());
}

#[test]
fn exp() {
    let a = ZERO;
    assert_eq!(a.exp(ZERO), ONE);
    assert_eq!(a.exp(ONE), ZERO);

    let a = ONE;
    assert_eq!(a.exp(ZERO), ONE);
    assert_eq!(a.exp(ONE), ONE);
    assert_eq!(a.exp(FieldElement { value: 3 }), ONE);

    let a: FieldElement = FieldElement { value: 69 };
    assert_eq!(a.exp(FieldElement::from(6u8)), a * a * a * a * a * a);
}

#[test]
fn test_square() {
    let r: FieldElement = FieldElement::new(5);
    assert_eq!(FieldElement::from(25u8), r.square());
}

#[test]
fn test_cube() {
    let r: FieldElement = FieldElement::new(5);
    assert_eq!(FieldElement::from(125u8), r.cube());
}

#[test]
fn test_double() {
    let r: FieldElement = FieldElement::new(99);
    assert_eq!(FieldElement::from(198u8), r.double());

    let r: FieldElement = FieldElement::new(PRIME - 1);
    assert_eq!(FieldElement::from(PRIME - 2), r.double());
}

#[test]
fn test_to_bytes() {
    let r: FieldElement = ZERO;
    assert_eq!(r.to_bytes(), [0u8; 8]);

    let r: FieldElement = ONE;
    assert_eq!(r.to_bytes(), [0, 0, 0, 0, 0, 0, 0, 1]);

    let r: FieldElement = FieldElement::new(PRIME - 1);
    assert_eq!(r.to_bytes(), [255, 255, 255, 255, 0, 0, 0, 0]);
}

#[test]
fn test_from_bytes() {
    let bytes = [255, 255, 255, 255, 0, 0, 0, 0];
    match FieldElement::from_bytes(&bytes) {
        Ok(fe) => assert_eq!(fe, FieldElement::new(PRIME - 1)),
        Err(_) => assert!(false),
    }

    let bytes = [255, 255, 255, 255, 0, 0, 0, 1];
    match FieldElement::from_bytes(&bytes) {
        Ok(_) => assert!(false),
        Err(e) => assert_eq!(e, FieldError::InvalidValue),
    }
}

#[test]
fn test_try_from() {
    test_from_bytes();
}
