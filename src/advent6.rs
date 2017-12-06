use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
struct Configuration {
	hash : u64,
	loop_size : u64,
}

impl Configuration {
	fn from(hash : u64, n: u64 ) -> Configuration {
		Configuration { hash : hash, loop_size : n }
	}
}

// Hash the bank configuration
fn make_hash(banks: &[u32]) -> u64 {
	let mut s : String = String::new();
	let mut hasher = DefaultHasher::new();
	for block_count in banks.iter() {
    	s.push_str((&block_count).to_string().as_str());
    }

	s.hash(&mut hasher);
    hasher.finish()
}

fn count_configurations(banks: &mut [u32]) -> (u64, u64) {
	let mut configurations : Vec<Configuration> = Vec::new();
	let initial_config = Configuration::from(make_hash(banks), 0);
	configurations.push(initial_config);

	let mut loop_counter : u64 = 0; // Answer to first
	let mut loop_size = 0; // Answer to second
	loop {
		loop_counter += 1;

		// Find bank with most blocks
		let mut bank_top = 0;
		let mut blocks_highest = 0;
		for (i, block_count) in banks.iter().enumerate() {
			if blocks_highest < *block_count {
				blocks_highest = *block_count;
				bank_top = i;
			}
		}

		// Set the block count for the bank with most blocks to 0
		banks[bank_top] = 0;

		// Reallocate the blocks
		let mut current = bank_top + 1;
		while blocks_highest > 0 {
			if current == banks.len() {
				current = 0;
			}

			banks[current] += 1;
			blocks_highest -= 1;
			current += 1;
		}

		let new_config = Configuration::from(make_hash(banks), loop_counter);

		// Check if the initial configuration matches the current one, if it does, we are done.
		match configurations.iter().find(|config| config.hash == new_config.hash) {
			None => (),
			Some(config) => { 
				loop_size = loop_counter - config.loop_size;
				break; 
			}
		};

		configurations.push(new_config);
	}

	(loop_counter, loop_size)
}

pub fn both() -> (u64, u64) {
	let mut banks : [u32; 16] = [11, 11, 13, 7, 0, 15, 5, 5, 4, 4, 1, 1, 7, 1, 15, 11];
	let (counter, size) = count_configurations(&mut banks);

	(counter, size)
}

#[cfg(test)]
mod  tests {
	use advent6::count_configurations;

	#[test]
	fn test_advent6_both() {
		let mut banks : [u32; 4] = [0, 2, 7, 0];
		let (counter, size) = count_configurations(&mut banks);
		assert_eq!(counter, 5);
		assert_eq!(size, 4);
	}
}