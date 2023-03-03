use lazy_static::lazy_static;
use std::{collections::HashMap, marker::PhantomData};

use crate::{
    commons::{narrow_wide_gar_bar, Barcode},
    Code,
};

lazy_static! {
    static ref CHARACTERS_MAP: HashMap<char, u16> = {
        let mut m = HashMap::new();
        m.insert('A', 0b100001001);
        m.insert('B', 0b001001001);
        m.insert('C', 0b101001000);
        m.insert('D', 0b000011001);
        m.insert('E', 0b100011000);
        m.insert('F', 0b001011000);
        m.insert('G', 0b000001101);
        m.insert('H', 0b100001100);
        m.insert('I', 0b001001100);
        m.insert('J', 0b000011100);
        m.insert('K', 0b100000011);
        m.insert('L', 0b001000011);
        m.insert('M', 0b101000010);
        m.insert('N', 0b000010011);
        m.insert('O', 0b100010010);
        m.insert('P', 0b001010010);
        m.insert('Q', 0b000000111);
        m.insert('R', 0b100000110);
        m.insert('S', 0b001000110);
        m.insert('T', 0b000010110);
        m.insert('U', 0b110000001);
        m.insert('V', 0b011000001);
        m.insert('W', 0b111000000);
        m.insert('X', 0b010010001);
        m.insert('Y', 0b110010000);
        m.insert('Z', 0b011010000);
        m.insert('0', 0b000110100);
        m.insert('1', 0b100100001);
        m.insert('2', 0b001100001);
        m.insert('3', 0b101100000);
        m.insert('4', 0b000110001);
        m.insert('5', 0b100110000);
        m.insert('6', 0b001110000);
        m.insert('7', 0b000100101);
        m.insert('8', 0b100100100);
        m.insert('9', 0b001100100);
        m.insert(' ', 0b011000100);
        m.insert('-', 0b010000101);
        m.insert('$', 0b010101000);
        m.insert('%', 0b000101010);
        m.insert('.', 0b110000100);
        m.insert('/', 0b010100010);
        m.insert('+', 0b010001010);
        m.insert('*', 0b010010100);
        m
    };
}
#[derive(Debug, Clone, Copy)]
pub enum Code39Error {
    InvalidMessage,
}

pub struct Code39<'a> {
    _data: &'a PhantomData<u8>,
}

impl<'a> Code39<'a> {
    fn parse_message(message: &'a str) -> Result<Vec<u16>, Code39Error> {
        let mut message = message.to_uppercase();
        if !&message.starts_with('*') {
            message.insert(0, '*');
        }

        if !&message.ends_with('*') {
            message.push('*');
        }

        message
            .chars()
            .map(|c| {
                if let Some(value) = CHARACTERS_MAP.get(&c) {
                    Ok(*value)
                } else {
                    Err(Code39Error::InvalidMessage)
                }
            })
            .collect()
    }
}
impl<'a> Barcode for Code39<'a> {
    type Error = Code39Error;
    type Input = &'a str;

    fn make_descriptor(input: &str, options: crate::CodeOptions) -> Result<Code, Code39Error> {
        let patterns = Code39::parse_message(input)?;

        let bars: Vec<_> = patterns
            .into_iter()
            .map(|pattern| narrow_wide_gar_bar(pattern, 9))
            .collect();

        let mut grouped = vec![];
        let size = bars.len();

        for (i, mut bar_group) in bars.into_iter().enumerate() {
            grouped.append(&mut bar_group);
            if i < size - 1 {
                grouped.push(0);
            }
        }
        Ok(Code::new(grouped, options))
    }
}

#[cfg(test)]
mod tests {
    use super::Code39;

    #[test]
    fn test_parsing() {
        // Some invalid messages
        Code39::parse_message("*[referef*").expect_err("should panic here");

        // Some valid messages
        Code39::parse_message("referef").expect("should not panic here");
        Code39::parse_message("*referfef*").expect("should not panic here");
        Code39::parse_message("*+-*/.%- *").expect("should not panic here");
        // Test Value
        let value = Code39::parse_message("*wiki*").unwrap();

        let expected: Vec<u16> = vec![
            0b010010100,
            0b111000000,
            0b001001100,
            0b100000011,
            0b001001100,
            0b010010100,
        ];

        assert_eq!(expected, value);
    }
}
