use std::ops::BitAnd;

/// An internal type for encoder
pub(crate) trait Barcode {
    type Input;
    type Error;

    /// Return the descriptor for the code
    fn make_descriptor(input: Self::Input, options: CodeOptions) -> Result<Code, Self::Error>;
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CodeOptions {
    pub quiet_space: u16,
    pub code_height: u16,
    pub border_width: u16,
}

impl Default for CodeOptions {
    fn default() -> Self {
        CodeOptions {
            quiet_space: 7,
            code_height: 50,
            border_width: 0,
        }
    }
}

/// A generic code descriptor
/// to draw code bar
pub struct Code {
    options: CodeOptions,
    bars: Vec<u8>,
}

impl Code {
    pub(crate) fn new(bars: Vec<u8>, options: CodeOptions) -> Code {
        Code { options, bars }
    }
    /// Retrieve the border witdh of the code
    pub fn border_width(&self) -> u16 {
        self.options.border_width
    }

    pub fn height(&self) -> u16 {
        self.options.code_height
    }

    pub fn quiet_space(&self) -> u16 {
        self.options.quiet_space
    }

    pub fn bars(&self) -> &[u8] {
        &self.bars
    }

    pub fn options(&self) -> CodeOptions {
        self.options
    }

    #[cfg(feature = "ffi-interface")]
    pub fn get_bars(self) -> Vec<u8> {
        self.bars
    }
}

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
