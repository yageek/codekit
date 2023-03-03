mod commons;
mod ean;

/// An internal type for encoder
trait Barcode {
    type Input;
    type Error;

    /// Return the descriptor for the code
    fn make_descriptor(input: Self::Input, options: CodeOptions) -> Result<Code, Self::Error>;
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CodeOptions {
    pub quiet_space: f32,
    pub code_height: f32,
    pub border_width: f32,
}

impl Default for CodeOptions {
    fn default() -> Self {
        CodeOptions {
            quiet_space: 7.0,
            code_height: 50.0,
            border_width: 0.0,
        }
    }
}
#[repr(C)]
/// A generic code descriptor
/// to draw code bar
pub struct Code {
    options: CodeOptions,
    bars: Vec<u8>,
}

impl Code {
    fn new(bars: Vec<u8>, options: CodeOptions) -> Code {
        Code { options, bars }
    }
    /// Retrieve the border witdh of the code
    pub fn border_width(&self) -> f32 {
        self.options.border_width
    }

    pub fn height(&self) -> f32 {
        self.options.code_height
    }

    pub fn quiet_space(&self) -> f32 {
        self.options.quiet_space
    }

    pub fn bars(&self) -> &[u8] {
        &self.bars
    }
}
