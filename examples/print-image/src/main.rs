

extern crate brother_ql_rs;

use std::fs::File;
use brother_ql_rs::printer::{printers, ThermalPrinter};
use brother_ql_rs::printer::job::PrintJob;
use brother_ql_rs::printer::setting::Resolution;

fn main() {

    let mut image = ImageBuffer::<Rgb<u8>>::new(WIDTH, HEIGHT);

    // set a central pixel to white
    image.get_pixel_mut(5, 5).data = [255, 255, 255];

    // write it out to a file
    image.save("output.png").unwrap();

    let file = File::open(format!("output.png")).unwrap();

    let decoder = png::Decoder::new(file);
    let mut reader = decoder.read_info().unwrap();
    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf).unwrap();
    println!("Info: {:?}", info);
    let bytes = &buf[..info.buffer_size()];
    let bytes_total = bytes.len();
    println!("bytes: {}", bytes_total);

    let job = PrintJob {
        cut_on_end: true,
        bytes,
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