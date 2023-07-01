
pub static GET_STATUS: [u8; 3] = [0x1B, 0x69, 0x53];
pub static START_PRINT_LAST_PAGE: [u8; 1] = [0x1A];
pub static START_PRINT: [u8; 1] = [0x0C];

pub enum Command {
    GetStatus,
    StartPrint(bool),
}

impl Command {

    pub fn get_byte_sequence(&self) -> &[u8] {
        match self {
            Command::GetStatus => &GET_STATUS,
            Command::StartPrint(is_last_page) => if *is_last_page {
                &START_PRINT_LAST_PAGE
            } else {
                &START_PRINT
            }
        }
    }
}