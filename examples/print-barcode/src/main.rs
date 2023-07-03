#![feature(slice_take)]
extern crate brother_ql_rs;

use barcoders::sym::ean13::EAN13;
use brother_ql_rs::printer::constants::{MAX_PIXEL_WIDTH, RASTER_LINE_LENGTH};
use brother_ql_rs::printer::job::PrintJob;
use brother_ql_rs::printer::{printers, ThermalPrinter};
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

    let bg = 0x0;
    let fg = 0x1;

    let mut line = [bg; RASTER_LINE_LENGTH];

    for (i, &b) in encoded.iter().enumerate() {
        let c = if b == 0 { bg } else { fg };

        for p in 0..BAR_WIDTH {
            let x = offset + (i * BAR_WIDTH) + p;
            let index = x / 8;
            let sub_index = x % 8;
            let existing_value = line[index];
            let bit_value = c << (7 - sub_index);
            line[index] = existing_value | bit_value;
        }
    }

    let bytes = line.repeat(BAR_HEIGHT);

    let job = PrintJob {
        cut_on_end: true,
        bytes: &bytes,
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