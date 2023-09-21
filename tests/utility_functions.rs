#[test]
fn get_hex_values() {
	let hexxdump = hexxdump::DEFAULT;

	assert_eq!("", hexxdump.get_hex_values(b""));
	assert_eq!("42", hexxdump.get_hex_values(b"\x42"));
	assert_eq!("48 65 6c 6c 6f", hexxdump.get_hex_values(b"Hello"));
	assert_eq!("00 48 65 6c 6c 6f", hexxdump.get_hex_values(b"\0Hello"));
	assert_eq!("48 65 6c 6c 6f 00", hexxdump.get_hex_values(b"Hello\0"));
	assert_eq!(
		 concat!("54 68 69 73 20 69 73 20 61 20 6c 6f 6e 67 69 73 68 20 6c 69 6e 65 2c 20 6f 76 65 72 20 61 6e ",
			 "79 20 64 65 66 61 75 6c 74 20 72 6f 77 20 6c 65 6e 67 74 68 20 66 6f 72 20 73 75 72 65 21"),
		hexxdump.get_hex_values(b"This is a longish line, over any default row length for sure!")
	);
}

#[test]
fn get_characters() {
	let hexxdump = hexxdump::DEFAULT;
	let hexxdump_with_cp = hexxdump::config::DEFAULT.use_control_pictures().into_hexxdump();

	assert_eq!("", hexxdump        .get_characters(b""));
	assert_eq!("", hexxdump_with_cp.get_characters(b""));

	assert_eq!("*", hexxdump        .get_characters(b"\x2a"));
	assert_eq!("*", hexxdump_with_cp.get_characters(b"\x2a"));

	assert_eq!("Hello", hexxdump        .get_characters(b"Hello"));
	assert_eq!("Hello", hexxdump_with_cp.get_characters(b"Hello"));

	assert_eq!(".Hello", hexxdump        .get_characters(b"\0Hello"));
	assert_eq!("␀Hello", hexxdump_with_cp.get_characters(b"\0Hello"));

	assert_eq!("Hello.", hexxdump        .get_characters(b"Hello\0"));
	assert_eq!("Hello␀", hexxdump_with_cp.get_characters(b"Hello\0"));

	assert_eq!("Not.print able.!...", hexxdump        .get_characters(b"Not\tprint able\n!\x7f\x80\xff"));
	assert_eq!("Not␉print able␊!␡..", hexxdump_with_cp.get_characters(b"Not\tprint able\n!\x7f\x80\xff"));
}
