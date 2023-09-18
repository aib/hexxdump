use crate::Hexxdump;

pub const DEFAULT: Config = Config {
	bytes_per_row: 16,
	address_width: 4,
	show_address: true,
	show_hex_values: true,
	show_characters: true,
	use_control_pictures: false,
	use_control_picture_for_space: false,
	substitute_character: '.',
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Config {
	pub bytes_per_row: usize,
	pub address_width: usize,
	pub show_address: bool,
	pub show_hex_values: bool,
	pub show_characters: bool,
	pub use_control_pictures: bool,
	pub use_control_picture_for_space: bool,
	pub substitute_character: char,
}

impl Config {
	pub const fn new() -> Self { DEFAULT }

	pub const fn into_hexxdump(self) -> Hexxdump { Hexxdump::with_config(self) }

	pub const fn bytes_per_row(mut self, bytes_per_row: usize) -> Self {
		self.bytes_per_row = bytes_per_row;
		self
	}

	pub const fn address_width(mut self, address_width: usize) -> Self {
		self.address_width = address_width;
		self
	}

	pub const fn show_address(mut self) -> Self {
		self.show_address = true;
		self
	}

	pub const fn hide_address(mut self) -> Self {
		self.show_address = false;
		self
	}

	pub const fn show_hex_values(mut self) -> Self {
		self.show_hex_values = true;
		self
	}

	pub const fn hide_hex_values(mut self) -> Self {
		self.show_hex_values = false;
		self
	}

	pub const fn show_characters(mut self) -> Self {
		self.show_characters = true;
		self
	}

	pub const fn hide_characters(mut self) -> Self {
		self.show_characters = false;
		self
	}

	pub const fn dont_use_control_pictures(mut self) -> Self {
		self.use_control_pictures = false;
		self.use_control_picture_for_space = false;
		self
	}

	pub const fn use_control_pictures(mut self) -> Self {
		self.use_control_pictures = true;
		self.use_control_picture_for_space = false;
		self
	}

	pub const fn use_full_control_pictures(mut self) -> Self {
		self.use_control_pictures = true;
		self.use_control_picture_for_space = true;
		self
	}

	pub const fn substitute_character(mut self, substitute_character: char) -> Self {
		self.substitute_character = substitute_character;
		self
	}
}

impl Default for Config {
	fn default() -> Self { Self::new() }
}
