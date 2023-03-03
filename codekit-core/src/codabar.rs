use std::{collections::HashMap, error::Error, fmt::Display, marker::PhantomData};

use lazy_static::lazy_static;

use crate::commons::Barcode;
lazy_static! {
    /// See: https://web.archive.org/web/20090122203927/http://www.barcodeisland.com/codabar.phtml
    static ref CHARACTERS_MAP: HashMap<char, &'static str> = {
        let mut m = HashMap::new();
        m.insert('0', "0000011");
        m.insert('1', "0000110");
        m.insert('2', "0001001");
        m.insert('3', "1100000");
        m.insert('4', "0010010");
        m.insert('5', "1000010");
        m.insert('6', "0100001");
        m.insert('7', "0100100");
        m.insert('8', "0110000");
        m.insert('9', "1001000");
        m.insert('-', "0001100");
        m.insert('$', "0011000");
        m.insert(':', "1000101");
        m.insert('/', "1010001");
        m.insert('.', "1010100");
        m.insert('+', "0011111");
        m.insert('A', "0011010");
        m.insert('B', "0101001");
        m.insert('C', "0001011");
        m.insert('D', "0001110");
        m
    };
}

/// The error used for Coadabar generation
#[derive(Debug, Clone, Copy)]
pub enum CodabarError {
    InvalidMessage,
}

impl Display for CodabarError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CodabarError::InvalidMessage => write!(f, "the message is invalid"),
        }
    }
}
impl Error for CodabarError {}

pub struct Codabar<'a> {
    _data: &'a PhantomData<u8>,
}

impl<'a> Codabar<'a> {
    fn parse_message(message: &str) -> Result<Vec<&str>, CodabarError> {
        if message.contains(|c| !CHARACTERS_MAP.contains_key(&c)) {
            Err(CodabarError::InvalidMessage)
        } else {
            let bars: Vec<&str> = message
                .chars()
                .flat_map(|c| CHARACTERS_MAP.get(&c))
                .copied()
                .collect();

            Ok(bars)
        }
    }
}
impl<'a> Barcode for Codabar<'a> {
    type Error = CodabarError;

    fn make_descriptor(input: &str) -> Result<String, Self::Error> {
        let bars = Codabar::parse_message(input)?;

        let mut grouped = String::new();
        let size = bars.len();

        for (i, bar_group) in bars.into_iter().enumerate() {
            grouped.push_str(&bar_group);
            if i < size - 1 {
                grouped.push('0');
            }
        }
        Ok(grouped)
    }
}
