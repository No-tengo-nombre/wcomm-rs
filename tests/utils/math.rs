use wcomm::utils::math::*;
use pretty_assertions::assert_eq;

#[test]
fn test_num_bits() {
    assert_eq!(num_bits::<f32>(), 32);
    assert_eq!(num_bits::<f64>(), 64);

    assert_eq!(num_bits::<i8>(), 8);
    assert_eq!(num_bits::<i16>(), 16);
    assert_eq!(num_bits::<i32>(), 32);
    assert_eq!(num_bits::<i64>(), 64);
    assert_eq!(num_bits::<i128>(), 128);

    assert_eq!(num_bits::<u8>(), 8);
    assert_eq!(num_bits::<u16>(), 16);
    assert_eq!(num_bits::<u32>(), 32);
    assert_eq!(num_bits::<u64>(), 64);
    assert_eq!(num_bits::<u128>(), 128);
}

#[test]
fn test_log2() {
    assert_eq!(log2(1), 0);
    assert_eq!(log2(1), 0);
    assert_eq!(log2(2), 1);
    assert_eq!(log2(3), 1);
    assert_eq!(log2(4), 2);
    assert_eq!(log2(5), 2);
    assert_eq!(log2(6), 2);
    assert_eq!(log2(7), 2);
    assert_eq!(log2(8), 3);
    assert_eq!(log2(9), 3);
    assert_eq!(log2(10), 3);
    assert_eq!(log2(11), 3);
    assert_eq!(log2(12), 3);
    assert_eq!(log2(13), 3);
    assert_eq!(log2(14), 3);
    assert_eq!(log2(15), 3);
    assert_eq!(log2(16), 4);
}
