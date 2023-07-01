//! Label media and USB ID constants used by Brother QL printers

#[derive(Debug, Copy, Clone)]
pub struct WidthLength(pub u32, pub u32);

#[derive(Debug, Copy, Clone)]
pub struct Label {
	pub tape_size: WidthLength,
	pub dots: WidthLength,
	pub dots_printable: WidthLength,
	pub right_margin: u8,
	pub feed_margin: u8,
}

/// Returns a corresponding label type given dimensions returned by the printer
///
/// These are predefined label rolls types sold by Brother and defined in the spec
pub fn label_data(width: u8, length: Option<u8>) -> Option<Label> {
	if let Some(length) = length {
		// Die cut label
		match (width, length) {
			(17, 54) => Some(Label {
				tape_size: WidthLength(17, 54),
				dots: WidthLength(201, 636),
				dots_printable: WidthLength(165, 566),
				right_margin: 0,
				feed_margin: 0,
			}),
			(17, 87) => Some(Label {
				tape_size: WidthLength(17, 87),
				dots: WidthLength(201, 1026),
				dots_printable: WidthLength(165, 956),
				right_margin: 0,
				feed_margin: 0,
			}),
			(23, 23) => Some(Label {
				tape_size: WidthLength(23, 23),
				dots: WidthLength(272, 272),
				dots_printable: WidthLength(202, 202),
				right_margin: 42,
				feed_margin: 0,
			}),
			(29, 42) => Some(Label {
				tape_size: WidthLength(29, 42),
				dots: WidthLength(342, 495),
				dots_printable: WidthLength(306, 425),
				right_margin: 6,
				feed_margin: 0,
			}),
			(29, 90) => Some(Label {
				tape_size: WidthLength(29, 90),
				dots: WidthLength(342, 1061),
				dots_printable: WidthLength(306, 991),
				right_margin: 6,
				feed_margin: 0,
			}),
			(39, 90) => Some(Label {
				tape_size: WidthLength(38, 90),
				dots: WidthLength(449, 1061),
				dots_printable: WidthLength(413, 991),
				right_margin: 12,
				feed_margin: 0,
			}),
			(39, 48) => Some(Label {
				tape_size: WidthLength(39, 48),
				dots: WidthLength(461, 565),
				dots_printable: WidthLength(425, 495),
				right_margin: 6,
				feed_margin: 0,
			}),
			(52, 29) => Some(Label {
				tape_size: WidthLength(52, 29),
				dots: WidthLength(614, 341),
				dots_printable: WidthLength(578, 271),
				right_margin: 0,
				feed_margin: 0,
			}),
			(62, 29) => Some(Label {
				tape_size: WidthLength(62, 29),
				dots: WidthLength(732, 341),
				dots_printable: WidthLength(696, 271),
				right_margin: 12,
				feed_margin: 0,
			}),
			(62, 100) => Some(Label {
				tape_size: WidthLength(62, 100),
				dots: WidthLength(732, 1179),
				dots_printable: WidthLength(696, 1109),
				right_margin: 12,
				feed_margin: 0,
			}),
			_ => None
		}
	}
	else {
		// Continuous label
		match width {
			12 => Some(Label {
				tape_size: WidthLength(12, 0),
				dots: WidthLength(142, 0),
				dots_printable: WidthLength(106, 0),
				right_margin: 29,
				feed_margin: 35
			}),
			29 => Some(Label {
				tape_size: WidthLength(29, 0),
				dots: WidthLength(342, 0),
				dots_printable: WidthLength(306, 0),
				right_margin: 6,
				feed_margin: 35
			}),
			38 => Some(Label {
				tape_size: WidthLength(38, 0),
				dots: WidthLength(449, 0),
				dots_printable: WidthLength(413, 0),
				right_margin: 12,
				feed_margin: 35
			}),
			50 => Some(Label {
				tape_size: WidthLength(50, 0),
				dots: WidthLength(590, 0),
				dots_printable: WidthLength(554, 0),
				right_margin: 12,
				feed_margin: 35
			}),
			54 => Some(Label {
				tape_size: WidthLength(54, 0),
				dots: WidthLength(636, 0),
				dots_printable: WidthLength(590, 0),
				right_margin: 0,
				feed_margin: 35
			}),
			62 => Some(Label {
				tape_size: WidthLength(62, 0),
				dots: WidthLength(732, 0),
				dots_printable: WidthLength(696, 0),
				right_margin: 12,
				feed_margin: 35
			}),
			102 => Some(Label {
				tape_size: WidthLength(102, 0),
				dots: WidthLength(1200, 0),
				dots_printable: WidthLength(1164, 0),
				right_margin: 12,
				feed_margin: 35
			}),
			_ => None
		}
	}
}

pub static BLACK_PIXEL: u8 = 0;
pub static WHITE_PIXEL: u8 = 1;

/// The static length of a line on all Brother QL printers
pub const RASTER_LINE_LENGTH: usize = 90;
pub const MAX_PIXEL_WIDTH: usize = RASTER_LINE_LENGTH * 8;

/// USB Vendor ID for Brother QL printers
pub const VENDOR_ID: u16 = 0x04F9;

/// Get the string representation of a printer's model name from a USB Product ID
pub fn printer_name_from_id(id: u16) -> Option<&'static str> {
	match id {
		0x2015 => Some("QL-500"),
		0x2016 => Some("QL-550"),
		0x2027 => Some("QL-560"),
		0x2028 => Some("QL-570"),
		0x2029 => Some("QL-580N"),
		0x201B => Some("QL-650TD"),
		0x2042 => Some("QL-700"),
		0x2020 => Some("QL-1050"),
		0x202A => Some("QL-1060N"),
		_ => None
	}
}
