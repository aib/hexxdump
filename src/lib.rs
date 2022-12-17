pub const SUBSTITUTE_CHAR: char = '.';

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

#[cfg(test)]
mod tests {
	use super::*;

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
}
