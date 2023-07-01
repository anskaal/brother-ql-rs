
pub enum PrinterSetting {
    SwitchToRasterMode,
    AutoCut(bool),
    HighResMode(bool),
    NormalResMode(bool),
    PowerOnWhenConnected(bool),
    SleepTimer(SleepTimerValue),
    Mirror(bool)
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
            PrinterSetting::AutoCut(on) => [0x1B, 0x69, 0x4D, if *on { 0x0 } else { 0x40 }],
            PrinterSetting::NormalResMode(cut) => {
                let cut_bit = if *cut { 0 } else { 1 };
                [0x1B, 0x69, 0x4B, cut_bit << 3]
            }
            PrinterSetting::HighResMode(cut) => {
                let cut_bit = if *cut { 0 } else { 1 };
                [0x1B, 0x69, 0x4B, cut_bit << 3 | 1 << 6]
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
            PrinterSetting::Mirror(mirrored) => {
                [0x1B, 0x69, 0x4d, if *mirrored { 0x40 } else { 0xc0 } ]
            }
        }
    }
}