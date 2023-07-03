use std::convert::TryInto;
use crate::printer::constants::RASTER_LINE_LENGTH;
use crate::printer::setting::Resolution;

pub struct PrintJob<'a> {
    pub cut_on_end: bool,
    pub bytes: &'a[u8],
    pub resolution: Resolution,
    pub mirrored: bool
}

impl <'a> PrintJob<'a> {
    pub(crate) fn get_raster_lines(&self) -> Vec<[u8; RASTER_LINE_LENGTH]> {
        let mut lines: Vec<[u8; RASTER_LINE_LENGTH]> = vec![];
        let chunks = self.bytes.chunks(RASTER_LINE_LENGTH);
        if self.mirrored {
            for (_, chunk) in chunks.enumerate() {
                let mut data: [u8; RASTER_LINE_LENGTH] = chunk.try_into().unwrap();
                data.reverse();
                let reverse = data.map(|byte|{ byte.reverse_bits() });
                let inverted = reverse.map(|px|{ !px });
                lines.push(inverted);
            }
        } else {
            for (_, chunk) in chunks.enumerate() {
                let data: [u8; RASTER_LINE_LENGTH] = chunk.try_into().unwrap();
                let inverted = data.map(|px|{ !px });
                lines.push(inverted);
            }
        }
        lines
    }
}