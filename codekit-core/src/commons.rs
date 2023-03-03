use std::ops::BitAnd;

pub fn map_bits_to_vec<T: Sized + BitAnd<u8> + Copy>(value: T, pattern_size: usize) -> Vec<u8>
where
    <T as BitAnd<u8>>::Output: PartialEq<u8>,
{
    (0..pattern_size)
        .map(|i| {
            let b = value & (1 << (pattern_size - 1 - i));
            if b == 0 {
                0
            } else {
                1
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::map_bits_to_vec;

    #[test]
    fn test_commons() {
        let value: u8 = 0b1110000;
        let bits = map_bits_to_vec(value, 7);
        assert_eq!(vec![1, 1, 1, 0, 0, 0, 0], bits);
    }

    #[test]
    fn test_commons_2() {
        let value: u8 = 0b0001011;
        let bits = map_bits_to_vec(value, 7);
        assert_eq!(vec![0, 0, 0, 1, 0, 1, 1], bits);
    }
}
