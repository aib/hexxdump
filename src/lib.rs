pub const SUBSTITUTE_CHAR: char = '.';
pub const MIN_ADDRESS_WIDTH: usize = 4;

pub fn get_hexdump(bytes: &[u8], bytes_per_row: usize) -> String {
	let chunk_size = if bytes_per_row == 0 { usize::MAX } else { bytes_per_row };
	let address_width = MIN_ADDRESS_WIDTH;

	let mut dump = String::new();
	let mut offset = 0usize;
	for row in bytes.chunks(chunk_size) {
		let address = format!("{:0width$x}: ", offset, width = address_width);
		dump.push_str(&address);
		dump.push_str(&get_hexdump_row(row, bytes_per_row));
		dump.push('\n');
		offset += chunk_size;
	}
	dump
}

pub fn get_hexdump_row(bytes: &[u8], bytes_per_row: usize) -> String {
	let (hex, view) = get_hexdump_tuple(bytes);

	let padding_width = bytes_per_row.saturating_sub(bytes.len()) * 3;

	let mut row = String::with_capacity(hex.len() + padding_width + view.len());
	row.push_str(&hex);
	row.push_str(&" ".repeat(padding_width));
	row.push_str("  ");
	row.push_str(&view);
	row
}

pub fn get_hexdump_tuple(bytes: &[u8]) -> (String, String) {
	let (mut hex, view) = bytes.iter().map(|b| {
		(format!("{:02x} ", b), u8_to_display_char(*b))
	}).unzip::<String, char, String, String>();

	hex.pop();

	(hex, view)
}

pub fn u8_to_display_char(u: u8) -> char {
	if u.is_ascii_graphic() {
		char::from(u)
	} else if u == 32 {
		' '
	} else {
		SUBSTITUTE_CHAR
	}
}

pub fn min_hex_digits_for(num: usize) -> usize {
	let mut digits = 1;
	let mut max = 16;

	while max < usize::MAX {
		if num < max {
			break;
		}
		max = max.saturating_mul(16);
		digits += 1;
	}
	digits
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_get_hexdump() {
		assert_eq!(
			get_hexdump(b"abcdefg\nABCDEFG\0", 0),
			"0000: 61 62 63 64 65 66 67 0a 41 42 43 44 45 46 47 00  abcdefg.ABCDEFG.\n"
		);
		assert_eq!(
			get_hexdump(b"abcdefg\nABCDEFG\0", 16),
			"0000: 61 62 63 64 65 66 67 0a 41 42 43 44 45 46 47 00  abcdefg.ABCDEFG.\n"
		);
		assert_eq!(
			get_hexdump(b"abcdefg\nABCDEFG\0", 17),
			"0000: 61 62 63 64 65 66 67 0a 41 42 43 44 45 46 47 00     abcdefg.ABCDEFG.\n"
		);
		assert_eq!(
			get_hexdump(b"abcdefg\nABCDEFG\0", 8),
			concat!(
				"0000: 61 62 63 64 65 66 67 0a  abcdefg.\n",
				"0008: 41 42 43 44 45 46 47 00  ABCDEFG.\n",
			)
		);
		assert_eq!(
			get_hexdump(b"abcdefg\nABCDEFG\0!", 8),
			concat!(
				"0000: 61 62 63 64 65 66 67 0a  abcdefg.\n",
				"0008: 41 42 43 44 45 46 47 00  ABCDEFG.\n",
				"0010: 21                       !\n",
			)
		);
		assert_eq!(
			get_hexdump(b"123", 0),
			"0000: 31 32 33  123\n"
		);
		assert_eq!(
			get_hexdump(b"1", 1),
			"0000: 31  1\n"
		);
		assert_eq!(
			get_hexdump(b"1", 2),
			"0000: 31     1\n"
		);
		assert_eq!(
			get_hexdump(b"12", 2),
			"0000: 31 32  12\n"
		);
		assert_eq!(
			get_hexdump(b"12", 1),
			concat!(
				"0000: 31  1\n",
				"0001: 32  2\n",
			)
		);
	}

	#[test]
	fn test_get_hexdump_row() {
		assert_eq!(
			get_hexdump_row(b"123", 0),
			"31 32 33  123"
		);
		assert_eq!(
			get_hexdump_row(b"123", 1),
			"31 32 33  123"
		);
		assert_eq!(
			get_hexdump_row(b"123", 2),
			"31 32 33  123"
		);
		assert_eq!(
			get_hexdump_row(b"123", 3),
			"31 32 33  123"
		);
		assert_eq!(
			get_hexdump_row(b"123", 4),
			"31 32 33     123"
		);
		assert_eq!(
			get_hexdump_row(b"123", 5),
			"31 32 33        123"
		);
		assert_eq!(
			get_hexdump_row(b" .\x00\x01\x02\x03", 9),
			"20 2e 00 01 02 03            ....."
		);
	}
	#[test]
	fn test_get_hexdump_tuple() {
		assert_eq!(
			get_hexdump_tuple(b"Hello, World!"),
			(
				String::from("48 65 6c 6c 6f 2c 20 57 6f 72 6c 64 21"),
				String::from("Hello, World!")
			)
		);

		assert_eq!(
			get_hexdump_tuple(b"123\nstr\0with \t n...a\x5c\x11\xff\x7f"),
			(
				String::from("31 32 33 0a 73 74 72 00 77 69 74 68 20 09 20 6e 2e 2e 2e 61 5c 11 ff 7f"),
				String::from("123.str.with . n...a\\...")
			)
		);

	}

	#[test]
	fn test_u8_to_display_char() {
		assert_eq!(u8_to_display_char(0x00), SUBSTITUTE_CHAR);
		assert_eq!(u8_to_display_char(0x01), SUBSTITUTE_CHAR);
		assert_eq!(u8_to_display_char(0x09), SUBSTITUTE_CHAR);
		assert_eq!(u8_to_display_char(0x0a), SUBSTITUTE_CHAR);
		assert_eq!(u8_to_display_char(0x1f), SUBSTITUTE_CHAR);
		assert_eq!(u8_to_display_char(0x20), ' ');
		assert_eq!(u8_to_display_char(0x21), '!');
		assert_eq!(u8_to_display_char(0x22), '"');
		assert_eq!(u8_to_display_char(0x27), '\'');
		assert_eq!(u8_to_display_char(0x3f), '?');
		assert_eq!(u8_to_display_char(0x40), '@');
		assert_eq!(u8_to_display_char(0x41), 'A');
		assert_eq!(u8_to_display_char(0x5a), 'Z');
		assert_eq!(u8_to_display_char(0x5c), '\\');
		assert_eq!(u8_to_display_char(0x61), 'a');
		assert_eq!(u8_to_display_char(0x7a), 'z');
		assert_eq!(u8_to_display_char(0x7e), '~');
		assert_eq!(u8_to_display_char(0x7f), SUBSTITUTE_CHAR);
		assert_eq!(u8_to_display_char(0x80), SUBSTITUTE_CHAR);
		assert_eq!(u8_to_display_char(0x81), SUBSTITUTE_CHAR);
		assert_eq!(u8_to_display_char(0xff), SUBSTITUTE_CHAR);
	}

	#[test]
	fn test_min_hex_digits_for() {
		assert_eq!(min_hex_digits_for(0x0), 1);
		assert_eq!(min_hex_digits_for(0x1), 1);
		assert_eq!(min_hex_digits_for(0xf), 1);
		assert_eq!(min_hex_digits_for(0x10), 2);
		assert_eq!(min_hex_digits_for(0xff), 2);
		assert_eq!(min_hex_digits_for(0x100), 3);
		assert_eq!(min_hex_digits_for(0xfff), 3);
		assert_eq!(min_hex_digits_for(0x1000), 4);
		assert_eq!(min_hex_digits_for(0xffff), 4);
		assert_eq!(min_hex_digits_for(usize::MAX), (usize::BITS / 4) as usize);
	}
}
