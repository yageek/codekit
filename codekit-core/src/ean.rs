use std::{error::Error, fmt::Display, marker::PhantomData};

use crate::commons::{map_bits_to_vec, Barcode, Code, CodeOptions};

//  See [IAN/EAN wikipedia page](https://en.wikipedia.org/wiki/International_Article_Number)
const EAN_PATTERNS: [[u8; 3]; 10] = [
    // [L-code,   G-Code,  Right-code
    [0b0001101, 0b0100111, 0b1110010], //  0
    [0b0011001, 0b0110011, 0b1100110], //  1
    [0b0010011, 0b0011011, 0b1101100], //  2
    [0b0111101, 0b0100001, 0b1000010], //  3
    [0b0100011, 0b0011101, 0b1011100], //  4
    [0b0110001, 0b0111001, 0b1001110], //  5
    [0b0101111, 0b0000101, 0b1010000], //  6
    [0b0111011, 0b0010001, 0b1000100], //  7
    [0b0110111, 0b0001001, 0b1001000], //  8
    [0b0001011, 0b0010111, 0b1110100], //  9
];

#[derive(Debug, Clone, Copy)]
enum EANPatternCode {
    L,
    G,
    R,
}

impl EANPatternCode {
    fn index_in_table(&self) -> usize {
        match self {
            EANPatternCode::L => 0,
            EANPatternCode::G => 1,
            EANPatternCode::R => 2,
        }
    }
}

const LEFT_ENCODE_MAP: [u8; 10] = [
    // L = 0 -> Use L-code for left part of ean13
    // G = 1 -> Use G-code for left part of ean13
    // (The right part use always the R-code
    0b000000, // 0
    0b001011, // 1
    0b001101, // 2
    0b001110, // 3
    0b010011, // 4
    0b011001, // 5
    0b011100, // 6
    0b010101, // 7
    0b010110, // 8
    0b011010, // 9
];

#[derive(Debug, Copy, Clone)]
struct EANCode<'a> {
    left: &'a [u8],
    right: &'a [u8],
    left_pattern: u8,
}

#[derive(Debug, Clone, Copy)]
pub enum EANParseError {
    /// The number of element is invalid
    WrongSize,
    InvalidChecksum,
    UnexpectedCharacter(char),
}

impl Display for EANParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EANParseError::WrongSize => write!(
                f,
                "the number of expected digits does not match the type of code"
            ),
            EANParseError::InvalidChecksum => write!(f, "the checksum of the barcode is invalid"),
            EANParseError::UnexpectedCharacter(ch) => {
                write!(f, "an invalid character has been meet: {}", ch)
            }
        }
    }
}

impl Error for EANParseError {}
impl<'a> EANCode<'a> {
    /// Parse one string for EAN values and return a vector of string
    fn parse_digit(code_str: &str) -> Result<Vec<u8>, EANParseError> {
        code_str
            .chars()
            .filter(|c| !c.eq(&'-'))
            .map(|ch| {
                if !ch.is_ascii() || !ch.is_ascii_digit() {
                    Err(EANParseError::UnexpectedCharacter(ch))
                } else {
                    match ch.to_digit(10) {
                        None => Err(EANParseError::UnexpectedCharacter(ch)),
                        Some(value) => Ok(value as u8),
                    }
                }
            })
            .collect()
    }

    fn validate_ean8_checksum(values: &[u8]) -> Result<(), EANParseError> {
        let check = values.iter().enumerate().fold(0, |check, (index, value)| {
            let adder = if index % 2 == 0 { (*value) * 3 } else { *value };
            (check + adder) % 10
        });

        if check == 0 {
            Ok(())
        } else {
            Err(EANParseError::InvalidChecksum)
        }
    }

    fn validate_ean13_checksum(values: &[u8]) -> Result<(), EANParseError> {
        let check = values.iter().enumerate().fold(0, |check, (index, value)| {
            let adder = if index % 2 == 0 { *value } else { value * 3 };
            (check + adder) % 10
        });

        if check == 0 {
            Ok(())
        } else {
            Err(EANParseError::InvalidChecksum)
        }
    }

    fn new_ean8(values: &[u8]) -> Result<EANCode, EANParseError> {
        if values.len() != 8 {
            Err(EANParseError::WrongSize)
        } else {
            EANCode::validate_ean8_checksum(values)?;

            let code = EANCode {
                left: &values[0..=3],
                right: &values[4..=7],
                left_pattern: LEFT_ENCODE_MAP[0],
            };
            Ok(code)
        }
    }

    fn new_ean13(values: &[u8]) -> Result<EANCode, EANParseError> {
        if values.len() != 13 {
            Err(EANParseError::WrongSize)
        } else {
            EANCode::validate_ean13_checksum(values)?;

            let index = values[0] as usize;

            let code = EANCode {
                left: &values[1..=6],
                right: &values[7..=12],
                left_pattern: LEFT_ENCODE_MAP[index],
            };
            Ok(code)
        }
    }

    fn to_code(self, options: CodeOptions) -> Result<Code, EANParseError> {
        let mut bars: Vec<u8> = vec![1, 0, 1];

        let mut left_elements: Vec<_> = self
            .left
            .iter()
            .enumerate()
            .flat_map(|(index, left_value)| {
                let number_index = *left_value as usize;
                let pattern_index =
                    if (self.left_pattern as i8) & (1 << (self.left.len() - index - 1)) == 0 {
                        EANPatternCode::L
                    } else {
                        EANPatternCode::G
                    };

                let bar_value = EAN_PATTERNS[number_index][pattern_index.index_in_table()];
                map_bits_to_vec(bar_value, 7)
            })
            .collect();

        bars.append(&mut left_elements);

        // Middle guard
        bars.append(&mut vec![0, 1, 0, 1, 0]);

        // Now right
        let mut right_elements = self
            .right
            .iter()
            .flat_map(|value| {
                let number_index = *value as usize;
                let bar_value = EAN_PATTERNS[number_index][EANPatternCode::R.index_in_table()];
                map_bits_to_vec(bar_value, 7)
            })
            .collect();

        bars.append(&mut right_elements);

        // End guards
        bars.append(&mut vec![1, 0, 1]);
        Ok(Code::new(bars, options))
    }
}

pub struct EAN8<'a> {
    _data: &'a PhantomData<u8>,
}

impl<'a> Barcode for EAN8<'a> {
    type Error = EANParseError;
    type Input = &'a str;

    fn make_descriptor(input: &str, options: crate::CodeOptions) -> Result<Code, EANParseError> {
        let digits = EANCode::parse_digit(input)?;
        let desc = EANCode::new_ean8(&digits)?;
        desc.to_code(options)
    }
}

pub struct EAN13<'a> {
    _data: &'a PhantomData<u8>,
}

impl<'a> Barcode for EAN13<'a> {
    type Error = EANParseError;
    type Input = &'a str;

    fn make_descriptor(input: &str, options: crate::CodeOptions) -> Result<Code, EANParseError> {
        let digits = EANCode::parse_digit(input)?;
        let desc = EANCode::new_ean13(&digits)?;
        desc.to_code(options)
    }
}

#[cfg(test)]
mod tests {
    use crate::{commons::Barcode, CodeOptions};

    use super::{EANCode, EAN13};

    #[test]
    fn test_parse() {
        let input = "4719-5127";
        let elements = EANCode::parse_digit(input).expect("parsing should work");
        assert_eq!([4, 7, 1, 9, 5, 1, 2, 7], &elements[..]);
    }

    #[test]
    fn test_ean13_check() {
        let values = EANCode::parse_digit("978-0-306-40615-7").unwrap();
        assert!(EANCode::validate_ean13_checksum(&values).is_ok());
    }

    #[test]
    fn test_ean8_check() {
        let values = EANCode::parse_digit("4719-5127").unwrap();
        assert!(EANCode::validate_ean8_checksum(&values).is_ok());
    }

    #[test]
    fn test_ean8_description() {
        let elements = EANCode::parse_digit("6583-3254").unwrap();
        let desc = EANCode::new_ean8(&elements).expect("the EAN8 description should be valid");

        assert_eq!([6, 5, 8, 3], desc.left);
        assert_eq!([3, 2, 5, 4], desc.right);
        assert_eq!(0, desc.left_pattern);
    }

    #[test]
    fn test_ean13_description() {
        let elements = EANCode::parse_digit("5901234123457").unwrap();
        let desc = EANCode::new_ean13(&elements).expect("the EAN8 description should be valid");

        assert_eq!([9, 0, 1, 2, 3, 4], desc.left);
        assert_eq!([1, 2, 3, 4, 5, 7], desc.right);
        assert_eq!(0b011001, desc.left_pattern);
    }

    #[test]
    fn test_descriptor() {
        let code = EAN13::make_descriptor("5901234123457", CodeOptions::default()).unwrap();
        let expected: Vec<u8> = vec![
            1, 0, 1, 0, 0, 0, 1, 0, 1, 1, 0, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 0, 0,
            1, 1, 0, 1, 1, 1, 1, 0, 1, 0, 0, 1, 1, 1, 0, 1, 0, 1, 0, 1, 0, 1, 1, 0, 0, 1, 1, 0, 1,
            1, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 1, 0, 1, 1, 1, 0, 0, 1, 0, 0, 1, 1, 1, 0, 1, 0,
            0, 0, 1, 0, 0, 1, 0, 1,
        ];

        assert_eq!(&code.bars(), &expected);
    }
}
