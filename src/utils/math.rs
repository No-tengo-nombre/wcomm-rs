/*!
 * Mathematical functions for the package.
 */

/**
 * Mathematical constant, corresponding to the circunference of a circle
 * of radius 0.5.
 */
pub const PI: f32 = 3.14157296;

/**
 * Get the number of bits associated to the type.
 */
const fn num_bits<T>() -> usize {
    return std::mem::size_of::<T>() * 8;
}

/**
 * Calculate the logarithm base 2 rounded down. Useful when you know
 * only power-of-2 inputs will be given.
 */
pub fn log2(x: u32) -> u32 {
    num_bits::<u32>() as u32 - x.leading_zeros() - 1
}

/**
 * Transform an integer to its gray coded equivalent.
 */
pub fn num_to_gray(num: u32) -> u32 {
    return num ^ (num >> 1);
}

/**
 * Transform a gray coded number to its integer equivalent.
 */
pub fn gray_to_num(num: u32) -> u32 {
    let mut result = num;
    let mut mask = result;
    while mask != 0 {
        mask >>= 1;
        result ^= mask;
    }
    return result;
}

/**
 * Transform an integer to its gray coded equivalent binary representation.
 */
pub fn num_to_gray_binary(num: u32) -> String {
    let mut den = 2;
    let mut output = String::from("");
    while den <= num {
        output = format!("{}{}", (num as f32 / den as f32).round() as u32 % 2, output);
        den *= 2;
    }
    return format!("{}{}", (num as f32 / den as f32).round() as u32 % 2, output);
}
