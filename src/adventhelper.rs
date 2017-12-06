use std::fs::File;
use std::io::Read;

/* Deprecated */
pub fn inputdata_as_string(abspath: String) -> String {
	let mut file_contents = String::new();
	let mut spreadsheet_file = File::open(&abspath[..]).unwrap();

	spreadsheet_file.read_to_string(&mut file_contents).unwrap();

    file_contents
}

// Assumes that the inputdata is available in the <cargo root>/data/<modulename>.txt
// Improvement: Errorhandling
pub fn read_inputdata(rust_path: &str) -> String {
	let absdir = "/home/rasmus/repos/advent-of-code-2017/data";
	let filename = rust_path.split("/").last().unwrap();
	let abspath = format!("{}/{}.txt", absdir, filename.split(".").nth(0).unwrap());

	let mut file = File::open(abspath).unwrap();
	let mut inputdata = String::new();

	file.read_to_string(&mut inputdata).unwrap();

	inputdata
}

#[cfg(test)]
mod tests {
	use adventhelper::*;

	#[test]
	fn test_read_inputdata() {
		let data : String = read_inputdata(file!());
		assert_eq!(data, String::from("hello!"));
	}
}