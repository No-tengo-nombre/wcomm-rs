pub const PI: f32 = 3.14157296;

const fn num_bits<T>() -> usize { std::mem::size_of::<T>() * 8 }

pub fn log2(x: u32) -> u32 {
    num_bits::<u32>() as u32 - x.leading_zeros() - 1
}
