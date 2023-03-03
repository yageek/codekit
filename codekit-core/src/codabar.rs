use std::{collections::HashMap, error::Error, fmt::Display, marker::PhantomData};

use lazy_static::lazy_static;

use crate::{
    commons::{narrow_wide_gar_bar_u8, Barcode},
    Code,
};
lazy_static! {
    /// See: https://web.archive.org/web/20090122203927/http://www.barcodeisland.com/codabar.phtml
    static ref CHARACTERS_MAP: HashMap<char, u8> = {
        let mut m = HashMap::new();
        m.insert('0', 0b0000011);
        m.insert('1', 0b0000110);
        m.insert('2', 0b0001001);
        m.insert('3', 0b1100000);
        m.insert('4', 0b0010010);
        m.insert('5', 0b1000010);
        m.insert('6', 0b0100001);
        m.insert('7', 0b0100100);
        m.insert('8', 0b0110000);
        m.insert('9', 0b1001000);
        m.insert('-', 0b0001100);
        m.insert('$', 0b0011000);
        m.insert(':', 0b1000101);
        m.insert('/', 0b1010001);
        m.insert('.', 0b1010100);
        m.insert('+', 0b0011111);
        m.insert('A', 0b0011010);
        m.insert('B', 0b0101001);
        m.insert('C', 0b0001011);
        m.insert('D', 0b0001110);
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
    fn parse_message(message: &str) -> Result<Vec<u8>, CodabarError> {
        if message.contains(|c| !CHARACTERS_MAP.contains_key(&c)) {
            Err(CodabarError::InvalidMessage)
        } else {
            let bars: Vec<_> = message
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

    fn make_descriptor(input: &str) -> Result<crate::Code, Self::Error> {
        let patterns = Codabar::parse_message(input)?;

        let bars: Vec<_> = patterns
            .into_iter()
            .map(|pattern| narrow_wide_gar_bar_u8(pattern, 7))
            .collect();

        let mut grouped = vec![];
        let size = bars.len();

        for (i, mut bar_group) in bars.into_iter().enumerate() {
            grouped.append(&mut bar_group);
            if i < size - 1 {
                grouped.push(0);
            }
        }
        Ok(Code::new(grouped))
    }
}
