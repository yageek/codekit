use std::{collections::HashMap, error::Error, fmt::Display, marker::PhantomData};

use lazy_static::lazy_static;

use crate::{
    commons::{map_bits_to_vec_u16, Barcode},
    Code,
};

lazy_static! {
    /// See: https://web.archive.org/web/20090225114452/http://www.barcodeisland.com/code93.phtml
    static ref CHARACTERS_MAP: HashMap<char, u16> = {
        let mut m = HashMap::new();
        m.insert('0', 0b100010100);
        m.insert('1', 0b101001000);
        m.insert('2', 0b101000100);
        m.insert('3', 0b101000010);
        m.insert('4', 0b100101000);
        m.insert('5', 0b100100100);
        m.insert('6', 0b100100010);
        m.insert('7', 0b101010000);
        m.insert('8', 0b100010010);
        m.insert('9', 0b100001010);
        m.insert('A', 0b110101000);
        m.insert('B', 0b110100100);
        m.insert('C', 0b110100010);
        m.insert('D', 0b110010100);
        m.insert('E', 0b110010010);
        m.insert('F', 0b110001010);
        m.insert('G', 0b101101000);
        m.insert('H', 0b101100100);
        m.insert('I', 0b101100010);
        m.insert('J', 0b100110100);
        m.insert('K', 0b100011010);
        m.insert('L', 0b101011000);
        m.insert('M', 0b101001100);
        m.insert('N', 0b101000110);
        m.insert('O', 0b100101100);
        m.insert('P', 0b100010110);
        m.insert('Q', 0b110110100);
        m.insert('R', 0b110110010);
        m.insert('S', 0b110101100);
        m.insert('T', 0b110100110);
        m.insert('U', 0b110010110);
        m.insert('V', 0b110011010);
        m.insert('W', 0b101101100);
        m.insert('X', 0b101100110);
        m.insert('Y', 0b100110110);
        m.insert('Z', 0b100111010);
        m.insert('-', 0b100101110);
        m.insert('.', 0b111010100);
        m.insert(' ', 0b111010010);
        m.insert('$', 0b111001010);
        m.insert('/', 0b101101110);
        m.insert('+', 0b101110110);
        m.insert('%', 0b110101110);
        m.insert('*', 0b101011110);
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

    fn parse_message(message: &'a str) -> Result<Vec<u16>, Code93Error> {
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

    fn make_descriptor(
        input: &str,
        options: crate::CodeOptions,
    ) -> Result<crate::Code, Self::Error> {
        let patterns = Code93::parse_message(input)?;

        let mut bars: Vec<_> = patterns
            .into_iter()
            .flat_map(|pattern| map_bits_to_vec_u16(pattern, 9))
            .collect();

        bars.push(1);
        Ok(Code::new(bars, options))
    }
}

#[cfg(test)]
mod tests {
    use crate::{commons::Barcode, CodeOptions};

    use super::Code93;

    #[test]
    fn test_code_check() {
        assert_eq!('+', Code93::compute_c("TEST93"));
        assert_eq!('6', Code93::compute_c("TEST93+"));
    }

    #[test]
    fn test_compute_elements() {
        Code93::make_descriptor("TEST93", CodeOptions::default()).expect("invalid element");
    }
}
