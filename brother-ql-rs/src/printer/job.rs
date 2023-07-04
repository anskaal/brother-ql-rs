use std::convert::TryInto;
use crate::printer::constants::RASTER_LINE_LENGTH;
use crate::printer::setting::Resolution;

pub struct PrintJob {
    pub cut_on_end: bool,
    pub raster_lines: Vec<[u8; RASTER_LINE_LENGTH]>,
    pub resolution: Resolution,
    pub mirrored: bool
}

impl PrintJob {
    pub(crate) fn get_raster_lines(&self) -> Vec<[u8; RASTER_LINE_LENGTH]> {
        self.raster_lines.iter()
            .map(|&chunk|{
                if self.mirrored {
                    let mut data: [u8; RASTER_LINE_LENGTH] = chunk.try_into().unwrap();
                    data.reverse();
                    data.map(|byte|{ byte.reverse_bits() })
                } else {
                    chunk
                }
            })
            .collect()
    }
}