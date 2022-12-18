const MIN_ADDRESS_WIDTH: usize = 4;

pub fn hexdump<B: AsRef<[u8]>>(bytes: B) {
	hexdump_to(std::io::stdout(), bytes, 16).ok();
}

pub fn hexdump_with_width<B: AsRef<[u8]>>(bytes: B, bytes_per_row: usize) {
	hexdump_to(std::io::stdout(), bytes, bytes_per_row).ok();
}

pub fn hexdump_to<W: std::io::Write, B: AsRef<[u8]>>(mut output: W, bytes: B, bytes_per_row: usize) -> std::io::Result<usize> {
	let mut written = 0;
	for (address, row) in get_rows(bytes.as_ref(), bytes_per_row) {
		written += output.write(address.as_bytes())?;
		written += output.write(row.as_bytes())?;
		written += output.write(&[10])?;
	}
	Ok(written)
}

pub fn get_hexdump<B: AsRef<[u8]>>(bytes: B, bytes_per_row: usize) -> String {
	let mut dump = String::new();
	for (address, row) in get_rows(bytes.as_ref(), bytes_per_row) {
		dump.push_str(&address);
		dump.push_str(&row);
		dump.push('\n');
	}
	dump
}

pub fn get_hexdump_row(bytes: &[u8], bytes_per_row: usize) -> String {
	let mut row = String::new();

	for b in bytes {
		row.push_str(&format!("{:02x} ", b));
	}

	for _ in 0..bytes_per_row.saturating_sub(bytes.len()) {
		row.push(' ');
		row.push(' ');
		row.push(' ');
	}

	row.push(' ');

	for b in bytes {
		row.push(u8_to_display_char(*b));
	}

	row
}

fn get_rows<'a>(bytes: &'a [u8], bytes_per_row: usize) -> impl Iterator<Item = (String, String)> + 'a {
	let chunk_size = if bytes_per_row == 0 { usize::MAX } else { bytes_per_row };
	let min_address_width = min_hex_digits_for(bytes.len().saturating_sub(1));
	let even_address_width = ((min_address_width + 1) / 2) * 2;
	let address_width = even_address_width.max(MIN_ADDRESS_WIDTH);

	let mut offset = 0;
	bytes.chunks(chunk_size).map(move |bs| {
		let address = format!("{:0width$x}: ", offset, width = address_width);
		let row = get_hexdump_row(bs, bytes_per_row);
		offset += bytes_per_row;
		(address, row)
	})
}

fn u8_to_display_char(u: u8) -> char {
	if u.is_ascii_graphic() {
		char::from(u)
	} else if u == 32 {
		' '
	} else {
		'.'
	}
}

fn min_hex_digits_for(num: usize) -> usize {
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

	fn get_rep_bytes(n: usize) -> Vec<u8> {
		std::iter::repeat((0..=255).collect::<Vec<u8>>()).flatten().take(n).collect()
	}

	#[test]
	fn test_hexdump_to_buffer() {
		let bs = get_rep_bytes(0x200);
		let dump_str = get_hexdump(&bs, 20);

		let mut out_buf = std::io::BufWriter::new(Vec::new());
		let written = hexdump_to(&mut out_buf, &bs, 20).unwrap();

		assert_eq!(dump_str.as_bytes(), out_buf.buffer());
		assert_eq!(dump_str.len(), written);
	}

	#[test]
	fn test_64k_hexdumps() {
		let dump_64k = get_hexdump(&get_rep_bytes(0x10000), 8);
		let lines_64k: Vec<&str> = dump_64k.lines().collect();

		assert_eq!(lines_64k[0x0000], "0000: 00 01 02 03 04 05 06 07  ........");
		assert_eq!(lines_64k[0x0006], "0030: 30 31 32 33 34 35 36 37  01234567");
		assert_eq!(lines_64k[0x1fe6], "ff30: 30 31 32 33 34 35 36 37  01234567");
		assert_eq!(lines_64k[0x1fff], "fff8: f8 f9 fa fb fc fd fe ff  ........");

		let dump_64k1 = get_hexdump(&get_rep_bytes(0x10001), 8);
		let lines_64k1: Vec<&str> = dump_64k1.lines().collect();

		assert_eq!(lines_64k1[0x0000], "000000: 00 01 02 03 04 05 06 07  ........");
		assert_eq!(lines_64k1[0x0006], "000030: 30 31 32 33 34 35 36 37  01234567");
		assert_eq!(lines_64k1[0x01ff], "000ff8: f8 f9 fa fb fc fd fe ff  ........");
		assert_eq!(lines_64k1[0x0200], "001000: 00 01 02 03 04 05 06 07  ........");
		assert_eq!(lines_64k1[0x1fe6], "00ff30: 30 31 32 33 34 35 36 37  01234567");
		assert_eq!(lines_64k1[0x1fff], "00fff8: f8 f9 fa fb fc fd fe ff  ........");
		assert_eq!(lines_64k1[0x2000], "010000: 00                       .");
	}

	#[test]
	fn test_1m_hexdumps() {
		let dump_1m = get_hexdump(&get_rep_bytes(0x100000), 16);
		let lines_1m: Vec<&str> = dump_1m.lines().collect();

		assert_eq!(lines_1m[0x0000], "000000: 00 01 02 03 04 05 06 07 08 09 0a 0b 0c 0d 0e 0f  ................");
		assert_eq!(lines_1m[0x0fff], "00fff0: f0 f1 f2 f3 f4 f5 f6 f7 f8 f9 fa fb fc fd fe ff  ................");
		assert_eq!(lines_1m[0x1000], "010000: 00 01 02 03 04 05 06 07 08 09 0a 0b 0c 0d 0e 0f  ................");
		assert_eq!(lines_1m[0xffff], "0ffff0: f0 f1 f2 f3 f4 f5 f6 f7 f8 f9 fa fb fc fd fe ff  ................");

		let dump_1m_32 = get_hexdump(&get_rep_bytes(0x100000), 32);
		let lines_1m_32: Vec<&str> = dump_1m_32.lines().collect();

		assert_eq!(lines_1m_32[0x0000], "000000: 00 01 02 03 04 05 06 07 08 09 0a 0b 0c 0d 0e 0f 10 11 12 13 14 15 16 17 18 19 1a 1b 1c 1d 1e 1f  ................................");
		assert_eq!(lines_1m_32[0x0002], "000040: 40 41 42 43 44 45 46 47 48 49 4a 4b 4c 4d 4e 4f 50 51 52 53 54 55 56 57 58 59 5a 5b 5c 5d 5e 5f  @ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_");
		assert_eq!(lines_1m_32[0x07ff], "00ffe0: e0 e1 e2 e3 e4 e5 e6 e7 e8 e9 ea eb ec ed ee ef f0 f1 f2 f3 f4 f5 f6 f7 f8 f9 fa fb fc fd fe ff  ................................");
		assert_eq!(lines_1m_32[0x0800], "010000: 00 01 02 03 04 05 06 07 08 09 0a 0b 0c 0d 0e 0f 10 11 12 13 14 15 16 17 18 19 1a 1b 1c 1d 1e 1f  ................................");
		assert_eq!(lines_1m_32[0x7fff], "0fffe0: e0 e1 e2 e3 e4 e5 e6 e7 e8 e9 ea eb ec ed ee ef f0 f1 f2 f3 f4 f5 f6 f7 f8 f9 fa fb fc fd fe ff  ................................");
	}

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
