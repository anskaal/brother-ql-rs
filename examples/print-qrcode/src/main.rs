use qrcode::{Color, QrCode};
use brother_ql_rs::printer::constants::{MAX_PIXEL_WIDTH, RASTER_LINE_LENGTH};
use brother_ql_rs::printer::job::PrintJob;
use brother_ql_rs::printer::{Printable, printers, ThermalPrinter};
use brother_ql_rs::printer::setting::Resolution;

const BLOCK_SIZE: usize = 8;

fn main() {
    let code = QrCode::new(b"01234567").unwrap();
    let data = code.to_colors();

    let offset = 0;
    let size = code.width();

    let mut lines: Vec<[u8; RASTER_LINE_LENGTH]> = vec![];
    for y in 0..size {
        let mut row = [true; MAX_PIXEL_WIDTH];
        for x in 0..size {
            for p in 0..BLOCK_SIZE {
                row[offset + x * BLOCK_SIZE + p] = data[y * size + x] == Color::Light;
            }
        }

        let line = row.into_raster_line();
        for _ in 0..BLOCK_SIZE {
            lines.push(line);
        }
    }

    let job = PrintJob {
        cut_on_end: true,
        raster_lines: lines,
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