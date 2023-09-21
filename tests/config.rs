use hexxdump::config::Config;

#[test]
fn test_bytes_per_row() {
	{
		let bpr = Config::new().address_width(2).bytes_per_row(1).into_hexxdump();
		assert_eq!(
			concat!("00: 31  1\n",
					"01: 32  2\n",
					"02: 33  3\n"),
			bpr.get_hexdump(b"123"),
		);
	}

	{
		let bpr = Config::new().address_width(2).bytes_per_row(2).into_hexxdump();
		assert_eq!(
			concat!("00: 31 32  12\n",
					"02: 33 34  34\n",
					"04: 35     5\n"),
			bpr.get_hexdump(b"12345"),
		);
	}

	{
		let bpr = Config::new().address_width(2).bytes_per_row(5).into_hexxdump();
		assert_eq!(
			concat!("00: 31 32 33 34 35  12345\n",
					"05: 36 37 38 39 30  67890\n",
					"0a: 61 62 63 64 65  abcde\n"),
			bpr.get_hexdump(b"1234567890abcde"),
		);
	}
}

#[test]
fn test_address_width() {
	{
		let a0 = Config::new().bytes_per_row(16).address_width(0).into_hexxdump();
		assert_eq!(
			concat!("00: 30 31 32 33 34 35 36 37 38 39 61 62 63 64 65 66  0123456789abcdef\n"),
			a0.get_hexdump(b"0123456789abcdef"),
		);
		assert_eq!(
			concat!("00: 30 31 32 33 34 35 36 37 38 39 61 62 63 64 65 66  0123456789abcdef\n",
			        "10: 67                                               g\n"),
			a0.get_hexdump(b"0123456789abcdefg"),
		);
	}

	{
		let a1 = Config::new().bytes_per_row(8).address_width(1).into_hexxdump();
		assert_eq!(
			concat!("0: 30 31 32 33 34 35 36 37  01234567\n",
			        "8: 38 39 61 62 63 64 65 66  89abcdef\n"),
			a1.get_hexdump(b"0123456789abcdef"),
		);
		assert_eq!(
			concat!("00: 30 31 32 33 34 35 36 37  01234567\n",
			        "08: 38 39 61 62 63 64 65 66  89abcdef\n",
			        "10: 67                       g\n"),
			a1.get_hexdump(b"0123456789abcdefg"),
		);
	}

	{
		let a2 = Config::new().bytes_per_row(8).address_width(2).into_hexxdump();
		{
			let dump = a2.get_hexdump((0..=255).collect::<Vec<_>>());
			let d256l = dump.lines().collect::<Vec<_>>();
			assert_eq!("00: 00 01 02 03 04 05 06 07  ........", d256l[0x00]);
			assert_eq!("10: 10 11 12 13 14 15 16 17  ........", d256l[0x02]);
			assert_eq!("f8: f8 f9 fa fb fc fd fe ff  ........", d256l[0x1f]);
		}
		{
			let dump = a2.get_hexdump([(0..=255).collect::<Vec<_>>(), vec![42]].concat());
			let d257l = dump.lines().collect::<Vec<_>>();
			assert_eq!("0000: 00 01 02 03 04 05 06 07  ........", d257l[0x00]);
			assert_eq!("0010: 10 11 12 13 14 15 16 17  ........", d257l[0x02]);
			assert_eq!("00f8: f8 f9 fa fb fc fd fe ff  ........", d257l[0x1f]);
			assert_eq!("0100: 2a                       *",        d257l[0x20]);
		}
	}

	{
		let a3 = Config::new().bytes_per_row(8).address_width(3).into_hexxdump();
		{
			let dump = a3.get_hexdump(
				std::iter::repeat((0..=255).collect::<Vec<_>>()).flatten().take(4096).collect::<Vec<_>>()
			);
			let d4096l = dump.lines().collect::<Vec<_>>();
			assert_eq!("000: 00 01 02 03 04 05 06 07  ........", d4096l[0x000]);
			assert_eq!("0f8: f8 f9 fa fb fc fd fe ff  ........", d4096l[0x01f]);
			assert_eq!("100: 00 01 02 03 04 05 06 07  ........", d4096l[0x020]);
			assert_eq!("ff8: f8 f9 fa fb fc fd fe ff  ........", d4096l[0x1ff]);
		}
		{
			let dump = a3.get_hexdump(
				std::iter::repeat((0..=255).collect::<Vec<_>>()).flatten().take(4097).collect::<Vec<_>>()
			);
			let d4097l = dump.lines().collect::<Vec<_>>();
			assert_eq!("0000: 00 01 02 03 04 05 06 07  ........", d4097l[0x000]);
			assert_eq!("00f8: f8 f9 fa fb fc fd fe ff  ........", d4097l[0x01f]);
			assert_eq!("0100: 00 01 02 03 04 05 06 07  ........", d4097l[0x020]);
			assert_eq!("0ff8: f8 f9 fa fb fc fd fe ff  ........", d4097l[0x1ff]);
			assert_eq!("1000: 00                       .",        d4097l[0x200]);
		}
	}
}

#[test]
fn test_show_address() {
	let hide = Config::new().address_width(2).bytes_per_row(8).show_address(false).into_hexxdump();
	let show = Config::new().address_width(2).bytes_per_row(8).show_address(true) .into_hexxdump();

	assert_eq!(    "48 65 6c 6c 6f           Hello\n", hide.get_hexdump(b"Hello"));
	assert_eq!("00: 48 65 6c 6c 6f           Hello\n", show.get_hexdump(b"Hello"));
}

#[test]
fn test_show_hex_values() {
	let hide = Config::new().address_width(2).bytes_per_row(8).show_hex_values(false).into_hexxdump();
	let show = Config::new().address_width(2).bytes_per_row(8).show_hex_values(true).into_hexxdump();

	assert_eq!("00: Hello\n",                           hide.get_hexdump(b"Hello"));
	assert_eq!("00: 48 65 6c 6c 6f           Hello\n" , show.get_hexdump(b"Hello"));
}

#[test]
fn test_show_characters() {
	let hide = Config::new().address_width(2).bytes_per_row(8).show_characters(false).into_hexxdump();
	let show = Config::new().address_width(2).bytes_per_row(8).show_characters(true) .into_hexxdump();

	assert_eq!("00: 48 65 6c 6c 6f           Hello\n", show.get_hexdump(b"Hello"));
	assert_eq!("00: 48 65 6c 6c 6f         \n",        hide.get_hexdump(b"Hello"));
}

#[test]
fn test_use_control_pictures() {
	let nouse = Config::new().address_width(2).bytes_per_row(4).use_control_pictures(false).into_hexxdump();
	let douse = Config::new().address_width(2).bytes_per_row(4).use_control_pictures(true) .into_hexxdump();

	assert_eq!("00: 09 0d 0a     ...\n", nouse.get_hexdump(b"\t\r\n"));
	assert_eq!("00: 09 0d 0a     ␉␍␊\n", douse.get_hexdump(b"\t\r\n"));
}

#[test]
fn test_substitute_character() {
	let pt = Config::new().address_width(2).bytes_per_row(5).substitute_character('.').into_hexxdump();
	let qm = Config::new().address_width(2).bytes_per_row(5).substitute_character('?').into_hexxdump();
	let cp = Config::new().address_width(2).bytes_per_row(5).substitute_character('␦').into_hexxdump();

	assert_eq!("00: 3e 80 fe ff     >...\n", pt.get_hexdump(b">\x80\xfe\xff"));
	assert_eq!("00: 3e 80 fe ff     >???\n", qm.get_hexdump(b">\x80\xfe\xff"));
	assert_eq!("00: 3e 80 fe ff     >␦␦␦\n", cp.get_hexdump(b">\x80\xfe\xff"));
}
