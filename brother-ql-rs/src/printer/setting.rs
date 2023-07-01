
pub enum PrinterSetting {
    SwitchToRasterMode,
    MirrorOrCut(bool, bool),
    HighResMode(bool),
    NormalResMode(bool),
    PowerOnWhenConnected(bool),
    SleepTimer(SleepTimerValue),
}

pub enum SleepTimerValue {
    Disable,
    TurnOffAfter10Minutes,
    TurnOffAfter20Minutes,
    TurnOffAfter30Minutes,
    TurnOffAfter40Minutes,
    TurnOffAfter50Minutes,
    TurnOffAfter60Minutes,
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
            PrinterSetting::SwitchToRasterMode => [0x1B, 0x69, 0x61, 0x01],
            PrinterSetting::MirrorOrCut(mirror, auto_cut) => {
                let mirror_bit = if *mirror { 1 } else { 0 };
                let cut_bit = if *auto_cut { 1 } else { 0 };
                [0x1B, 0x69, 0x4d, mirror_bit << 7 | cut_bit << 6]
            },
            PrinterSetting::NormalResMode(cut) => {
                let cut_bit = if *cut { 1 } else { 0 };
                [0x1B, 0x69, 0x4B, cut_bit << 3]
            }
            PrinterSetting::HighResMode(cut) => {
                let cut_bit = if *cut { 1 } else { 0 };
                [0x1B, 0x69, 0x4b, cut_bit << 3 | 1 << 6]
            }
            PrinterSetting::PowerOnWhenConnected(on) => {
                [0x1B, 0x69, 0x70, if *on { 0x00 } else { 0x01 }]
            }
            PrinterSetting::SleepTimer(value) => {
                let last_byte = match value {
                    SleepTimerValue::Disable => 0x00,
                    SleepTimerValue::TurnOffAfter10Minutes => 0x01,
                    SleepTimerValue::TurnOffAfter20Minutes => 0x02,
                    SleepTimerValue::TurnOffAfter30Minutes => 0x03,
                    SleepTimerValue::TurnOffAfter40Minutes => 0x04,
                    SleepTimerValue::TurnOffAfter50Minutes => 0x05,
                    SleepTimerValue::TurnOffAfter60Minutes => 0x06,
                };
                [0x1B, 0x69, 0x70, last_byte]
            }
        }
    }
}