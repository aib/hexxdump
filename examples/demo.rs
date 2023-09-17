fn main() {
	let bytes = (0..=255).collect::<Vec<u8>>();

	println!("Default:");
	hexxdump::hexdump(&bytes);
	println!("");

	println!("Control pictures:");
	hexxdump::config::DEFAULT
		.use_full_control_pictures()
		.into_hexxdump()
		.hexdump(&bytes);
	println!("");

	println!("No address:");
	hexxdump::config::DEFAULT
		.hide_address()
		.into_hexxdump()
		.hexdump(&bytes);
	println!("");

	println!("Extended address:");
	hexxdump::config::DEFAULT
		.min_address_width(8)
		.into_hexxdump()
		.hexdump(&bytes);
	println!("");

	println!("Character dump:");
	hexxdump::config::DEFAULT
		.bytes_per_row(64)
		.hide_hex_values()
		.substitute_character('␦')
		.into_hexxdump()
		.hexdump(&bytes);
	println!("");

	println!("Hex value dump:");
	hexxdump::config::DEFAULT
		.hide_characters()
		.bytes_per_row(32)
		.into_hexxdump()
		.hexdump(&bytes);
	println!("");
}
