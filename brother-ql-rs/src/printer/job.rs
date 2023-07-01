use crate::printer::constants::RASTER_LINE_LENGTH;
use crate::printer::setting::Resolution;

pub struct PrintJob {
    pub cut_on_end: bool,
    pub lines: Vec<[u8; RASTER_LINE_LENGTH as usize]>,
    pub resolution: Resolution,
    pub mirrored: bool
}