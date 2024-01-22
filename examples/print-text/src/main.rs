
use noto_sans_mono_bitmap::{get_raster, get_raster_width, FontWeight, RasterHeight};
use brother_ql_rs::printer::job::PrintJob;
use brother_ql_rs::printer::{Printable, printers, ThermalPrinter};
use brother_ql_rs::printer::constants::MAX_PIXEL_WIDTH;
use brother_ql_rs::printer::setting::Resolution;

fn main() {
    let raster_height = RasterHeight::Size16;
    let font_weight = FontWeight::Regular;

    let width = get_raster_width(font_weight, raster_height);
    println!(
        "Each char of the mono-spaced font will be {}px in width if the font \
         weight={:?} and the bitmap height={}",
        width,
        font_weight,
        raster_height.val()
    );
    let char_raster = get_raster('A', font_weight, raster_height)
            .expect("unsupported char");
    println!("{:?}", char_raster);

    let mut lines = vec![];
    for &row in char_raster.raster().iter() {
        let mut line = [true; MAX_PIXEL_WIDTH];

        for (col_i, &pixel) in row.iter().enumerate() {
            if col_i < MAX_PIXEL_WIDTH {
                line[col_i] = pixel < 128
            }
        }

        lines.push(line.into_raster_line())
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