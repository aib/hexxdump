pub const SUBSTITUTE_CHAR: char = '.';

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
