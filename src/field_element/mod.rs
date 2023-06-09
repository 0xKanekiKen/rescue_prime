use core::convert::TryFrom;
use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

#[cfg(test)]
mod tests;

use crate::utils::errors::FieldError;

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

#[allow(dead_code)]
impl FieldElement {
    /// Create a new FieldElement. If the value is >= PRIME, then the value is
    /// reduced modulo PRIME.
    pub const fn new(value: u64) -> FieldElement {
        FieldElement {
            value: value % PRIME,
        }
    }

    /// Return the value of the FieldElement.
    #[inline]
    pub fn value(&self) -> u64 {
        self.value
    }

    /// Return the summation of the field element with itself.
    #[inline]
    pub fn double(&self) -> Self {
        let (res, carry) = self.value.overflowing_add(self.value);
        Self::new(res.wrapping_sub(PRIME * (carry as u64)))
    }

    /// Return the exponentiation of the field element with `pow` field element.
    #[inline]
    pub fn exp(self, pow: Self) -> Self {
        let mut base = self;

        if pow == ZERO {
            return ONE;
        } else if base == ZERO {
            return ZERO;
        }

        // TODO: come up with an implementation that takes constant time to execute.
        // This implementation is not constant time.
        // Checks if the least significant bit is 1. If it is, then the result is
        // the base. Otherwise, the result is 1.
        let mut res = if (pow.value & 1) == 1 { base } else { ONE };

        // Shift the bits of the exponent to the right by 1.
        let mut pow_val = pow.value >> 1;

        // While the exponent is greater than 0, square the base and multiply the
        // result by the base if the least significant bit of the exponent is 1.
        // Then, shift the bits of the exponent to the right by 1. This is repeated
        // until the exponent is 0.
        //
        // Mathematically, this is equivalent to:
        //             $a^b = a^{b_0 + 2b_1 + 4b_2 + ... + 2^{k-1}b_{k-1}}$
        //             $a^b = a^{b_0} * a^{2b_1} * a^{4b_2} * ... * a^{2^{k-1}b_{k-1}}$
        // Therefore   $a^b = a^{b_0} * a^{b_1}^2 * a^{b_2}^4 * ... * a^{b_{k-1}}^{2^{k-1}}$
        while pow_val > 0 {
            base = base.square();
            if (pow_val & 1) == 1 {
                res *= base;
            }
            pow_val >>= 1;
        }

        res
    }

    /// Return the inverse of the FieldElement. According to the Fermat Little
    /// Theorem, the inverse of a number is the number raised to the power of
    /// PRIME - 2.
    ///
    /// NOTE: The inverse of zero is undefined. The caller must ensure that
    ///       this function is never called with the zero element.
    ///
    /// Mathematically, this is equivalent to:
    ///             $a^(p-1)     = 1 (mod p)$
    ///             $a^(p-2) * a = 1 (mod p)$
    /// Therefore   $a^(p-2)     = a^{-1} (mod p)$
    ///
    /// This is a very fast way to calculate the inverse of a number and happens in constant time.
    ///
    /// Adapted from: https://github.com/facebook/winterfell/blob/d238a1ecc8da42179d0b8a06c0d4a510256aa0a6/math/src/field/f64/mod.rs#L136-L164
    #[inline]
    pub fn inv(self) -> Self {
        debug_assert!(self != ZERO, "The inverse of zero is undefined.");

        // compute base^(M - 2) using 72 multiplications
        // The exponent M - 2 is represented in binary as:
        // 0b1111111111111111111111111111111011111111111111111111111111111111

        // compute base^11
        let t2 = self.cube();

        // compute base^111
        let t3 = t2.square() * self;

        // compute base^111111 (6 ones)
        // repeatedly square t3 3 times and multiply by t3
        let t6 = exp_acc::<3>(t3, t3);

        // compute base^111111111111 (12 ones)
        // repeatedly square t6 6 times and multiply by t6
        let t12 = exp_acc::<6>(t6, t6);

        // compute base^111111111111111111111111 (24 ones)
        // repeatedly square t12 12 times and multiply by t12
        let t24 = exp_acc::<12>(t12, t12);

        // compute base^1111111111111111111111111111111 (31 ones)
        // repeatedly square t24 7 times and multiply by t6
        let t30 = exp_acc::<6>(t24, t6);
        let t31 = t30.square() * self;

        // compute base^111111111111111111111111111111101111111111111111111111111111111
        // repeatedly square t31 31 times and multiply by t31
        let t63 = exp_acc::<32>(t31, t31);

        // compute base^1111111111111111111111111111111011111111111111111111111111111111
        t63.square() * self
    }

    /// Returns the square of the FieldElement which is equivalent to multiplying the FieldElement by itself.
    pub fn square(&self) -> Self {
        self.mul(*self)
    }

    /// Returns the cube of the FieldElement which is equivalent to multiplying the FieldElement by itself twice.
    pub fn cube(&self) -> Self {
        self.square().mul(*self)
    }

    /// Serialize the FieldElement into a little-endian byte array of size 8.
    pub fn to_bytes(self) -> [u8; 8] {
        self.value.to_le_bytes()
    }

    /// Deserialize the FieldElement from a little-endian byte array of size 8.
    pub fn from_bytes(arr: &[u8; 8]) -> Result<Self, FieldError> {
        let value = u64::from_le_bytes(*arr);
        if value >= PRIME {
            Err(FieldError::DeserializationError)
        } else {
            Ok(Self::new(value))
        }
    }
}

/// Implement the Display trait for FieldElement.
impl Display for FieldElement {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
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

/// Implement Add, AddAssign, Div, DivAssign, Neg, Mul, MulAssign, Sub, SubAssign for
/// FieldElements. These operations are performed modulo PRIME.
impl Add for FieldElement {
    type Output = Self;

    #[inline]
    #[allow(clippy::suspicious_arithmetic_impl)]
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

impl Mul for FieldElement {
    type Output = Self;

    #[inline]
    fn mul(self, other: FieldElement) -> FieldElement {
        Self::new(reduce((self.value as u128) * (other.value as u128)))
    }
}

impl Div for FieldElement {
    type Output = Self;

    #[inline]
    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(self, other: FieldElement) -> FieldElement {
        self * other.inv()
    }
}

impl DivAssign for FieldElement {
    #[inline]
    fn div_assign(&mut self, other: FieldElement) {
        *self = *self / other;
    }
}

impl MulAssign for FieldElement {
    #[inline]
    fn mul_assign(&mut self, other: FieldElement) {
        *self = *self * other;
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

impl Sub for FieldElement {
    type Output = Self;

    #[inline]
    #[allow(clippy::suspicious_arithmetic_impl)]
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

// TYPE CONVERSIONS
// =============================================================================

impl From<u64> for FieldElement {
    fn from(x: u64) -> Self {
        Self::new(x)
    }
}

impl From<u32> for FieldElement {
    fn from(x: u32) -> Self {
        Self::new(x as u64)
    }
}

impl From<u16> for FieldElement {
    fn from(x: u16) -> Self {
        Self::new(x as u64)
    }
}

impl From<u8> for FieldElement {
    fn from(x: u8) -> Self {
        Self::new(x as u64)
    }
}

// From<[u8;8]> is not implemented since it will have to be very
// opinionated about the endianness of the bytes and perform
// modulo PRIME silently.

impl TryFrom<[u8; 8]> for FieldElement {
    type Error = FieldError;

    fn try_from(bytes: [u8; 8]) -> Result<Self, Self::Error> {
        Self::from_bytes(&bytes)
    }
}

// HELPER FUNCTIONS
// =============================================================================

/// This function reduces a 128-bit number modulo PRIME, based on the instructions at the link below.
/// https://cp4space.hatsya.com/2021/09/01/an-efficient-prime-for-number-theoretic-transforms/
#[inline]
fn reduce(x: u128) -> u64 {
    // Split the 128-bit number into 3 parts, such that the number can be written as follows.
    // x = low + 2^64 * middle + 2^96 * high
    let low: u64 = x as u64; // low 64 bits
    let middle_high: u64 = (x >> 64) as u64; // remaining 64 bits
    let middle: u64 = (middle_high as u32) as u64; // of which, low 32 bits
    let high: u64 = middle_high >> 32; // of which, high 32 bits

    // In the finite field with modulus PRIME, we know that:
    // p = 2^64 - 2^32 + 1, or
    // 2^64 = p + 2^32 - 1
    // Multiplying both sides by 2^32, we get:
    // 2^96 = 2^32 * (p + 2^32 - 1)
    // 2^96 = 2^32 * p + (2^64 - 2^32), or
    // 2^96 = 2^32 * p + (p - 1), or
    // 2^96 ≡ -1 (mod p)
    // Replace 2^96 with this value in the equation for x.
    // x ≡ low + 2^64 * middle - high
    // x ≡ low - high + 2^64 * middle
    let (diff, under) = low.overflowing_sub(high);
    // If an underflow occurred, then we need to add PRIME to the result.
    let diff = diff.wrapping_add((under as u64) * PRIME);

    // Using a similar analysis as above, we can show that:
    // 2^64 = p + 2^32 - 1, or that 2^64 ≡ 2^32 - 1
    // Replace 2^64 with this value in the equation for x.
    // x ≡ low - high + (2^32 - 1) * middle
    let product = (middle << 32) - middle;

    // Add the product to low - high.
    let (result, over) = diff.overflowing_add(product);

    // If an overflow occurred, then we need to subtract PRIME from the result.
    result.wrapping_sub((over as u64) * PRIME)
}

/// Squares the base N number of times and multiplies the result by the tail value.
#[inline(always)]
fn exp_acc<const N: usize>(base: FieldElement, tail: FieldElement) -> FieldElement {
    let mut result = base;
    for _ in 0..N {
        result = result.square();
    }
    result * tail
}
