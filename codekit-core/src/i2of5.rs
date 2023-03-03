use std::{collections::HashMap, error::Error, fmt::Display, marker::PhantomData};

use lazy_static::lazy_static;

use crate::{commons::Barcode, Code};

lazy_static! {
    static ref CHARACTERS_MAP: HashMap<char, u8> = {
        let mut m = HashMap::new();

        m.insert('0', 0b00110);
        m.insert('1', 0b10001);
        m.insert('2', 0b01001);
        m.insert('3', 0b11000);
        m.insert('4', 0b00101);
        m.insert('5', 0b10100);
        m.insert('6', 0b01100);
        m.insert('7', 0b00011);
        m.insert('8', 0b10010);
        m.insert('9', 0b01010);

        m
    };
}
#[derive(Debug, Clone, Copy)]
pub enum I2of5Error {
    InvalidMessage,
}

impl Display for I2of5Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            I2of5Error::InvalidMessage => write!(f, "the message is invalid"),
        }
    }
}
impl Error for I2of5Error {}

pub struct I2of5<'a> {
    _data: &'a PhantomData<u8>,
}

impl<'a> I2of5<'a> {
    fn parse_message(message: &'a str) -> Result<Vec<u8>, I2of5Error> {
        if message.contains(|c| !CHARACTERS_MAP.contains_key(&c)) && message.len() != 14 {
            Err(I2of5Error::InvalidMessage)
        } else {
            let converted: Vec<_> = message
                .chars()
                .flat_map(|c| CHARACTERS_MAP.get(&c))
                .copied()
                .collect();

            Ok(converted)
        }
    }

    fn interleaved_converted_patterns(pattern: Vec<u8>) -> Result<Vec<u8>, I2of5Error> {
        if pattern.len() % 2 != 0 {
            return Err(I2of5Error::InvalidMessage);
        }

        let mut interleaved_pattern: Vec<u8> = vec![];

        for i in 0..pattern.len() / 2 {
            let bar_pattern = pattern[2 * i];
            let spaces_pattern = pattern[2 * i + 1];

            for j in 0..5 {
                let bar_value = (1 << (4 - j) & bar_pattern) >> (4 - j);
                interleaved_pattern.push(bar_value);

                let space_value = (1 << (4 - j) & spaces_pattern) >> (4 - j);
                interleaved_pattern.push(space_value);
            }
        }

        let converted_patterns: Vec<_> = interleaved_pattern
            .into_iter()
            .enumerate()
            .flat_map(|(offset, narrow_wide)| {
                let value = if offset % 2 == 0 { 1u8 } else { 0u8 };

                if narrow_wide != 0 {
                    return vec![value, value];
                } else {
                    return vec![value];
                }
            })
            .collect();

        Ok(converted_patterns)
    }
}
impl<'a> Barcode for I2of5<'a> {
    type Input = &'a str;

    type Error = I2of5Error;

    fn make_descriptor(
        input: Self::Input,
        options: crate::CodeOptions,
    ) -> Result<crate::Code, Self::Error> {
        let patterns = I2of5::parse_message(input)?;

        let mut converted = I2of5::interleaved_converted_patterns(patterns)?;

        let mut bars = vec![1, 0, 1, 0];
        bars.append(&mut converted);
        bars.append(&mut vec![1, 1, 0, 1]);

        Ok(Code::new(bars, options))
    }
}

#[cfg(test)]
mod tests {
    use super::I2of5;

    #[test]
    fn test_patterns() {
        let inputs: Vec<u8> = vec![0b10001, 0b01001]; // "12"
        let expected: Vec<u8> = vec![1, 1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 1, 0, 0];
        let computed = I2of5::interleaved_converted_patterns(inputs).expect("valid conversion");

        assert_eq!(expected, computed);
    }

    #[test]
    fn test_patterns2() {
        let inputs: Vec<u8> = vec![
            0b10001, // 1
            0b01001, // 2
            0b11000, // 3
            0b00101, // 4
            0b10100, // 5
            0b01100, // 6
            0b00011, // 7
            0b00110, // 0
        ];

        let expected: Vec<u8> = vec![
            1, 1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 1, 0, 0, // "12"
            1, 1, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0, // "34"
            1, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, // "56"
            1, 0, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, // "70"
        ];

        let computed = I2of5::interleaved_converted_patterns(inputs).expect("valid conversion");

        assert_eq!(expected, computed);
    }
}
