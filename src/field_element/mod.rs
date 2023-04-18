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

    /// Return the inverse of the FieldElement. According to the Fermat Little
    /// Theorem, the inverse of a number is the number raised to the power of
    /// PRIME - 2.
    ///
    /// Mathematically, this is equivalent to:
    ///             $a^(p-1)     = 1 (mod p)$
    ///             $a^(p-2) * a = 1 (mod p)$
    /// Therefore   $a^(p-2)     = a^{-1} (mod p)$
    ///
    /// This is a very fast way to calculate the inverse of a number and happens
    /// to in constant time.
    pub fn inv(&self) -> Self {
        unimplemented!("FieldElement::inv")
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
        // Add the values and check for overflow. If there is overflow, then
        // subtract PRIME from the result.
        let (result, carry) = self.value().overflowing_add(other.value());

        // the summation of two field elements is always less than 2*PRIME therefore
        // the carry is always 0 or 1. If the carry is 1, then the result is greater
        // than PRIME and we need to subtract PRIME from the result.
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
        // Subtract the values and check for overflow. If there is underflow, then
        // add PRIME to the result.
        let (result, carry) = self.value().overflowing_sub(other.value());

        // If the carry is 1, then the result is less than 0 and we need to add
        // PRIME to the result.
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

        // If the value is 0, then the negation is 0. Otherwise, the negation is
        // PRIME - value.
        if value == 0 {
            Self { value }
        } else {
            Self {
                value: PRIME - value,
            }
        }
    }
}
