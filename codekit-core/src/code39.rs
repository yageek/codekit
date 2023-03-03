use lazy_static::lazy_static;
use std::{collections::HashMap, error::Error, fmt::Display};

use crate::commons::Barcode;
lazy_static! {
    /// See https://en.wikipedia.org/wiki/Code_39
    static ref CHARACTERS_MAP: HashMap<char, &'static str> = {
        let mut m = HashMap::new();
        m.insert('A', "100001001");
        m.insert('B', "001001001");
        m.insert('C', "101001000");
        m.insert('D', "000011001");
        m.insert('E', "100011000");
        m.insert('F', "001011000");
        m.insert('G', "000001101");
        m.insert('H', "100001100");
        m.insert('I', "001001100");
        m.insert('J', "000011100");
        m.insert('K', "100000011");
        m.insert('L', "001000011");
        m.insert('M', "101000010");
        m.insert('N', "000010011");
        m.insert('O', "100010010");
        m.insert('P', "001010010");
        m.insert('Q', "000000111");
        m.insert('R', "100000110");
        m.insert('S', "001000110");
        m.insert('T', "000010110");
        m.insert('U', "110000001");
        m.insert('V', "011000001");
        m.insert('W', "111000000");
        m.insert('X', "010010001");
        m.insert('Y', "110010000");
        m.insert('Z', "011010000");
        m.insert('0', "000110100");
        m.insert('1', "100100001");
        m.insert('2', "001100001");
        m.insert('3', "101100000");
        m.insert('4', "000110001");
        m.insert('5', "100110000");
        m.insert('6', "001110000");
        m.insert('7', "000100101");
        m.insert('8', "100100100");
        m.insert('9', "001100100");
        m.insert(' ', "011000100");
        m.insert('-', "010000101");
        m.insert('$', "010101000");
        m.insert('%', "000101010");
        m.insert('.', "110000100");
        m.insert('/', "010100010");
        m.insert('+', "010001010");
        m.insert('*', "010010100");
        m
    };
}

#[derive(Debug, Clone, Copy)]
pub enum Code39Error {
    InvalidMessage,
}
impl Display for Code39Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Code39Error::InvalidMessage => write!(f, "the message is invalid"),
        }
    }
}
impl Error for Code39Error {}

pub struct Code39;

impl Code39 {
    fn parse_message(message: &str) -> Result<Vec<&str>, Code39Error> {
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
impl Barcode for Code39 {
    type Error = Code39Error;
    fn make_descriptor(input: &str) -> Result<String, Code39Error> {
        let bars = Code39::parse_message(input)?;

        let mut grouped = String::new();
        let size = bars.len();

        for (i, bar_group) in bars.into_iter().enumerate() {
            grouped.push_str(bar_group);
            if i < size - 1 {
                grouped.push('0');
            }
        }
        Ok(grouped)
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

        let expected = [
            "010010100",
            "111000000",
            "001001100",
            "100000011",
            "001001100",
            "010010100",
        ];

        assert_eq!(expected[..], value);
    }
}
