use super::field_element::{reduce, FieldElement, PRIME, ZERO};
use sha3::{
    digest::{ExtendableOutput, Update},
    Shake256,
};

/// This function generates the round constants for the Rescue hash function.
#[allow(dead_code)]
pub fn compute_round_constants<const RATE: usize, const WIDTH: usize, const N: usize>(
    security_level: usize,
) -> [[[FieldElement; WIDTH]; N]; 2] {
    // compute the number of bytes needed to represent a single FieldElement
    // bytes_per_field = $ceil(|p| / 8) + 1#
    // where |p| is the number of bits in the PRIME
    let bytes_per_field = 9;
    let num_bytes = bytes_per_field * WIDTH * N * 2;

    // number of fields elements not impacted during absorb operation.
    let capacity = WIDTH - RATE;

    // seed_string = "Rescue - XLIX (p, w, c, security_level)" mentioned in the paper.
    let seed_string = format!("Rescue - XLIX ({},{},{},{}", PRIME, WIDTH, capacity, security_level);

    let seed_bytes = seed_string.as_bytes();
    let byte_string = shake256(seed_bytes, num_bytes);

    let mut round_constants = [[[ZERO; WIDTH]; N]; 2];

    // process byte_string into chunks of bytes_per_field.
    for i in 0..2 * WIDTH * N {
        let bytes_chunk = byte_string[i * bytes_per_field..(i + 1) * bytes_per_field].to_vec();

        let constant = bytes_chunk
            .iter()
            .enumerate()
            .map(|(i, val)| {
                // calculate the integer value of the chunk using least significant byte first encoding.
                let pow = 256u128.pow(i as u32);
                let zz = *val as u128;
                pow * zz
            })
            .sum::<u128>();

        // reduce the constant to a `u64` value.
        let reduced_constant = reduce(constant);

        round_constants[i / (WIDTH * N)][i / WIDTH % N][i % WIDTH] =
            FieldElement::new(reduced_constant);
    }

    round_constants
}

// HELPER METHODS
/// ================================================================================================

/// This function returns a `Box<[u8]>` of `num_bytes` length. The bytes are generated using the
/// SHAKE256 hash function.
#[allow(dead_code)]
fn shake256(input: &[u8], num_bytes: usize) -> Box<[u8]> {
    let mut hasher = Shake256::default();
    hasher.update(input);
    hasher.finalize_boxed(num_bytes)
}
