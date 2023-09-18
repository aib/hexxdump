use crate::Hexxdump;

/// The default configuration for [`hexxdump::DEFAULT`](`crate::DEFAULT`)
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

/// An object to build and store a [`Hexxdump`]'s configuration
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Config {
	/// Number of bytes to show per row
	pub bytes_per_row: usize,

	/// The width (in hex digits) to use for the address column
	pub address_width: usize,

	/// Whether to show the address (left column)
	pub show_address: bool,

	/// Whether to show the hex values (middle column)
	pub show_hex_values: bool,

	/// Whether to show the characters (right column)
	pub show_characters: bool,

	/// Whether to present ASCII control bytes using characters from the Unicode "Control Pictures" block
	pub use_control_pictures: bool,

	/// Whether to present the ASCII space byte using the Unicode "Control Pictures" space character
	pub use_control_picture_for_space: bool,

	/// The character to present for non-ASCII and non-printable bytes
	///
	/// ASCII control bytes are substituted if `use_control_pictures` is not set
	pub substitute_character: char,
}

impl Config {
	/// Creates a new, default configuration object (equivalent to [`DEFAULT`])
	pub const fn new() -> Self { DEFAULT }

	/// Builds a [`Hexxdump`] object with this configuration
	///
	/// Equivalent to calling [`Hexxdump::with_config`]`(self)`
	pub const fn into_hexxdump(self) -> Hexxdump { Hexxdump::with_config(self) }

	/// Sets `bytes_per_row` to the given value
	pub const fn bytes_per_row(mut self, bytes_per_row: usize) -> Self {
		self.bytes_per_row = bytes_per_row;
		self
	}

	/// Sets `address_width` to the given value
	pub const fn address_width(mut self, address_width: usize) -> Self {
		self.address_width = address_width;
		self
	}

	/// Sets `show_address` to `true`
	pub const fn show_address(mut self) -> Self {
		self.show_address = true;
		self
	}

	/// Sets `show_address` to `false`
	pub const fn hide_address(mut self) -> Self {
		self.show_address = false;
		self
	}

	/// Sets `show_hex_values` to `true`
	pub const fn show_hex_values(mut self) -> Self {
		self.show_hex_values = true;
		self
	}

	/// Sets `show_hex_values` to `false`
	pub const fn hide_hex_values(mut self) -> Self {
		self.show_hex_values = false;
		self
	}

	/// Sets `show_characters` to `true`
	pub const fn show_characters(mut self) -> Self {
		self.show_characters = true;
		self
	}

	/// Sets `show_characters` to `false`
	pub const fn hide_characters(mut self) -> Self {
		self.show_characters = false;
		self
	}

	/// Sets `use_control_pictures` and `use_control_picture_for_space` to `false`
	pub const fn dont_use_control_pictures(mut self) -> Self {
		self.use_control_pictures = false;
		self.use_control_picture_for_space = false;
		self
	}

	/// Sets `use_control_pictures` to `true`, `use_control_picture_for_space` to `false`
	pub const fn use_control_pictures(mut self) -> Self {
		self.use_control_pictures = true;
		self.use_control_picture_for_space = false;
		self
	}

	/// Sets `use_control_pictures` and `use_control_picture_for_space` to `true`
	pub const fn use_full_control_pictures(mut self) -> Self {
		self.use_control_pictures = true;
		self.use_control_picture_for_space = true;
		self
	}

	/// Sets `substitute_character` to the given value
	pub const fn substitute_character(mut self, substitute_character: char) -> Self {
		self.substitute_character = substitute_character;
		self
	}
}

impl Default for Config {
	fn default() -> Self { Self::new() }
}
