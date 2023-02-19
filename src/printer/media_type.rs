
#[derive(Debug)]
pub enum MediaType {
    None,
    ContinuousTape,
    DieCutLabels,
}

impl MediaType {

    pub fn from_byte(byte: u8) -> MediaType {
        match byte {
            0x0A => MediaType::ContinuousTape,
            0x0B => MediaType::DieCutLabels,
            _    => MediaType::None,
        }
    }

    pub fn to_byte(&self) -> Option<u8> {
        match self {
            MediaType::ContinuousTape => Some(0x0A),
            MediaType::DieCutLabels => Some(0x0B),
            MediaType::None => None
        }
    }
}