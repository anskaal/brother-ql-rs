#![feature(slice_take)]
extern crate brother_ql_rs;

use barcoders::sym::ean13::EAN13;
use brother_ql_rs::printer::constants::{MAX_PIXEL_WIDTH, RASTER_LINE_LENGTH};
use brother_ql_rs::printer::job::PrintJob;
use brother_ql_rs::printer::{Printable, printers, ThermalPrinter};
use brother_ql_rs::printer::setting::Resolution;

const BAR_HEIGHT: usize = 80;
const BAR_WIDTH: usize = 4;

fn main() {
    let barcode = EAN13::new("7035620025037").unwrap();
    let encoded: Vec<u8> = barcode.encode();

    let offset = (MAX_PIXEL_WIDTH - encoded.len() * BAR_WIDTH) / 2;
    if offset < 0 {
        panic!("The barcode exceeds the maximum image width.")
    }

    let mut row = [true; MAX_PIXEL_WIDTH];
    for (i, &b) in encoded.iter().enumerate() {
        for p in 0..BAR_WIDTH {
            let x = offset + (i * BAR_WIDTH) + p;
            row[x] = b == 0;
        }
    }

    let line = vec![row.into_raster_line()];
    let job = PrintJob {
        cut_on_end: true,
        raster_lines: line.repeat(BAR_HEIGHT),
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