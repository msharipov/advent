#[derive(Debug, PartialEq)]
pub struct Screen {
    pixels: [[bool; 50]; 6],
}

impl Default for Screen {
    fn default() -> Self {
        Self {
            pixels: [[false; 50]; 6]
        }
    }
}
