extern crate brother_ql_rs;

use barcoders::sym::ean13::EAN13;
use brother_ql_rs::printer::constants::MAX_PIXEL_WIDTH;
use brother_ql_rs::printer::job::PrintJob;
use brother_ql_rs::printer::setting::Resolution;
use brother_ql_rs::printer::{printers, Printable, ThermalPrinter};

const BAR_HEIGHT: usize = 80;
const BAR_WIDTH: usize = 4;

fn main() {
    let barcode = EAN13::new("7035620025037").unwrap();
    let encoded: Vec<u8> = barcode.encode();

    if BAR_WIDTH < 1 {
        panic!("BAR_WIDTH must be greater than 0.")
    }

    let barcode_bitmap_length = encoded.len();
    if barcode_bitmap_length < 1 {
        panic!("Failed to encode barcode.")
    }

    let offset = (MAX_PIXEL_WIDTH - encoded.len() * BAR_WIDTH) / 2;

    let mut row = [true; MAX_PIXEL_WIDTH];
    for (i, &b) in encoded.iter().enumerate() {
        for p in 0..BAR_WIDTH {
            let x = offset + (i * BAR_WIDTH) + p;
            row[x] = b == 0;
        }
    }

    let line = [row.into_raster_line()];
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
            Err(e) => panic!("Failed to init Thermal Printer: {:?}", e),
        };
    }
}
