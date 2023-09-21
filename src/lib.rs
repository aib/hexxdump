//! `hexxdump` is a configurable tool for generating hex dumps

mod util;
pub mod config;

/// A pre-made Hexxdump object with a default configuration ([`config::DEFAULT`])
pub const DEFAULT: Hexxdump = Hexxdump::with_config(config::DEFAULT);

/// A hex dumper object that contains a given configuration ([`config::Config`])
#[derive(Clone, Debug)]
pub struct Hexxdump {
	config: config::Config,
}

impl Hexxdump {
	/// Create a [`Hexxdump`] object using the given [`config::Config`] object
	pub const fn with_config(config: config::Config) -> Self {
		Self { config }
	}

	/// Writes the hex dump of `bytes` to [`stdout`](`std::io::Stdout`)
	///
	/// Equivalent to calling [`hexdump_to`]`(std::io::stdout(), bytes)`
	pub fn hexdump<B: AsRef<[u8]>>(&self, bytes: B) {
		self.hexdump_to(std::io::stdout(), bytes).ok();
	}

	/// Writes the hex dump of `bytes` to the given byte sink, `output`
	pub fn hexdump_to<W: std::io::Write, B: AsRef<[u8]>>(&self, mut output: W, bytes: B) -> std::io::Result<usize> {
		let mut written = 0;
		for (address, row) in self.get_rows(bytes.as_ref()) {
			if self.config.show_address {
				written += output.write(address.as_bytes())?;
			}
			written += output.write(row.as_bytes())?;
			written += output.write(&[b'\n'])?;
		}
		Ok(written)
	}

	/// Returns the hex dump of `bytes` as a `String`
	pub fn get_hexdump<B: AsRef<[u8]>>(&self, bytes: B) -> String {
		let mut dump = String::new();
		for (address, row) in self.get_rows(bytes.as_ref()) {
			if self.config.show_address {
				dump.push_str(&address);
			}
			dump.push_str(&row);
			dump.push('\n');
		}
		dump
	}

	/// Utility function to convert a given `byte` to its `char` representatio
	pub fn byte_to_char(&self, byte: u8) -> char {
		if byte.is_ascii_graphic() {
			char::from(byte)
		} else if byte == b' ' {
			' '
		} else {
			if byte < 32 && self.config.use_control_pictures {
				['␀', '␁', '␂', '␃', '␄', '␅', '␆', '␇', '␈', '␉', '␊', '␋', '␌', '␍', '␎', '␏',
				 '␐', '␑', '␒', '␓', '␔', '␕', '␖', '␗', '␘', '␙', '␚', '␛', '␜', '␝', '␞', '␟']
					 [byte as usize]
			} else if byte == 127 && self.config.use_control_pictures {
				'␡'
			} else {
				self.config.substitute_character
			}
		}
	}

	/// Utility function to convert the given `bytes` into a string of hex values
	///
	/// This would be the same output as the middle column of a single-row hex dump
	pub fn get_hex_values(&self, bytes: &[u8]) -> String {
		let mut vals = String::with_capacity(bytes.len() * 3);
		for b in bytes {
			vals.push_str(&format!("{:02x} ", b));
		}
		vals.pop();
		vals
	}

	/// Utility function to convert the given `bytes` into a string of characters
	///
	/// This would be the same output as the right column of a single-row hex dump
	pub fn get_characters(&self, bytes: &[u8]) -> String {
		let mut chars = String::with_capacity(bytes.len());
		for b in bytes {
			chars.push(self.byte_to_char(*b));
		}
		chars
	}

	fn get_row(&self, bytes: &[u8], bytes_per_row: usize) -> String {
		let mut row = String::new();

		if self.config.show_hex_values {
			row.push_str(&self.get_hex_values(bytes));
			row.push(' ');

			for _ in 0..bytes_per_row.saturating_sub(bytes.len()) {
				row.push(' ');
				row.push(' ');
				row.push(' ');
			}

			row.push(' ');
		}

		if self.config.show_characters {
			row.push_str(&self.get_characters(bytes));
		}

		row
	}

	fn get_rows<'a>(&'a self, bytes: &'a [u8]) -> impl Iterator<Item = (String, String)> + 'a {
		let bytes_per_row = self.config.bytes_per_row;
		let chunk_size = if bytes_per_row == 0 { usize::MAX } else { bytes_per_row };
		let address_width = util::min_hex_digits_for(bytes.len().saturating_sub(1));
		let even_address_width = ((address_width + 1) / 2) * 2;
		let address_width = even_address_width.max(self.config.address_width);

		let mut offset = 0;
		bytes.chunks(chunk_size).map(move |bs| {
			let address = format!("{:0width$x}: ", offset, width = address_width);
			let row = self.get_row(bs, bytes_per_row);
			offset += bytes_per_row;
			(address, row)
		})
	}
}

/// Writes the hex dump of `bytes` to stdout with a default configuration
///
/// Equivalent to calling [`DEFAULT`].[`hexdump`]`(bytes)`
///
/// [`hexdump`]: `Hexxdump::hexdump`
pub fn hexdump<B: AsRef<[u8]>>(bytes: B) {
	DEFAULT.hexdump(bytes)
}

/// Writes the hexdump of `bytes` to the given byte sink `output` with a default configuration
///
/// Equivalent to calling [`DEFAULT`].[`hexdump_to`]`(output, bytes)`
///
/// [`hexdump_to`]: `Hexxdump::hexdump_to`
pub fn hexdump_to<W: std::io::Write, B: AsRef<[u8]>>(output: W, bytes: B) -> std::io::Result<usize> {
	DEFAULT.hexdump_to(output, bytes)
}

/// Returns the hexdump of `bytes` as a `String`, with a default configuration
///
/// Equivalent to calling [`DEFAULT`].[`get_hexdump`]`(bytes)`
///
/// [`get_hexdump`]: `Hexxdump::get_hexdump`
pub fn get_hexdump<B: AsRef<[u8]>>(bytes: B) -> String {
	DEFAULT.get_hexdump(bytes)
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
		let dump_str = get_hexdump(&bs);

		let mut out_buf = std::io::BufWriter::new(Vec::new());
		let written = hexdump_to(&mut out_buf, &bs).unwrap();

		assert_eq!(dump_str.as_bytes(), out_buf.buffer());
		assert_eq!(dump_str.len(), written);
	}

	#[test]
	fn test_64k_hexdumps() {
		let hd8 = Hexxdump::with_config(config::DEFAULT.bytes_per_row(8));
		let dump_64k = hd8.get_hexdump(&get_rep_bytes(0x10000));
		let lines_64k: Vec<&str> = dump_64k.lines().collect();

		assert_eq!(lines_64k[0x0000], "0000: 00 01 02 03 04 05 06 07  ........");
		assert_eq!(lines_64k[0x0006], "0030: 30 31 32 33 34 35 36 37  01234567");
		assert_eq!(lines_64k[0x1fe6], "ff30: 30 31 32 33 34 35 36 37  01234567");
		assert_eq!(lines_64k[0x1fff], "fff8: f8 f9 fa fb fc fd fe ff  ........");

		let dump_64k1 = hd8.get_hexdump(&get_rep_bytes(0x10001));
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
		let dump_1m = get_hexdump(&get_rep_bytes(0x100000));
		let lines_1m: Vec<&str> = dump_1m.lines().collect();

		assert_eq!(lines_1m[0x0000], "000000: 00 01 02 03 04 05 06 07 08 09 0a 0b 0c 0d 0e 0f  ................");
		assert_eq!(lines_1m[0x0fff], "00fff0: f0 f1 f2 f3 f4 f5 f6 f7 f8 f9 fa fb fc fd fe ff  ................");
		assert_eq!(lines_1m[0x1000], "010000: 00 01 02 03 04 05 06 07 08 09 0a 0b 0c 0d 0e 0f  ................");
		assert_eq!(lines_1m[0xffff], "0ffff0: f0 f1 f2 f3 f4 f5 f6 f7 f8 f9 fa fb fc fd fe ff  ................");

		let hd32 = Hexxdump::with_config(config::DEFAULT.bytes_per_row(32));
		let dump_1m_32 = hd32.get_hexdump(&get_rep_bytes(0x100000));
		let lines_1m_32: Vec<&str> = dump_1m_32.lines().collect();

		assert_eq!(lines_1m_32[0x0000], "000000: 00 01 02 03 04 05 06 07 08 09 0a 0b 0c 0d 0e 0f 10 11 12 13 14 15 16 17 18 19 1a 1b 1c 1d 1e 1f  ................................");
		assert_eq!(lines_1m_32[0x0002], "000040: 40 41 42 43 44 45 46 47 48 49 4a 4b 4c 4d 4e 4f 50 51 52 53 54 55 56 57 58 59 5a 5b 5c 5d 5e 5f  @ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_");
		assert_eq!(lines_1m_32[0x07ff], "00ffe0: e0 e1 e2 e3 e4 e5 e6 e7 e8 e9 ea eb ec ed ee ef f0 f1 f2 f3 f4 f5 f6 f7 f8 f9 fa fb fc fd fe ff  ................................");
		assert_eq!(lines_1m_32[0x0800], "010000: 00 01 02 03 04 05 06 07 08 09 0a 0b 0c 0d 0e 0f 10 11 12 13 14 15 16 17 18 19 1a 1b 1c 1d 1e 1f  ................................");
		assert_eq!(lines_1m_32[0x7fff], "0fffe0: e0 e1 e2 e3 e4 e5 e6 e7 e8 e9 ea eb ec ed ee ef f0 f1 f2 f3 f4 f5 f6 f7 f8 f9 fa fb fc fd fe ff  ................................");
	}

	#[test]
	fn test_get_hexdump() {
		let get_hexdump = |bs: &[u8], bytes_per_row: usize| {
			let hd = Hexxdump::with_config(config::DEFAULT.bytes_per_row(bytes_per_row));
			hd.get_hexdump(bs)
		};

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
		assert_eq!(
			get_hexdump(b"123", 6),
			"0000: 31 32 33           123\n"
		);
	}
}
