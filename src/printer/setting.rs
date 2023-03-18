
pub enum PrinterSetting {
    SwitchToRasterMode,
    AutoCut(bool),
    HighResMode(bool),
    NormalResMode(bool),
}

pub enum Resolution {
    /// 600 dpi. Has a 2:1 height-to-with proportion.
    Normal,
    /// 300 dpi. Has a 1:1 height-to-with proportion.
    High,
}

impl PrinterSetting {
    pub fn get_byte_sequence(&self) -> [u8; 4] {
        match self {
            PrinterSetting::SwitchToRasterMode => [0x1B, 0x69, 0x61, 0x1],
            PrinterSetting::AutoCut(on) => [0x1B, 0x69, 0x4D, if *on { 0x0 } else { 0x40 }],
            PrinterSetting::NormalResMode(cut) => {
                let cut_bit = if *cut { 0 } else { 1 };
                [0x1B, 0x69, 0x4B, cut_bit << 3]
            }
            PrinterSetting::HighResMode(cut) => {
                let cut_bit = if *cut { 0 } else { 1 };
                [0x1B, 0x69, 0x4B, cut_bit << 3 | 1 << 6]
            }
        }
    }
}