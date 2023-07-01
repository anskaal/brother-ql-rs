

extern crate brother_ql_rs;

use std::fs::File;
use brother_ql_rs::printer::{printers, ThermalPrinter};
use brother_ql_rs::printer::job::PrintJob;
use brother_ql_rs::printer::setting::Resolution;

/// The static length of a line on all Brother QL printers
pub const RASTER_LINE_LENGTH: usize = 90;
pub const MAX_PIXEL_WIDTH: usize = RASTER_LINE_LENGTH * 8;

fn main() {
    let file = File::open(format!("img.png")).unwrap();

    let decoder = png::Decoder::new(file);
    let mut reader = decoder.read_info().unwrap();
    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf).unwrap();
    println!("Info: {:?}", info);
    let bytes = &buf[..info.buffer_size()];
    let bytes_total = bytes.len();
    println!("bytes: {}", bytes_total);

    let mut lines: Vec<[u8; RASTER_LINE_LENGTH]> = vec![];
    let chunks = bytes.chunks(RASTER_LINE_LENGTH);
    for (_, chunk) in chunks.enumerate() {
        let mut data: [u8; RASTER_LINE_LENGTH] = chunk.try_into().unwrap();
        data.reverse();
        let reverse = data.map(|byte|{ byte.reverse_bits() });
        let inverted = reverse.map(|px|{ !px });
        lines.push(inverted);
    }

    let job = PrintJob {
        cut_on_end: true,
        lines,
        resolution: Resolution::Normal,
        mirrored: true
    };

    for printer in printers() {
        match ThermalPrinter::new(printer) {
            Ok(p) => {
                println!("Sending job to printer...");
                p.print(&job).unwrap()
            },
            Err(e) => panic!("Failed to init Thermal Printer: {:?}", e)
        };
    }
}