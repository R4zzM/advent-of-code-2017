use adventhelper::read_inputdata;

fn count_jumps<F>(mut intvec: Vec<i32>, change: F) -> u32
	where F: Fn(i32) -> i32 {
	let mut pos : i32 = 0;
	let mut count = 0;
	while pos < intvec.len() as i32 {
		let next_pos = intvec[pos as usize];
		intvec[pos as usize] += change(next_pos);
		pos += next_pos;
 		count += 1;
	}

	count
}

pub fn first_answer() -> u32 {
    let input : String = read_inputdata(file!());
    let int_vec : Vec<i32> = input.lines()
    	.map(|x| x.parse::<i32>().unwrap())
    	.collect();

	let int_change = |_offset| 1;
	count_jumps(int_vec, int_change)
}

pub fn second_answer() -> u32 {
    let input : String = read_inputdata(file!());
    let int_vec : Vec<i32> = input.lines()
    	.map(|x| x.parse::<i32>().unwrap())
    	.collect();

    let int_change = |offset| if offset >= 3 { -1 } else { 1 };
	count_jumps(int_vec, int_change)
}

#[cfg(test)]
mod tests {
	use advent5::*;

	const TEST_SEQ_1 : &str = "0\n3\n0\n1\n-3";

	#[test]
	fn test_count_jumps1() {
  		let int_vec : Vec<i32> = TEST_SEQ_1.lines()
  									  .map(|x| x.parse::<i32>().unwrap())
  									  .collect();

    	println!("int_vec: {:?}", int_vec);
    	let dint_doff = |offset| 1;
		let jumps = count_jumps(int_vec, dint_doff);
		assert_eq!(jumps, 5);
	}

	#[test]
	fn test_count_jumps2() {
  		let int_vec : Vec<i32> = TEST_SEQ_1.lines()
  									  .map(|x| x.parse::<i32>().unwrap())
  									  .collect();

		let dint_doff = |offset| if offset >= 3 { -1 } else { 1 };
		let jumps2 = count_jumps(int_vec, dint_doff);
		assert_eq!(jumps2, 10);
	}
}