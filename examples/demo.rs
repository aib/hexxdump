fn main() {
	let bytes = (0..=255).collect::<Vec<u8>>();

	println!("Default:");
	hexxdump::hexdump(&bytes);
	println!("");

	println!("Control pictures:");
	hexxdump::config::DEFAULT
		.use_control_pictures(true)
		.into_hexxdump()
		.hexdump(&bytes);
	println!("");

	println!("No address:");
	hexxdump::config::DEFAULT
		.show_address(false)
		.into_hexxdump()
		.hexdump(&bytes);
	println!("");

	println!("Extended address:");
	hexxdump::config::DEFAULT
		.address_width(8)
		.into_hexxdump()
		.hexdump(&bytes);
	println!("");

	println!("Character dump:");
	hexxdump::config::DEFAULT
		.bytes_per_row(64)
		.show_hex_values(false)
		.substitute_character('␦')
		.into_hexxdump()
		.hexdump(&bytes);
	println!("");

	println!("Hex value dump:");
	hexxdump::config::DEFAULT
		.show_characters(false)
		.bytes_per_row(32)
		.into_hexxdump()
		.hexdump(&bytes);
	println!("");
}
