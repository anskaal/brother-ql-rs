extern crate brother_ql_rs;

use std::fs::File;
use brother_ql_rs::printer::{printers, ThermalPrinter};
use brother_ql_rs::printer::constants::RASTER_LINE_LENGTH;
use brother_ql_rs::printer::job::PrintJob;
use brother_ql_rs::printer::setting::Resolution;

fn main() {
    let file = File::open("img.png").unwrap();
    let decoder = png::Decoder::new(file);
    let mut reader = decoder.read_info().unwrap();
    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf).unwrap();
    println!("Info: {:?}", info);
    let bytes = &buf[..info.buffer_size()];
    let bytes_total = bytes.len();
    println!("bytes: {}", bytes_total);

    let lines = bytes.chunks(RASTER_LINE_LENGTH)
        .map(|chunk| {
            let mut row = [255u8; RASTER_LINE_LENGTH];
            for (x, &byte) in chunk.iter().enumerate() {
                row[x] = 255u8 - byte
            }
            row
        })
        .collect();

    let job = PrintJob {
        cut_on_end: true,
        raster_lines: lines,
        resolution: Resolution::Normal,
        mirrored: true,
    };

    for printer in printers() {
        match ThermalPrinter::new(printer) {
            Ok(p) => {
                println!("Sending job to printer...");
                p.print(&job).unwrap()
            }
            Err(e) => panic!("Failed to init Thermal Printer: {:?}", e)
        };
    }
}