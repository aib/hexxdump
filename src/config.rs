use crate::Hexxdump;

/// The default configuration for [`hexxdump::DEFAULT`](`crate::DEFAULT`)
pub const DEFAULT: Config = Config {
	bytes_per_row: 16,
	address_width: 4,
	show_address: true,
	show_hex_values: true,
	show_characters: true,
	use_control_pictures: false,
	substitute_character: '.',
};

/// An object to build and store a [`Hexxdump`]'s configuration
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Config {
	pub(super) bytes_per_row: usize,
	pub(super) address_width: usize,
	pub(super) show_address: bool,
	pub(super) show_hex_values: bool,
	pub(super) show_characters: bool,
	pub(super) use_control_pictures: bool,
	pub(super) substitute_character: char,
}

impl Config {
	/// Creates a new, default configuration object (equivalent to [`DEFAULT`])
	pub const fn new() -> Self { DEFAULT }

	/// Builds a [`Hexxdump`] object with this configuration
	///
	/// Equivalent to calling [`Hexxdump::with_config`]`(self)`
	pub const fn into_hexxdump(self) -> Hexxdump { Hexxdump::with_config(self) }

	/// Sets the number of bytes to output per row
	pub const fn bytes_per_row(mut self, bytes_per_row: usize) -> Self {
		self.bytes_per_row = bytes_per_row;
		self
	}

	/// Sets the width (in hex digits) to use for the address
	pub const fn address_width(mut self, address_width: usize) -> Self {
		self.address_width = address_width;
		self
	}

	/// Sets the address (left) column to be included in dumps
	pub const fn show_address(mut self) -> Self {
		self.show_address = true;
		self
	}

	/// Sets the address (left) column to be excluded from dumps
	pub const fn hide_address(mut self) -> Self {
		self.show_address = false;
		self
	}

	/// Sets the hex values (middle) column to be included in dumps
	pub const fn show_hex_values(mut self) -> Self {
		self.show_hex_values = true;
		self
	}

	/// Sets the hex values (middle) column to be excluded from dumps
	pub const fn hide_hex_values(mut self) -> Self {
		self.show_hex_values = false;
		self
	}

	/// Sets the characters (right) column to be included in dumps
	pub const fn show_characters(mut self) -> Self {
		self.show_characters = true;
		self
	}

	/// Sets the characters (right) column to be excluded from dumps
	pub const fn hide_characters(mut self) -> Self {
		self.show_characters = false;
		self
	}

	/// Sets ASCII control characters (0 ~ 31) to be presented using the [`substitute character`](Self::substitute_character)
	pub const fn dont_use_control_pictures(mut self) -> Self {
		self.use_control_pictures = false;
		self
	}

	/// Sets ASCII control characters (0 ~ 31) to be printed using Unicode characters from the "Control Pictures" block
	pub const fn use_control_pictures(mut self) -> Self {
		self.use_control_pictures = true;
		self
	}

	/// Sets the substitute character, the character printed to represent non-ASCII-printable bytes
	///
	/// The substitute character will be printed for non-ASCII (80 ~ 255) bytes.
	/// It will also be printed for ASCII control characters (0 ~ 31) if [`dont_use_control_pictures`](Self::dont_use_control_pictures) is set.
	///
	/// Defaults to `'.'`
	pub const fn substitute_character(mut self, substitute_character: char) -> Self {
		self.substitute_character = substitute_character;
		self
	}
}

impl Default for Config {
	fn default() -> Self { Self::new() }
}
