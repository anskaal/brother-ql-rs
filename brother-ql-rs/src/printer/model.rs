
#[derive(Debug)]
pub enum PrinterModel {
    QL500O550,
    QL560,
    QL570,
    QL580N,
    QL650T,
    QL700,
    QL1050,
    QL1060N,
    Unknown
}

impl PrinterModel {

    pub fn from_byte(byte: u8) -> PrinterModel {
        match byte {
            0x4F => PrinterModel::QL500O550,
            0x31 => PrinterModel::QL560,
            0x32 => PrinterModel::QL570,
            0x33 => PrinterModel::QL580N,
            0x51 => PrinterModel::QL650T,
            0x35 => PrinterModel::QL700,
            0x50 => PrinterModel::QL1050,
            0x34 => PrinterModel::QL1060N,
            _ => PrinterModel::Unknown
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            PrinterModel::QL500O550 => "QL-500/550",
            PrinterModel::QL560 => "QL-560",
            PrinterModel::QL570 => "QL-570",
            PrinterModel::QL580N => "QL-580N",
            PrinterModel::QL650T => "QL-650TD",
            PrinterModel::QL700 => "QL-700",
            PrinterModel::QL1050 => "QL-1050",
            PrinterModel::QL1060N => "QL-1060N",
            PrinterModel::Unknown => "Unknown"
        }
    }
}