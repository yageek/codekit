/// An internal type for encoder
pub(crate) trait Barcode {
    type Error: std::error::Error;

    /// Return the descriptor for the code
    fn make_descriptor(input: &str) -> Result<String, Self::Error>;
}
