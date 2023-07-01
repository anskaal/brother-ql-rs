//! Everything to do with USB protocol for Brother QL printers
//!
//! Based on the published [Brother QL Series Command Reference](https://download.brother.com/welcome/docp000678/cv_qlseries_eng_raster_600.pdf)

use std::time::Duration;
use std::thread;
use crate::printer::command::Command;
use crate::printer::command::Command::{GetStatus, StartPrint};
use crate::printer::constants::RASTER_LINE_LENGTH;
use crate::printer::media_type::MediaType;
use crate::printer::job::PrintJob;
use crate::printer::setting::{PrinterSetting, Resolution};
use crate::printer::setting::PrinterSetting::{MirrorOrCut, HighResMode, NormalResMode, SwitchToRasterMode};
use crate::printer::model::PrinterModel;
use crate::printer::status_type::StatusType;

pub mod constants;
mod model;
mod status_type;
mod media_type;
pub mod setting;
pub mod job;
mod command;

error_chain! {
	foreign_links {
		USB(rusb::Error);
	}
}

#[allow(non_snake_case)]
pub mod status {
	//! A representation of the status message Brother QL printers use
	//!
	//! Includes:
	//! * Model name
	//! * Loaded media
	//! * Current operation
	//! * Any errors that have occurred
	use crate::printer::media_type::MediaType;
	use crate::printer::model::PrinterModel;
	use crate::printer::status_type::StatusType;
	use super::constants::*;


	#[derive(Debug)]
	pub struct Media {
		pub media_type: MediaType,
		pub width: u8, // unit: mm
		pub length: u8, // unit: mm
	}
	impl Media {
		pub fn to_label(&self) -> Label {
			let length = if self.length == 0 {
				None
			}
			else {
				Some(self.length)
			};
			label_data(self.width, length).expect("Printer reported invalid label dimensions")
		}
	}


	#[derive(Debug)]
	pub struct Response {
		pub model: PrinterModel,
		pub status_type: StatusType,
		pub errors: Vec<&'static str>,
		pub media: Media,
	}
}

fn printer_filter<T: rusb::UsbContext>(device: &rusb::Device<T>) -> bool {
	let descriptor = device.device_descriptor().unwrap();
	if descriptor.vendor_id() == constants::VENDOR_ID && descriptor.product_id() == 0x2049 {
		eprintln!("You must disable Editor Lite mode on your QL-700 before you can print with it");
	}
	descriptor.vendor_id() == constants::VENDOR_ID && constants::printer_name_from_id(descriptor.product_id()).is_some()
}

/// Get a vector of all attached and supported Brother QL printers as USB devices from which `ThermalPrinter` structs can be initialized.
pub fn printers() -> Vec<rusb::Device<rusb::GlobalContext>> {
	rusb::DeviceList::new()
		.unwrap()
		.iter()
		.filter(printer_filter)
		.collect()
}

/// The primary interface for dealing with Brother QL printers. Handles all USB communication with the printer.
pub struct ThermalPrinter<T: rusb::UsbContext> {
	pub manufacturer: String,
	pub model: String,
	pub serial_number: String,
	handle: rusb::DeviceHandle<T>,
	in_endpoint: u8,
	out_endpoint: u8,
}
impl<T: rusb::UsbContext> std::fmt::Debug for ThermalPrinter<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} ({})", self.manufacturer, self.model, self.serial_number)
    }
}
impl<T: rusb::UsbContext> ThermalPrinter<T> {
	/// Create a new `ThermalPrinter` instance using a `rusb` USB device handle.
	///
	/// Obtain list of connected device handles by calling `printers()`.
	pub fn new(device: rusb::Device<T>) -> Result<Self> {
		let mut handle = device.open()?;
		let mut in_endpoint: Option<u8> = None;
		let mut out_endpoint: Option<u8> = None;

		let config = device.active_config_descriptor()?;
		let interface = config.interfaces().next().chain_err(|| "Brother QL printers should have exactly one interface")?;
		let interface_descriptor = interface.descriptors().next().chain_err(|| "Brother QL printers should have exactly one interface descriptor")?;
		for endpoint in interface_descriptor.endpoint_descriptors() {
			if endpoint.transfer_type() != rusb::TransferType::Bulk {
				bail!("Brother QL printers are defined as using only bulk endpoint communication");
			}
			match endpoint.direction() {
				rusb::Direction::In  => in_endpoint  = Some(endpoint.address()),
				rusb::Direction::Out => out_endpoint = Some(endpoint.address()),
			}
		}
		if in_endpoint.is_none() || out_endpoint.is_none() {
			bail!("Input or output endpoint not found");
		}

		handle.claim_interface(interface.number())?;
		if let Ok(kd_active) = handle.kernel_driver_active(interface.number()) {
			if kd_active {
				handle.detach_kernel_driver(interface.number())?;
			}
		}

		let device_descriptor = device.device_descriptor()?;

		let printer = ThermalPrinter {
			manufacturer: handle.read_manufacturer_string_ascii(&device_descriptor)?,
			model: handle.read_product_string_ascii(&device_descriptor)?,
			serial_number: handle.read_serial_number_string_ascii(&device_descriptor)?,
			handle,
			in_endpoint: in_endpoint.unwrap(),
			out_endpoint: out_endpoint.unwrap(),
		};

		// Reset printer
		let clear_command = [0x00; 200];
		ThermalPrinter::write(&printer, &clear_command)?;
		let initialize_command = [0x1B, 0x40];
		ThermalPrinter::write(&printer, &initialize_command)?;

		ThermalPrinter::get_status(&printer)?;
		Ok(printer)
	}

	/// Sends raster lines to the USB printer, begins printing, and immediately returns
	///
	/// Images on the label tape are comprised of bits representing either black (`1`) or white (`0`). They are
	/// arranged in lines of a static width that corresponds to the width of the printer's thermal print head.
	///
	/// **Note:** the raster line width does not change for label media of different sizes. This means the
	/// printer can print out-of-bounds and even print on parts of the label not originally intended to
	/// contain content. Your rasterizer will have to figure out, given a media type, which parts of the
	/// image will appear on the media and resize or shift margins and content accordingly.
	pub fn print(&self, job: &PrintJob) -> Result<status::Response> {
		let raster_lines: &Vec<[u8; RASTER_LINE_LENGTH as usize]> = &job.lines;

		self.apply_setting(SwitchToRasterMode)?;
		self.print_info(raster_lines)?;

		self.apply_setting(MirrorOrCut(job.mirrored, job.cut_on_end))?;
		let resolution = match &job.resolution {
			Resolution::Normal => NormalResMode(job.cut_on_end),
			Resolution::High => HighResMode(job.cut_on_end)
		};
		self.apply_setting(resolution)?;

		let label = self.current_label()?;

		let margins_command = [0x1B, 0x69, 0x64, label.feed_margin, 0x00];
		self.write(&margins_command)?;

		for line in raster_lines.iter() {
			let mut raster_command = vec![0x67, 0x00, RASTER_LINE_LENGTH as u8];
			raster_command.extend_from_slice(line);
			self.write(&raster_command)?;
		}

		self.send_command(StartPrint(true))?;
		self.read()
	}

	/// Print information command
	/// Flags:
	/// 	PI_KIND		0x02	Paper type
	/// 	PI_WIDTH	0x04	Paper width
	/// 	PI_LENGTH	0x08	Paper length
	/// 	PI_QUALITY	0x40	Give priority to print quality
	/// 	PI_RECOVER	0x80	Always ON
	fn print_info(&self, raster_lines: &Vec<[u8; RASTER_LINE_LENGTH]>) -> Result<()> {
		let status = self.get_status()?;
		const VALID_FLAGS: u8 = 0x80 | 0x02 | 0x04 | 0x08 | 0x40; // Everything enabled
		let media_type: u8 = match status.media.media_type.to_byte() {
			Some(value) => value,
			None => return Err("No media loaded into printer".into())
		};

		let starting_page = 0x01; // Starting page: 0; Other pages: 1.
		let mut media_command = [0x1B, 0x69, 0x7A, VALID_FLAGS, media_type, status.media.width, status.media.length, 0, 0, 0, 0, starting_page, 0];
		let line_count = (raster_lines.len() as u32).to_le_bytes();
		media_command[7..7 + 4].copy_from_slice(&line_count);
		self.write(&media_command)
	}

	/// Same as `print()` but will not return until the printer reports that it has finished printing.
	pub fn print_blocking(&self, job: &PrintJob) -> Result<()> {
		self.print(job)?;
		loop {
			match self.read() {
				Ok(ref response) if response.status_type == StatusType::PrintingCompleted => break,
				_ => thread::sleep(Duration::from_millis(50)),
			}
		}
		Ok(())
	}

	/// Get the currently loaded label size.
	pub fn current_label(&self) -> Result<constants::Label> {
		let media = self.get_status()?.media;
		constants::label_data(media.width, match media.length {
			0 => None,
			_ => Some(media.length)
		}).ok_or("Unknown media loaded in printer".into())
	}

	/// Get the current status of the printer including possible errors, media type, and model name.
	pub fn get_status(&self) -> Result<status::Response> {
		self.send_command(GetStatus)?;
		self.read()
	}

	fn read(&self) -> Result<status::Response> {
		let response = match self.read_bulk() {
			Ok(bytes) => bytes,
			Err(error) => return Err(error)
		};

		let model = PrinterModel::from_byte(response[4]);

		let mut errors = Vec::new();

		fn error_if(byte: u8, flag: u8, message: &'static str, errors: &mut Vec<&'static str>) {
			if byte & flag != 0 {
				errors.push(message);
			}
		}
		error_if(response[8], 0x01, "No media when printing", &mut errors);
		error_if(response[8], 0x02, "End of media", &mut errors);
		error_if(response[8], 0x04, "Tape cutter jam", &mut errors);
		error_if(response[8], 0x10, "Main unit in use", &mut errors);
		error_if(response[8], 0x80, "Fan doesn't work", &mut errors);
		error_if(response[9], 0x04, "Transmission error", &mut errors);
		error_if(response[9], 0x10, "Cover open", &mut errors);
		error_if(response[9], 0x40, "Cannot feed", &mut errors);
		error_if(response[9], 0x80, "System error", &mut errors);

		let width = response[10];
		let length = response[17];

		let media_type = MediaType::from_byte(response[11]);
		let status_type = StatusType::from_byte(response[18]);

		Ok(status::Response {
			model,
			status_type,
			errors,
			media: status::Media {
				media_type,
				width,
				length,
			}
		})
	}

	fn read_bulk(&self) -> Result<[u8; 32]> {
		const RECEIVE_SIZE: usize = 32;
		let mut response = [0; RECEIVE_SIZE];
		let bytes_read = self.handle.read_bulk(
			self.in_endpoint,
			&mut response,
			Duration::from_millis(500)
		)?;

		if bytes_read != RECEIVE_SIZE || response[0] != 0x80 {
			return Err("Invalid response received from printer".into());
		}
		return Ok(response)
	}

	fn apply_setting(&self, setting: PrinterSetting) -> Result<()> {
		let sequence = setting.get_byte_sequence();
		println!("Setting: {:x?}", sequence);
		self.write(&sequence)
	}

	fn send_command(&self, command: Command) -> Result<()> {
		let sequence = command.get_byte_sequence();
		println!("Command: {:x?}", sequence);
		self.write(sequence)
	}

	fn write(&self, data: &[u8]) -> Result<()> {
		self.handle.write_bulk(self.out_endpoint, data, Duration::from_millis(500))?;
		Ok(())
	}
}