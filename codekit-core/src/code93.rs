use std::{collections::HashMap, error::Error, fmt::Display, marker::PhantomData};

use lazy_static::lazy_static;

use crate::commons::Barcode;

lazy_static! {
    /// See: https://web.archive.org/web/20090225114452/http://www.barcodeisland.com/code93.phtml
    static ref CHARACTERS_MAP: HashMap<char, &'static str> = {
        let mut m = HashMap::new();
        m.insert('0', "100010100");
        m.insert('1', "101001000");
        m.insert('2', "101000100");
        m.insert('3', "101000010");
        m.insert('4', "100101000");
        m.insert('5', "100100100");
        m.insert('6', "100100010");
        m.insert('7', "101010000");
        m.insert('8', "100010010");
        m.insert('9', "100001010");
        m.insert('A', "110101000");
        m.insert('B', "110100100");
        m.insert('C', "110100010");
        m.insert('D', "110010100");
        m.insert('E', "110010010");
        m.insert('F', "110001010");
        m.insert('G', "101101000");
        m.insert('H', "101100100");
        m.insert('I', "101100010");
        m.insert('J', "100110100");
        m.insert('K', "100011010");
        m.insert('L', "101011000");
        m.insert('M', "101001100");
        m.insert('N', "101000110");
        m.insert('O', "100101100");
        m.insert('P', "100010110");
        m.insert('Q', "110110100");
        m.insert('R', "110110010");
        m.insert('S', "110101100");
        m.insert('T', "110100110");
        m.insert('U', "110010110");
        m.insert('V', "110011010");
        m.insert('W', "101101100");
        m.insert('X', "101100110");
        m.insert('Y', "100110110");
        m.insert('Z', "100111010");
        m.insert('-', "100101110");
        m.insert('.', "111010100");
        m.insert(' ', "111010010");
        m.insert('$', "111001010");
        m.insert('/', "101101110");
        m.insert('+', "101110110");
        m.insert('%', "110101110");
        m.insert('*', "101011110");
        m
    };
}

static INDEX_MAP: &[char] = &[
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I',
    'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '-', '.',
    ' ', '$', '/', '+', '%', '*',
];
#[derive(Debug, Clone, Copy)]
pub enum Code93Error {
    InvalidMessage,
}
impl Display for Code93Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Code93Error::InvalidMessage => write!(f, "the message is invalid"),
        }
    }
}
impl Error for Code93Error {}

pub struct Code93<'a> {
    _data: &'a PhantomData<u8>,
}

impl<'a> Code93<'a> {
    fn compute_checksum(message: &str, weight_module: usize) -> char {
        let mut sum = 0;

        for (index, ch) in message.chars().enumerate() {
            let weight = (message.len() - index) % weight_module;
            let index = INDEX_MAP.iter().position(|c| *c == ch).unwrap();
            sum += index * weight;
        }
        let value = sum % 47;
        INDEX_MAP[value]
    }

    fn compute_c(message: &str) -> char {
        Code93::compute_checksum(message, 20)
    }

    fn compute_k(message: &str) -> char {
        Code93::compute_checksum(message, 15)
    }

    fn parse_message(message: &str) -> Result<Vec<&'static str>, Code93Error> {
        let mut message = message.to_uppercase();

        if message
            .find(|c| c == '*' || !CHARACTERS_MAP.contains_key(&c))
            .is_some()
        {
            Err(Code93Error::InvalidMessage)
        } else {
            message.push(Code93::compute_c(&message));
            message.push(Code93::compute_k(&message));

            message.insert(0, '*');
            message.insert(message.len(), '*');

            let converted = message
                .chars()
                .flat_map(|c| CHARACTERS_MAP.get(&c))
                .copied()
                .collect();

            Ok(converted)
        }
    }
}

impl<'a> Barcode for Code93<'a> {
    type Error = Code93Error;

    fn make_descriptor(input: &str) -> Result<String, Self::Error> {
        let patterns = Code93::parse_message(input)?;

        let mut bars: String = patterns.into_iter().to_owned().collect();

        bars.push('1');
        Ok(bars)
    }
}

#[cfg(test)]
mod tests {
    use crate::commons::Barcode;

    use super::Code93;

    #[test]
    fn test_code_check() {
        assert_eq!('+', Code93::compute_c("TEST93"));
        assert_eq!('6', Code93::compute_c("TEST93+"));
    }

    #[test]
    fn test_compute_elements() {
        Code93::make_descriptor("TEST93").expect("invalid element");
    }
}
