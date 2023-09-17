pub const DEFAULT: Config = Config {
	bytes_per_row: 16,
	min_address_width: 4,
	use_control_pictures: false,
	use_control_picture_for_space: false,
	substitute_character: '.',
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Config {
	pub bytes_per_row: usize,
	pub min_address_width: usize,
	pub use_control_pictures: bool,
	pub use_control_picture_for_space: bool,
	pub substitute_character: char,
}

impl Config {
	pub const fn new() -> Self { DEFAULT }

	pub const fn bytes_per_row(mut self, bytes_per_row: usize) -> Self {
		self.bytes_per_row = bytes_per_row;
		self
	}

	pub const fn min_address_width(mut self, min_address_width: usize) -> Self {
		self.min_address_width = min_address_width;
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
