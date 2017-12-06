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
fn make_hash(arg: &[u32]) -> u64 {
	let mut current = 0;
	let mut s : String = String::new();
	let mut hasher = DefaultHasher::new();
    while current < arg.len() {
    	s.push_str(arg[current].to_string().as_str());
    	current += 1;
    }

	s.hash(&mut hasher);
    hasher.finish()
}

fn count_configurations(banks: &mut [u32]) -> (u64, u64) {
	let mut configurations : Vec<Configuration> = Vec::new();
	let config = Configuration::from(make_hash(banks), 0);
	configurations.push(config);

	let mut loop_counter : u64 = 0; // Answer to first
	let mut loop_size = 0; // Answer to second
	let mut current = 0;
	loop {
		loop_counter += 1;

		// Find bank with most blocks
		current = 0;
		let mut bank_top = 0;
		let mut blocks_highest = 0;
		while current < banks.len() {
			if blocks_highest < banks[current] {
				blocks_highest = banks[current];
				bank_top = current;
			}
			current += 1;
		}

		// Set the block count for the bank with most blocks to 0
		banks[bank_top] = 0;

		// Reallocate the blocks
		current = bank_top + 1;
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
		match configurations.iter().find(|config| new_config.hash == config.hash) {
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
	let banks : &mut [u32; 16] = &mut [11, 11, 13, 7, 0, 15, 5, 5, 4, 4, 1, 1, 7, 1, 15, 11];
	let (counter, size) = count_configurations(banks);

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