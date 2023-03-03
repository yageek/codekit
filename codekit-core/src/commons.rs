use std::ops::BitAnd;

/// An internal type for encoder
pub(crate) trait Barcode {
    type Error: std::error::Error;

    /// Return the descriptor for the code
    fn make_descriptor(input: &str, options: CodeOptions) -> Result<Code, Self::Error>;
}

/// A structure holding
/// the display information to render a bar
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CodeOptions {
    /// The quiet space around the bar code
    pub quiet_space: u16,
    /// The height of the code
    pub code_height: u16,
    /// The border with of the code
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

    #[cfg(any(feature = "ffi-interface", feature = "jni-interface"))]
    pub fn get_bars(self) -> Vec<u8> {
        self.bars
    }
}

/// From a set of bit in one ``u8``, returns the bits
/// inside an array (0b1101 -> [1, 1, 0, 1])
/// - Parameter value: The ``u8``
/// - Returns: An array of ``u8``
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

pub fn map_bits_to_vec_u16<T: Sized + BitAnd<u16> + Copy>(value: T, pattern_size: usize) -> Vec<u8>
where
    <T as BitAnd<u16>>::Output: PartialEq<u16>,
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
/// Map a character pattern to bars
/// This will map the patterns from bits to descriptors.
/// For one pattern, one bit represents alternatively a gap and a bar
/// 0 means narrow bar/gap and 1 means wide large bar/gap.
///
/// 1 large bar/gap should be at least twice wider than narrow one (max 3 times wider)
/// We start with bar representation and alternate.
/// *Ex*: Character 'A' => 100001001
///
/// #1 bit: 1 - Wide bar
/// #2 bit: 0 - Narrow gap
/// #3 bit: 0 - Narrow bar
/// #4 bit: 0 - Narrow gap
///
/// Between each character we have one narrow gap
/// - Parameter pattern: The character pattern from the internal map
/// - Returns: The bars to display. **NOTE**: `bars` means bars ready for the BarcodeGenerator filter.
/// This means it handle the rules about the gap/bar thickness so you will get more bars at the output
/// than at the input
pub fn narrow_wide_gar_bar<T: Sized + BitAnd<u16> + Copy + std::ops::Sub<u16>>(
    pattern: T,
    pattern_size: usize,
) -> Vec<u8>
where
    <T as BitAnd<u16>>::Output: PartialEq<u16>,
{
    let mut bars: Vec<u8> = vec![];

    for i in 0..pattern_size {
        let value = if i % 2 == 0 { 1u8 } else { 0u8 };

        let is_wide = (pattern & 1 << ((pattern_size) - 1 - i)) != 0;

        bars.push(value);
        if is_wide {
            bars.push(value);
            bars.push(value);
        }
    }
    bars
}

pub fn narrow_wide_gar_bar_u8<T: Sized + BitAnd<u8> + Copy + std::ops::Sub<u8>>(
    pattern: T,
    pattern_size: usize,
) -> Vec<u8>
where
    <T as BitAnd<u8>>::Output: PartialEq<u8>,
{
    let mut bars: Vec<u8> = vec![];

    for i in 0..pattern_size {
        let value = if i % 2 == 0 { 1u8 } else { 0u8 };

        let is_wide = (pattern & 1 << ((pattern_size) - 1 - i)) != 0;

        bars.push(value);
        if is_wide {
            bars.push(value);
            bars.push(value);
        }
    }
    bars
}

#[cfg(test)]
mod tests {
    use super::{map_bits_to_vec, narrow_wide_gar_bar};

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

    #[test]
    fn test_bars_map() {
        let input: u16 = 0b100001001;
        let expected: Vec<u8> = vec![
            1, 1, 1, // 1
            0, // 0
            1, // 0
            0, // 0
            1, // 0
            0, 0, 0, // 1
            1, // 0
            0, // 0
            1, 1, 1, // 1
        ];

        assert_eq!(expected, narrow_wide_gar_bar(input, 9));

        let input: u16 = 0b010010100;

        // *
        let expected: Vec<u8> = vec![
            1, // 0
            0, 0, 0, // 1
            1, // 0
            0, // 0
            1, 1, 1, // 1
            0, // 0
            1, 1, 1, // 1
            0, // 0
            1, // 0
        ];
        assert_eq!(expected, narrow_wide_gar_bar(input, 9));
    }
}
