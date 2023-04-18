use std::{
    fmt::{Display, Formatter, Result},
    ops::{Add, AddAssign, Neg, Sub, SubAssign},
};

#[cfg(test)]
mod tests;

// CONSTANTS
// =============================================================================

/// Prime number that defines the field the FieldElement is in. It is 2^64 - 2^32 + 1.
const PRIME: u64 = 0xFFFFFFFF00000001;

const ZERO: FieldElement = FieldElement { value: 0 };
const ONE: FieldElement = FieldElement { value: 1 };

// STRUCTS
// =============================================================================

/// A field element is a number in the range 0..=MODULUS-1. It is represented
/// as a u64, but it is not valid to create a FieldElement with a value >=
/// PRIME.
#[derive(Clone, Copy, Debug)]
struct FieldElement {
    value: u64,
}

/// IMPLEMENTATIONS
/// =============================================================================

impl FieldElement {
    /// Create a new FieldElement. If the value is >= PRIME, then the value is
    /// reduced modulo PRIME.
    pub fn new(value: u64) -> FieldElement {
        FieldElement {
            value: value % PRIME,
        }
    }

    /// Return the value of the FieldElement.
    pub fn value(&self) -> u64 {
        self.value
    }
}

/// Implement the Display trait for FieldElement.
impl Display for FieldElement {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.value())
    }
}

/// Implement the PartialEq trait for FieldElement.
impl PartialEq for FieldElement {
    #[inline]
    fn eq(&self, other: &FieldElement) -> bool {
        self.value() == other.value()
    }
}

/// Implement Add, AddAssign, Sub, SubAssign, and Neg for FieldElement. These
/// operations are performed modulo PRIME.
impl Add for FieldElement {
    type Output = Self;

    #[inline]
    fn add(self, other: FieldElement) -> Self {
        let (result, carry) = self.value().overflowing_add(other.value());
        Self::new(result.wrapping_sub(PRIME * (carry as u64)))
    }
}

impl AddAssign for FieldElement {
    #[inline]
    fn add_assign(&mut self, other: FieldElement) {
        *self = *self + other;
    }
}

impl Sub for FieldElement {
    type Output = Self;

    #[inline]
    fn sub(self, other: FieldElement) -> FieldElement {
        let (result, carry) = self.value().overflowing_sub(other.value());
        Self::new(result.wrapping_add(PRIME * (carry as u64)))
    }
}

impl SubAssign for FieldElement {
    #[inline]
    fn sub_assign(&mut self, other: FieldElement) {
        *self = *self - other;
    }
}

impl Neg for FieldElement {
    type Output = Self;

    #[inline]
    fn neg(self) -> FieldElement {
        let value = self.value();
        if value == 0 {
            Self { value }
        } else {
            Self {
                value: PRIME - value,
            }
        }
    }
}
