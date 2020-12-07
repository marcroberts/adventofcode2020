use std::env;
use std::io;
use std::io::BufRead;
use std::fs::File;

fn main() {
  let args: Vec<String> = env::args().collect();
  let filename = &args[1];

	let mut values: Vec<u32> = Vec::new();

	let file = File::open(filename).unwrap();
	let reader = io::BufReader::new(file);

	// load the values
	for line in reader.lines() {
		let number: u32 = line.unwrap().trim().parse().unwrap();
		values.push(number);
	}

	// loop on all number, => a
	'outer: for (a_index, a) in values.iter().enumerate() {


		for (b_index, b) in values.iter().enumerate() {
			if b_index < a_index {
				continue;
			}

			for (c_index, c) in values.iter().enumerate() {
				if c_index < b_index {
					continue;
				}

				if a + b + c== 2020 {
					println!("{} + {} + {} = 2020. Sum is {}", a, b, c, a*b*c);
					break 'outer;
				}
			}
		}


	}

}
