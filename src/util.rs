pub fn min_hex_digits_for(num: usize) -> usize {
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
