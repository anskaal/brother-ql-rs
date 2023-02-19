
#[derive(Debug, PartialEq)]
pub enum StatusType {
    ReplyToStatusRequest,
    PrintingCompleted,
    ErrorOccurred,
    Notification,
    PhaseChange,
}

impl StatusType {

    pub fn from_byte(byte: u8) -> StatusType {
        match byte {
            0x00 => StatusType::ReplyToStatusRequest,
            0x01 => StatusType::PrintingCompleted,
            0x02 => StatusType::ErrorOccurred,
            0x05 => StatusType::Notification,
            0x06 => StatusType::PhaseChange,
            _ => StatusType::Notification // Will never occur
        }
    }
}