pub const PI: f32 = 3.14157296;

const fn num_bits<T>() -> usize {
    return std::mem::size_of::<T>() * 8;
}

pub fn log2(x: u32) -> u32 {
    num_bits::<u32>() as u32 - x.leading_zeros() - 1
}

pub fn num_to_gray(num: u32) -> u32 {
    let output = num_to_gray_binary(num);
    return u32::from_str_radix(&output, 2).unwrap();
}

pub fn num_to_gray_binary(num: u32) -> String {
    let mut den = 2;
    let mut output = String::from("");
    while den <= num {
        output = format!("{}{}", (num as f32 / den as f32).round() as u32 % 2, output);
        den *= 2;
    }
    return format!("{}{}", (num as f32 / den as f32).round() as u32 % 2, output);
}
